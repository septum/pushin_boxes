use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::resources::{
    input::{Action, Direction},
    prelude::*,
};

use super::{level, state::GameState};

pub fn process(
    input: &GameInput,
    level: &mut Level,
    game_state: &mut ResMut<State<GameState>>,
    levels: &LevelHandles,
    level_states: &Res<Assets<LevelState>>,
    audio: &Audio,
    sounds: &mut Sounds,
) {
    match input {
        GameInput::Direction(direction) => {
            handle_direction(level, direction, audio, sounds);
        }
        GameInput::Action(action) => {
            handle_action(
                action,
                level,
                game_state,
                levels,
                level_states,
                audio,
                sounds,
            );
        }
    }
}

fn update_position(direction: &Direction, position: &mut MapPosition) {
    match direction {
        Direction::Up => position.decrement_y(),
        Direction::Left => position.decrement_x(),
        Direction::Down => position.increment_y(),
        Direction::Right => position.increment_x(),
    };
}

fn handle_direction(level: &mut Level, direction: &Direction, audio: &Audio, sounds: &mut Sounds) {
    let audio_source = sounds.sfx.move_player.clone();
    let channel_id = &sounds.channels.sfx;
    audio.play_in_channel(audio_source, channel_id);
    level.save_snapshot();

    let mut next_position = level.state.player_position;
    update_position(direction, &mut next_position);

    let next_entity = level.get_entity(&next_position);
    match next_entity {
        MapEntity::B | MapEntity::P => {
            let audio_source = sounds.sfx.push_box.clone();
            let channel_id = &sounds.channels.sfx;
            audio.play_in_channel(audio_source, channel_id);

            let in_zone = matches!(next_entity, MapEntity::P);
            let updated_next_entity = if in_zone { MapEntity::Z } else { MapEntity::F };

            let mut adjacent_position = next_position;
            update_position(direction, &mut adjacent_position);

            let adjacent_entity = level.get_entity(&adjacent_position);
            match adjacent_entity {
                MapEntity::F => {
                    level.set_entity(&next_position, updated_next_entity);
                    level.set_entity(&adjacent_position, MapEntity::B);
                    level.move_player(next_position);
                    level.increment_moves();

                    if in_zone {
                        level.increment_remaining_zones();
                    }
                }
                MapEntity::Z => {
                    let audio_source = sounds.sfx.set_zone.clone();
                    let channel_id = &sounds.channels.sfx;
                    audio.play_in_channel(audio_source, channel_id);

                    level.set_entity(&next_position, updated_next_entity);
                    level.set_entity(&adjacent_position, MapEntity::P);
                    level.move_player(next_position);
                    level.increment_moves();

                    if !in_zone {
                        level.decrement_remaining_zones();
                    }
                }
                _ => (),
            }
        }
        MapEntity::W => {}
        _ => {
            level.move_player(next_position);
            level.increment_moves();
        }
    }
}

fn handle_action(
    action: &Action,
    level: &mut Level,
    state: &mut ResMut<State<GameState>>,
    levels: &LevelHandles,
    level_states: &Res<Assets<LevelState>>,
    audio: &Audio,
    sounds: &mut Sounds,
) {
    match action {
        Action::Undo => {
            if level.undo() {
                let audio_source = sounds.sfx.undo_move.clone();
                let channel_id = &sounds.channels.sfx;
                audio.play_in_channel(audio_source, channel_id);
            }
        }
        Action::Reload => {
            let audio_source = sounds.sfx.reload_level.clone();
            let channel_id = &sounds.channels.sfx;
            audio.play_in_channel(audio_source, channel_id);
            level::reload(level, levels, level_states);
        }
        Action::Selection => {
            let audio_source = sounds.sfx.push_box.clone();
            let channel_id = &sounds.channels.sfx;
            audio.play_in_channel(audio_source, channel_id);
            handle_selection_action(&level.tag, state)
        }
        Action::Exit => {
            let audio_source = sounds.sfx.push_box.clone();
            let channel_id = &sounds.channels.sfx;
            audio.play_in_channel(audio_source, channel_id);
            state.set(GameState::stock_selection()).unwrap();
        }
        Action::Volume => {
            let audio_source = sounds.sfx.toggle_volume.clone();
            let channel_id = &sounds.channels.sfx;
            audio.play_in_channel(audio_source, channel_id);

            if sounds.volume < 0.1 {
                sounds.volume = 1.0;
            } else {
                sounds.volume -= 0.25;
            }

            audio.set_volume_in_channel(sounds.volume / 2.0, &sounds.channels.music);
            audio.set_volume_in_channel(sounds.volume, &sounds.channels.sfx);
        }
        _ => (),
    }
}

fn handle_selection_action(tag: &LevelTag, state: &mut ResMut<State<GameState>>) {
    match tag {
        LevelTag::Stock(_) => state.set(GameState::stock_selection()).unwrap(),
    }
}
