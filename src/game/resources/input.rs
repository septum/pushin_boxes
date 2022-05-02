use bevy::prelude::*;

use crate::{
    resources::{
        input::{Action, Direction, Input},
        prelude::*,
    },
    state::GameState,
};

use super::level;

pub fn process(
    input: &Input,
    level: &mut Level,
    game_state: &mut ResMut<State<GameState>>,
    levels: &LevelHandles,
    level_states: &Res<Assets<LevelState>>,
) {
    match input {
        Input::Direction(direction) => {
            handle_direction(level, direction);
        }
        Input::Action(action) => {
            handle_action(action, level, game_state, levels, level_states);
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

fn handle_direction(level: &mut Level, direction: &Direction) {
    level.save_snapshot();

    let mut next_position = level.state.player_position;
    update_position(direction, &mut next_position);

    let next_entity = level.get_entity(&next_position);
    match next_entity {
        MapEntity::B | MapEntity::P => {
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
) {
    match action {
        Action::Undo => level.undo(),
        Action::Reload => level::reload(level, levels, level_states),
        Action::Selection => handle_selection_action(&level.tag, state),
        Action::Exit => state.set(GameState::Title).unwrap(),
    }
}

fn handle_selection_action(tag: &LevelTag, state: &mut ResMut<State<GameState>>) {
    match tag {
        LevelTag::Stock(_) => state.set(GameState::stock_selection()).unwrap(),
        LevelTag::Custom(_) => state.set(GameState::custom_selection()).unwrap(),
        LevelTag::Test(_) => state.set(GameState::Editor).unwrap(),
    }
}
