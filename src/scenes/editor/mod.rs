mod ui;

use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{
    resources::prelude::*,
    ui::{DynamicTextData, OverlayMarker},
};

const VALID_ID: usize = 0;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelValidity::default())
            .add_systems(
                (
                    check_total_custom_levels,
                    self::ui::spawn,
                    Brush::insert,
                    setup_level,
                )
                    .in_schedule(OnEnter(GameState::Editor)),
            )
            .add_systems(
                (
                    handle_action_input,
                    handle_direction_input,
                    blink_tile,
                    apply_brush_to_level,
                    update_character_position,
                    update_map,
                    update_brush_sprite,
                    check_validity,
                    play_action_sfx.run_if(on_event::<ActionInputEvent>()),
                    play_direction_sfx.run_if(on_event::<DirectionInputEvent>()),
                )
                    .in_set(OnUpdate(GameState::Editor)),
            )
            .add_systems(
                (
                    cleanup::<OverlayMarker>,
                    cleanup::<CharacterMarker>,
                    cleanup::<MapPosition>,
                    cleanup::<BrushSprite>,
                )
                    .in_schedule(OnExit(GameState::Editor)),
            );
    }
}

fn check_total_custom_levels(
    save_file: Res<SaveFile>,
    mut scene_transition_event_writer: EventWriter<SceneTransitionEvent>,
) {
    if save_file.number_custom_levels() == TOTAL_CUSTOM_LEVELS {
        scene_transition_event_writer.send(SceneTransitionEvent::limit());
    }
}

fn setup_level(
    mut commands: Commands,
    images: Res<Images>,
    mut level_validity: ResMut<LevelValidity>,
) {
    let mut level = Level::editable();
    level.spawn(&mut commands, &images);
    commands.insert_resource(level);
    level_validity.reset();
}

fn handle_direction_input(
    mut brush: ResMut<Brush>,
    mut direction_event_reader: EventReader<DirectionInputEvent>,
) {
    for direction_event in direction_event_reader.iter() {
        match direction_event.value {
            DirectionInput::Up => brush.position.decrement_y(),
            DirectionInput::Down => brush.position.increment_y(),
            DirectionInput::Left => brush.position.decrement_x(),
            DirectionInput::Right => brush.position.increment_x(),
        }
    }
}

fn handle_action_input(
    level: Res<Level>,
    level_validity: Res<LevelValidity>,
    mut brush: ResMut<Brush>,
    mut game_state_event_writer: EventWriter<SceneTransitionEvent>,
    mut level_insertion_event_writer: EventWriter<LevelInsertionEvent>,
    mut action_event_reader: EventReader<ActionInputEvent>,
) {
    for action_event in action_event_reader.iter() {
        match action_event.value {
            ActionInput::Toggle => brush.cycle(),
            ActionInput::Select => {
                if level_validity.zones > 0 && level_validity.zones == level_validity.boxes {
                    level_insertion_event_writer.send(LevelInsertionEvent::new(
                        LevelKind::Playtest(level.clone_state()),
                    ));
                }
            }
            ActionInput::Exit => game_state_event_writer.send(SceneTransitionEvent::title()),
            _ => (),
        }
    }
}

fn blink_tile(
    time: Res<Time>,
    mut brush: ResMut<Brush>,
    mut entity_query: Query<(&mut Sprite, &MapPosition), With<MapPosition>>,
) {
    brush.blink_timer.tick(time.delta());

    if brush.blink_timer.just_finished() {
        for (mut sprite, position) in entity_query.iter_mut() {
            if position == &brush.position {
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
    mut level: ResMut<Level>,
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
    level: Res<Level>,
    mut query: Query<&mut Transform, With<CharacterMarker>>,
) {
    let mut transform = query.single_mut();
    level
        .character_position()
        .update_translation(&mut transform.translation);
}

fn update_brush_sprite(
    brush: Res<Brush>,
    images: Res<Images>,
    mut query: Query<(&mut Handle<Image>, &mut Transform), With<BrushSprite>>,
) {
    let (mut image, mut transform) = query.single_mut();
    brush
        .position
        .update_translation(&mut transform.translation);
    transform.translation.y += 20.0;
    transform.translation.z = 20.0;

    *image = match brush.entity {
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
    mut texts: Query<(&mut Text, &DynamicTextData)>,
) {
    let (mut text, data) = texts.single_mut();
    text.sections[1].value = match data.id {
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
    level: Res<Level>,
    images: Res<Images>,
    mut query: Query<(&mut Handle<Image>, &mut Transform, &MapPosition)>,
) {
    for (mut image, mut transform, position) in query.iter_mut() {
        let map_entity = level.get_entity(position);
        *image = map_entity.to_image(&images);
        position.update_translation(&mut transform.translation);

        // NOTE: The floor entity is drawn over the character
        // and somehow it's related to the alphabetical order filenames
        if matches!(map_entity, MapEntity::F) {
            transform.translation.z -= 1.0;
        }
    }
}

fn play_action_sfx(
    mut action_event_reader: EventReader<ActionInputEvent>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    for action_event in action_event_reader.iter() {
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
    for _ in direction_event_reader.iter() {
        sfx.play(sounds.sfx_move_character.clone());
    }
}
