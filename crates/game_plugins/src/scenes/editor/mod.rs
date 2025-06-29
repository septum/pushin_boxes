mod ui;

use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_kira_audio::{AudioChannel, AudioControl};
use game_ui::{Colors, DynamicTextData, OverlayMarker};

use crate::{
    input::{ActionInput, ActionInputEvent, DirectionInput, DirectionInputEvent},
    level::{
        Brush, BrushEntity, BrushSprite, LevelInsertionEvent, LevelKind, LevelResource,
        LevelValidity, MapEntity, MapPositionComponent, MapPositionExtension, TOTAL_CUSTOM_LEVELS,
    },
    resources::prelude::*,
    save_file::SaveFile,
};

const VALID_ID: usize = 0;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelValidity::default())
            .add_systems(
                OnEnter(GameState::Editor),
                (
                    check_total_custom_levels,
                    self::ui::spawn,
                    Brush::insert,
                    setup_level,
                ),
            )
            .add_systems(
                Update,
                (
                    handle_action_input,
                    handle_direction_input,
                    blink_tile,
                    apply_brush_to_level,
                    update_character_position,
                    update_map,
                    update_brush_sprite,
                    check_validity,
                    play_action_sfx.run_if(on_event::<ActionInputEvent>),
                    play_direction_sfx.run_if(on_event::<DirectionInputEvent>),
                )
                    .run_if(in_state(GameState::Editor)),
            )
            .add_systems(
                OnExit(GameState::Editor),
                (
                    cleanup::<OverlayMarker>,
                    cleanup::<CharacterMarker>,
                    cleanup::<MapPositionComponent>,
                    cleanup::<BrushSprite>,
                ),
            );
    }
}

fn check_total_custom_levels(
    save_file: Res<SaveFile>,
    mut scene_transition_event_writer: EventWriter<SceneTransitionEvent>,
) {
    if save_file.number_custom_levels() == TOTAL_CUSTOM_LEVELS {
        scene_transition_event_writer.write(SceneTransitionEvent::limit());
    }
}

fn setup_level(
    mut commands: Commands,
    images: Res<Images>,
    mut level_validity: ResMut<LevelValidity>,
) {
    let mut level = LevelResource::editable();
    level.spawn(&mut commands, &images);
    commands.insert_resource(level);
    level_validity.reset();
}

fn handle_direction_input(
    mut brush: ResMut<Brush>,
    mut direction_event_reader: EventReader<DirectionInputEvent>,
) {
    for direction_event in direction_event_reader.read() {
        match direction_event.value {
            DirectionInput::Up => brush.position.decrement_y(),
            DirectionInput::Down => brush.position.increment_y(),
            DirectionInput::Left => brush.position.decrement_x(),
            DirectionInput::Right => brush.position.increment_x(),
        }
    }
}

fn handle_action_input(
    level: Res<LevelResource>,
    level_validity: Res<LevelValidity>,
    mut brush: ResMut<Brush>,
    mut game_state_event_writer: EventWriter<SceneTransitionEvent>,
    mut level_insertion_event_writer: EventWriter<LevelInsertionEvent>,
    mut action_event_reader: EventReader<ActionInputEvent>,
) {
    for action_event in action_event_reader.read() {
        match action_event.value {
            ActionInput::Toggle => brush.cycle(),
            ActionInput::Select => {
                if level_validity.zones > 0 && level_validity.zones == level_validity.boxes {
                    level_insertion_event_writer.write(LevelInsertionEvent::new(
                        LevelKind::Playtest(*level.state()),
                    ));
                }
            }
            ActionInput::Exit => {
                game_state_event_writer.write(SceneTransitionEvent::title());
            }
            _ => (),
        }
    }
}

fn blink_tile(
    time: Res<Time>,
    mut brush: ResMut<Brush>,
    mut entity_query: Query<(&mut Sprite, &MapPositionComponent), With<MapPositionComponent>>,
) {
    brush.blink_timer.tick(time.delta());

    if brush.blink_timer.just_finished() {
        for (mut sprite, position) in &mut entity_query {
            if position.x() == brush.position.x() && position.y() == brush.position.y() {
                if sprite.color == Colors::PRIMARY {
                    sprite.color = Colors::LIGHT;
                } else {
                    sprite.color = Colors::PRIMARY;
                }
            } else {
                sprite.color = Colors::LIGHT;
            }
        }
    }
}

fn apply_brush_to_level(
    brush: Res<Brush>,
    mut level: ResMut<LevelResource>,
    mut level_validity: ResMut<LevelValidity>,
) {
    if matches!(brush.entity, BrushEntity::Character) {
        if matches!(
            level.get_entity(&brush.position),
            MapEntity::F | MapEntity::Z
        ) {
            level.move_character(brush.position);
        }
    } else if level.character_position() == brush.position {
        level.set_entity(
            &brush.position,
            match brush.entity {
                BrushEntity::Floor => MapEntity::F,
                BrushEntity::Zone => MapEntity::Z,
                _ => return,
            },
        );

        if matches!(brush.entity, BrushEntity::Floor)
            && matches!(level.get_entity(&brush.position), MapEntity::Z)
        {
            level_validity.zones -= 1;
            level.decrement_remaining_zones();
        } else if matches!(brush.entity, BrushEntity::Zone)
            && matches!(level.get_entity(&brush.position), MapEntity::F)
        {
            level_validity.zones += 1;
            level.increment_remaining_zones();
        }
    } else {
        if matches!(
            brush.entity,
            BrushEntity::Floor | BrushEntity::Void | BrushEntity::BoxInFloor
        ) && matches!(level.get_entity(&brush.position), MapEntity::Z)
        {
            level_validity.zones -= 1;
            level.decrement_remaining_zones();
        } else if matches!(brush.entity, BrushEntity::Zone)
            && matches!(
                level.get_entity(&brush.position),
                MapEntity::F | MapEntity::V | MapEntity::B
            )
        {
            level_validity.zones += 1;
            level.increment_remaining_zones();
        }

        if !matches!(brush.entity, BrushEntity::BoxInFloor)
            && matches!(level.get_entity(&brush.position), MapEntity::B)
        {
            level_validity.boxes -= 1;
        } else if matches!(brush.entity, BrushEntity::BoxInFloor)
            && !matches!(level.get_entity(&brush.position), MapEntity::B)
        {
            level_validity.boxes += 1;
        }

        level.set_entity(
            &brush.position,
            match brush.entity {
                BrushEntity::Floor => MapEntity::F,
                BrushEntity::Void => MapEntity::V,
                BrushEntity::Zone => MapEntity::Z,
                BrushEntity::BoxInFloor => MapEntity::B,
                BrushEntity::BoxInZone => MapEntity::P,
                BrushEntity::Character => return,
            },
        );
    }
}

fn update_character_position(
    level: Res<LevelResource>,
    mut query: Query<&mut Transform, With<CharacterMarker>>,
) {
    let mut transform = query.single_mut().unwrap();
    level
        .character_position()
        .update_translation(&mut transform.translation);

    // TODO: There should be another way to do this proper
    transform.translation.z += 1.;
}

fn update_brush_sprite(
    brush: Res<Brush>,
    images: Res<Images>,
    mut query: Query<(&mut Sprite, &mut Transform), With<BrushSprite>>,
) {
    let (mut sprite, mut transform) = query.single_mut().unwrap();
    brush
        .position
        .update_translation(&mut transform.translation);
    transform.translation.y += 20.0;
    transform.translation.z = 20.0;

    sprite.image = match brush.entity {
        BrushEntity::Floor => images.brush_floor.clone(),
        BrushEntity::Void => images.brush_void.clone(),
        BrushEntity::Zone => images.brush_zone.clone(),
        BrushEntity::BoxInFloor => images.brush_box.clone(),
        BrushEntity::BoxInZone => images.brush_placed_box.clone(),
        BrushEntity::Character => images.brush_character.clone(),
    };
}

fn check_validity(
    level_validity: Res<LevelValidity>,
    mut writer: TextUiWriter,
    mut texts: Query<(Entity, &DynamicTextData)>,
) {
    let (entity, data) = texts.single_mut().unwrap();
    *writer.text(entity, 1) = match data.id {
        VALID_ID => {
            if level_validity.zones > 0 && level_validity.zones == level_validity.boxes {
                "YES".to_string()
            } else {
                "NO".to_string()
            }
        }
        _ => unreachable!("The text id does not exists"),
    };
}

fn update_map(
    level: Res<LevelResource>,
    images: Res<Images>,
    mut query: Query<(&mut Sprite, &mut Transform, &MapPositionComponent)>,
) {
    for (mut sprite, mut transform, position) in &mut query {
        let map_entity = level.get_entity(position);
        sprite.image = match map_entity {
            MapEntity::V => images.entity_void.clone(),
            MapEntity::F => images.entity_floor.clone(),
            MapEntity::Z => images.entity_zone.clone(),
            MapEntity::B => images.entity_box.clone(),
            MapEntity::P => images.entity_placed_box.clone(),
        };
        position.update_translation(&mut transform.translation);
    }
}

fn play_action_sfx(
    mut action_event_reader: EventReader<ActionInputEvent>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    for action_event in action_event_reader.read() {
        match action_event.value {
            ActionInput::Exit => {
                sfx.play(sounds.sfx_push_box.clone());
            }
            ActionInput::Toggle => {
                sfx.play(sounds.sfx_toggle_volume.clone());
            }
            ActionInput::Select => {
                sfx.play(sounds.sfx_set_zone.clone());
            }
            _ => (),
        }
    }
}

pub fn play_direction_sfx(
    mut direction_event_reader: EventReader<DirectionInputEvent>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    for _ in direction_event_reader.read() {
        sfx.play(sounds.sfx_move_character.clone());
    }
}
