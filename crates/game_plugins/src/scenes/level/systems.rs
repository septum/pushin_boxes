use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use bevy_ui_bits::DynamicTextData;
use game_core::{
    level::{LevelKind, LevelUpdate},
    map::MapEntity,
};

use crate::{
    assets::prelude::*,
    character::Character,
    input::InputEvent,
    level::{EntityComponent, LevelResource, apply_position_to_translation},
    state::{GameStateTransitionEvent, SelectionKind},
};

use super::ui::{MOVES_COUNTER_ID, STOPWATCH_COUNTER_ID, UNDOS_COUNTER_ID};

pub fn spawn_level(mut commands: Commands, mut level: ResMut<LevelResource>, images: Res<Images>) {
    level.spawn(&mut commands, &images);
}

pub fn handle_input(
    mut game_state_event_writer: EventWriter<GameStateTransitionEvent>,
    mut input_event_reader: EventReader<InputEvent>,
    mut level: ResMut<LevelResource>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    if level.no_remaining_zones() {
        return;
    }

    for input_event in input_event_reader.read() {
        if let Some(update) = level.update(input_event) {
            match update {
                LevelUpdate::PushBox => {
                    sfx.play(sounds.sfx_move_character.clone());
                    sfx.play(sounds.sfx_push_box.clone());
                }
                LevelUpdate::PlaceBox => {
                    sfx.play(sounds.sfx_move_character.clone());
                    sfx.play(sounds.sfx_push_box.clone());
                    sfx.play(sounds.sfx_set_zone.clone());
                }
                LevelUpdate::MoveCharacter => {
                    sfx.play(sounds.sfx_move_character.clone());
                }
                LevelUpdate::UndoMove => {
                    sfx.play(sounds.sfx_undo_move.clone());
                }
                LevelUpdate::Reload => {
                    sfx.play(sounds.sfx_reload_level.clone());
                }
                LevelUpdate::Exit => {
                    sfx.play(sounds.sfx_push_box.clone());
                    game_state_event_writer.write(GameStateTransitionEvent::selection(
                        if level.is_stock() {
                            SelectionKind::Stock
                        } else {
                            SelectionKind::Custom
                        },
                    ));
                }
            }
        }
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

pub fn update_counters(
    level: Res<LevelResource>,
    mut writer: TextUiWriter,
    texts: Query<(Entity, &DynamicTextData)>,
) {
    for (entity, data) in texts {
        *writer.text(entity, 1) = match data.id {
            MOVES_COUNTER_ID => level.moves_string(),
            UNDOS_COUNTER_ID => level.undos_string(),
            STOPWATCH_COUNTER_ID => level.time_string(),
            _ => unreachable!("The counter id does not exists"),
        };
    }
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

pub fn update_level_state(time: Res<Time>, mut level: ResMut<LevelResource>) {
    level.tick(time.delta());
}

pub fn check_lever_timer_just_finished(
    level: Res<LevelResource>,
    mut scene_transition_event_writer: EventWriter<GameStateTransitionEvent>,
) {
    if level.finished() {
        match level.kind() {
            LevelKind::Stock(_) | LevelKind::Custom(_) => {
                scene_transition_event_writer.write(GameStateTransitionEvent::win());
            }
            LevelKind::Editable(_) => {
                scene_transition_event_writer.write(GameStateTransitionEvent::passed());
            }
        }
    }
}
