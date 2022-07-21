use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;

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
    sfx: &AudioChannel<Sfx>,
    music: &AudioChannel<Music>,
    sounds: &mut Sounds,
    player_animation: &mut PlayerAnimation,
) {
    match input {
        GameInput::Direction(direction) => {
            match direction {
                Direction::Down => level.set_sprite_index(0),
                Direction::Up => level.set_sprite_index(1),
                Direction::Left => level.set_sprite_index(2),
                Direction::Right => level.set_sprite_index(3),
            }
            handle_direction(level, direction, sfx, sounds, player_animation);
        }
        GameInput::Action(action) => {
            handle_action(
                action,
                level,
                game_state,
                levels,
                level_states,
                sfx,
                music,
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

fn handle_direction(
    level: &mut Level,
    direction: &Direction,
    sfx: &AudioChannel<Sfx>,
    sounds: &Sounds,
    player_animation: &mut PlayerAnimation,
) {
    level.save_snapshot();

    let mut next_position = level.state.player_position;
    update_position(direction, &mut next_position);

    let next_entity = level.get_entity(&next_position);
    match next_entity {
        MapEntity::B | MapEntity::P => {
            player_animation.idle_timer.reset();
            player_animation.long_idle_timer.reset();

            let in_zone = matches!(next_entity, MapEntity::P);
            let updated_next_entity = if in_zone { MapEntity::Z } else { MapEntity::F };

            let mut adjacent_position = next_position;
            update_position(direction, &mut adjacent_position);

            let adjacent_entity = level.get_entity(&adjacent_position);
            match adjacent_entity {
                MapEntity::F => {
                    sfx.play(sounds.sfx.move_player.clone());
                    sfx.play(sounds.sfx.push_box.clone());

                    level.set_entity(&next_position, updated_next_entity);
                    level.set_entity(&adjacent_position, MapEntity::B);
                    level.move_player(next_position);
                    level.increment_moves();

                    if in_zone {
                        level.increment_remaining_zones();
                    }
                }
                MapEntity::Z => {
                    sfx.play(sounds.sfx.move_player.clone());
                    sfx.play(sounds.sfx.push_box.clone());
                    sfx.play(sounds.sfx.set_zone.clone());

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
            sfx.play(sounds.sfx.move_player.clone());

            player_animation.idle_timer.reset();
            player_animation.long_idle_timer.reset();

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
    sfx: &AudioChannel<Sfx>,
    music: &AudioChannel<Music>,
    sounds: &mut Sounds,
) {
    match action {
        Action::Undo => {
            if level.undo() {
                sfx.play(sounds.sfx.undo_move.clone());
            }
        }
        Action::Reload => {
            sfx.play(sounds.sfx.reload_level.clone());
            level::reload(level, levels, level_states);
        }
        Action::Selection => {
            sfx.play(sounds.sfx.push_box.clone());
            handle_selection_action(&level.tag, state);
        }
        Action::Exit => {
            sfx.play(sounds.sfx.push_box.clone());
            state.set(GameState::stock_selection()).unwrap();
        }
        Action::Volume => {
            if sounds.volume < 0.1 {
                sounds.volume = 1.0;
            } else {
                sounds.volume -= 0.25;
            }

            music.set_volume(sounds.volume / 2.0);
            sfx.set_volume(sounds.volume);

            sfx.play(sounds.sfx.toggle_volume.clone());
        }
        Action::Pick => (),
    }
}

fn handle_selection_action(tag: &LevelTag, state: &mut ResMut<State<GameState>>) {
    match tag {
        LevelTag::Stock(_) => state.set(GameState::stock_selection()).unwrap(),
    }
}
