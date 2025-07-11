use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use game_core::{
    input::{Action, Input},
    level::LevelKind,
    map::MapEntity,
};
use game_ui::{Colors, DynamicTextData};

use crate::{
    assets::prelude::*,
    character::Character,
    input::InputEvent,
    level::{
        Brush, BrushEntity, BrushSprite, EntityComponent, LevelInsertionEvent, LevelResource,
        LevelValidity, TOTAL_CUSTOM_LEVELS, apply_position_to_translation,
    },
    save_file::SaveFile,
    state::GameStateTransitionEvent,
};

use super::ui::VALID_ID;

pub fn check_total_custom_levels(
    save_file: Res<SaveFile>,
    mut scene_transition_event_writer: EventWriter<GameStateTransitionEvent>,
) {
    if save_file.number_custom_levels() == TOTAL_CUSTOM_LEVELS {
        scene_transition_event_writer.write(GameStateTransitionEvent::limit());
    }
}

pub fn setup_level(
    mut commands: Commands,
    images: Res<Images>,
    mut level_validity: ResMut<LevelValidity>,
) {
    let mut level = LevelResource::default();
    level.spawn(&mut commands, &images);
    commands.insert_resource(level);
    level_validity.reset();
}

pub fn handle_input(
    level: Res<LevelResource>,
    level_validity: Res<LevelValidity>,
    mut brush: ResMut<Brush>,
    mut game_state_event_writer: EventWriter<GameStateTransitionEvent>,
    mut level_insertion_event_writer: EventWriter<LevelInsertionEvent>,
    mut input_event_reader: EventReader<InputEvent>,
) {
    for input_event in input_event_reader.read() {
        match **input_event {
            Input::Direction(direction) => {
                brush.position.update(&direction);
            }
            Input::Action(Action::Toggle) => brush.cycle(),
            Input::Action(Action::Select) => {
                if level_validity.zones > 0 && level_validity.zones == level_validity.boxes {
                    level_insertion_event_writer.write(LevelInsertionEvent::new(
                        LevelKind::Editable(*level.state()),
                    ));
                }
            }
            Input::Action(Action::Exit) => {
                game_state_event_writer.write(GameStateTransitionEvent::title());
            }
            Input::Action(_) => (),
        }
    }
}

pub fn blink_tile(
    time: Res<Time>,
    mut brush: ResMut<Brush>,
    mut entity_query: Query<(&mut Sprite, &EntityComponent), With<EntityComponent>>,
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

pub fn apply_brush_to_level(
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

pub fn update_character_position(
    level: Res<LevelResource>,
    mut query: Query<&mut Transform, With<Character>>,
) {
    let mut transform = query.single_mut().unwrap();
    apply_position_to_translation(&level.character_position(), &mut transform.translation);

    // TODO: There should be another way to do this proper
    transform.translation.z += 1.;
}

pub fn update_brush_sprite(
    brush: Res<Brush>,
    images: Res<Images>,
    mut query: Query<(&mut Sprite, &mut Transform), With<BrushSprite>>,
) {
    let (mut sprite, mut transform) = query.single_mut().unwrap();
    apply_position_to_translation(&brush.position, &mut transform.translation);
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

pub fn check_validity(
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

pub fn update_map(
    level: Res<LevelResource>,
    images: Res<Images>,
    mut query: Query<(&mut Sprite, &mut Transform, &EntityComponent)>,
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
        apply_position_to_translation(position, &mut transform.translation);
    }
}

pub fn play_sfx(
    mut input_event_reader: EventReader<InputEvent>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    for input_event in input_event_reader.read() {
        match **input_event {
            Input::Direction(_) => {
                sfx.play(sounds.sfx_move_character.clone());
            }
            Input::Action(Action::Exit) => {
                sfx.play(sounds.sfx_push_box.clone());
            }
            Input::Action(Action::Toggle) => {
                sfx.play(sounds.sfx_toggle_volume.clone());
            }
            Input::Action(Action::Select) => {
                sfx.play(sounds.sfx_set_zone.clone());
            }
            Input::Action(_) => (),
        }
    }
}
