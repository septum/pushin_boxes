use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};
use game_ui::DynamicTextData;

use crate::{
    assets::prelude::*,
    input::{ActionInput, ActionInputEvent, DirectionInputEvent},
    level::{
        LevelDoneTimer, LevelHandles, LevelKind, LevelResource, LevelState, MapEntity,
        MapPositionComponent, MapPositionExtension,
    },
    state::{GameStateTransitionEvent, SelectionKind},
};

use super::ui::{MOVES_COUNTER_ID, STOPWATCH_COUNTER_ID, UNDOS_COUNTER_ID};

pub fn spawn_level(
    mut commands: Commands,
    mut level: ResMut<LevelResource>,
    mut done_timer: ResMut<LevelDoneTimer>,
    images: Res<Images>,
) {
    done_timer.reset();
    level.spawn(&mut commands, &images);
}

// TODO: Prevent LevelState leakage
pub fn handle_action_input(
    mut game_state_event_writer: EventWriter<GameStateTransitionEvent>,
    mut action_event_reader: EventReader<ActionInputEvent>,
    mut level: ResMut<LevelResource>,
    level_handles: Res<LevelHandles>,
    level_states_assets: Res<Assets<LevelState>>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    if level.no_remaining_zones() {
        return;
    }

    for action_event in action_event_reader.read() {
        match &action_event.value {
            ActionInput::Undo => {
                if level.undo() {
                    sfx.play(sounds.sfx_undo_move.clone());
                }
            }
            ActionInput::Reload => {
                if level.reload(&level_handles, &level_states_assets) {
                    sfx.play(sounds.sfx_reload_level.clone());
                }
            }
            ActionInput::Exit => {
                sfx.play(sounds.sfx_push_box.clone());
                game_state_event_writer.write(GameStateTransitionEvent::selection(
                    if level.is_stock() {
                        SelectionKind::Stock
                    } else {
                        SelectionKind::Custom
                    },
                ));
            }
            _ => (),
        }
    }
}

pub fn handle_direction_input(
    mut direction_event_reader: EventReader<DirectionInputEvent>,
    mut level: ResMut<LevelResource>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    if level.no_remaining_zones() {
        return;
    }

    for direction_event in direction_event_reader.read() {
        let direction = &direction_event.value;
        level.set_character_facing_direction_with(direction);

        let mut next_position = level.character_position();
        next_position.update_position(direction);

        let next_entity = level.get_entity(&next_position);
        match next_entity {
            MapEntity::B | MapEntity::P => {
                let in_zone = matches!(next_entity, MapEntity::P);
                let updated_next_entity = if in_zone { MapEntity::Z } else { MapEntity::F };

                let mut adjacent_position = next_position;
                adjacent_position.update_position(direction);

                let adjacent_entity = level.get_entity(&adjacent_position);
                match adjacent_entity {
                    MapEntity::F => {
                        sfx.play(sounds.sfx_move_character.clone());
                        sfx.play(sounds.sfx_push_box.clone());

                        level.save_snapshot();
                        level.set_entity(&next_position, updated_next_entity);
                        level.set_entity(&adjacent_position, MapEntity::B);
                        level.move_character(next_position);
                        level.increment_moves();

                        if in_zone {
                            level.increment_remaining_zones();
                        }
                    }
                    MapEntity::Z => {
                        sfx.play(sounds.sfx_move_character.clone());
                        sfx.play(sounds.sfx_push_box.clone());
                        sfx.play(sounds.sfx_set_zone.clone());

                        level.save_snapshot();
                        level.set_entity(&next_position, updated_next_entity);
                        level.set_entity(&adjacent_position, MapEntity::P);
                        level.move_character(next_position);
                        level.increment_moves();

                        if !in_zone {
                            level.decrement_remaining_zones();
                        }
                    }
                    _ => (),
                }
            }
            MapEntity::V => {}
            _ => {
                level.save_snapshot();
                level.move_character(next_position);
                level.increment_moves();
                sfx.play(sounds.sfx_move_character.clone());
            }
        }
    }
}

pub fn update_character_position(
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

pub fn update_character_sprite(
    time: Res<Time>,
    level: Res<LevelResource>,
    mut character_animation: ResMut<CharacterAnimation>,
    mut query: Query<&mut Sprite, With<CharacterMarker>>,
) {
    let mut sprite = query.single_mut().unwrap();
    let level_character_facing_direction = level.get_character_facing_direction();

    character_animation.tick(time.delta());

    if level_character_facing_direction == 0 {
        if character_animation.secondary_timer_just_finished() {
            character_animation.set_blink_row();
            character_animation.reset_primary_timer();
        }

        if character_animation.tertiary_timer_just_finished() {
            character_animation.set_sleep_row();
            character_animation.reset_primary_timer();
        }
    } else {
        character_animation.reset_secondary_timer();
        character_animation.reset_tertiary_timer();
    }

    if !character_animation.row_is(level_character_facing_direction)
        && !character_animation.secondary_timer_finished()
        && !character_animation.tertiary_timer_finished()
    {
        character_animation.reset_primary_timer();
        character_animation.reset_index();
        character_animation.set_row(level_character_facing_direction);
    }

    if character_animation.primary_timer_just_finished() {
        character_animation.next_index();
    }

    sprite.texture_atlas.as_mut().unwrap().index = character_animation.sprite_index();
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
            STOPWATCH_COUNTER_ID => level.stopwatch_string(),
            _ => unreachable!("The counter id does not exists"),
        };
    }
}

pub fn update_map(
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

pub fn update_level_state(
    time: Res<Time>,
    mut level: ResMut<LevelResource>,
    mut done_timer: ResMut<LevelDoneTimer>,
) {
    let delta = time.delta();
    if level.no_remaining_zones() {
        done_timer.tick(delta);
    } else {
        level.tick_stopwatch(delta);
    }
}

pub fn check_lever_timer_just_finished(
    mut scene_transition_event_writer: EventWriter<GameStateTransitionEvent>,
    done_timer: Res<LevelDoneTimer>,
    level: Res<LevelResource>,
) {
    if done_timer.just_finished() {
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
