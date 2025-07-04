use std::time::Duration;

use crate::{
    input::{Action, Direction, Input},
    level::{LevelUpdate, data::LevelData},
    map::{MAP_COLS, MAP_ROWS, MapEntity, MapPosition},
};

use super::{kind::LevelKind, record::LevelRecord, snapshots::LevelSnapshots, state::LevelState};

#[derive(Default)]
pub struct Level {
    kind: LevelKind,
    state: LevelData,
    record: LevelRecord,
    snapshots: LevelSnapshots,
}

impl Level {
    pub fn new(kind: LevelKind, state: LevelState) -> Level {
        Level {
            kind,
            state: state.into(),
            ..Level::default()
        }
    }

    pub fn update(&mut self, input: &Input) -> Option<LevelUpdate> {
        match input {
            Input::Direction(direction) => self.handle_direction_input(direction),
            Input::Action(action) => self.handle_action_input(action),
        }
    }

    fn handle_direction_input(&mut self, direction: &Direction) -> Option<LevelUpdate> {
        match direction {
            Direction::Down => self.set_character_facing_direction(0),
            Direction::Up => self.set_character_facing_direction(1),
            Direction::Left => self.set_character_facing_direction(2),
            Direction::Right => self.set_character_facing_direction(3),
        }

        let mut next_position = self.character_position();
        next_position.update(direction);

        let next_entity = self.get_entity(&next_position);
        match next_entity {
            MapEntity::V => None,
            MapEntity::B | MapEntity::P => {
                let in_zone = matches!(next_entity, MapEntity::P);
                let updated_next_entity = if in_zone { MapEntity::Z } else { MapEntity::F };

                let mut adjacent_position = next_position;
                adjacent_position.update(direction);

                let adjacent_entity = self.get_entity(&adjacent_position);
                match adjacent_entity {
                    MapEntity::F => {
                        self.snapshots.save(&self.state);
                        self.set_entity(&next_position, updated_next_entity);
                        self.set_entity(&adjacent_position, MapEntity::B);
                        self.move_character(next_position);
                        self.record.increment_moves();

                        if in_zone {
                            self.state.increment_remaining_zones();
                        }

                        Some(LevelUpdate::PushBox)
                    }
                    MapEntity::Z => {
                        self.snapshots.save(&self.state);
                        self.set_entity(&next_position, updated_next_entity);
                        self.set_entity(&adjacent_position, MapEntity::P);
                        self.move_character(next_position);
                        self.record.increment_moves();

                        if !in_zone {
                            self.state.decrement_remaining_zones();
                        }

                        Some(LevelUpdate::PlaceBox)
                    }
                    _ => None,
                }
            }
            _ => {
                self.snapshots.save(&self.state);
                self.move_character(next_position);
                self.record.increment_moves();

                Some(LevelUpdate::MoveCharacter)
            }
        }
    }

    fn handle_action_input(&mut self, action: &Action) -> Option<LevelUpdate> {
        match action {
            Action::Undo => {
                if let Some(state) = self.snapshots.shift() {
                    *self.state = state;
                    self.record.decrement_moves();
                    Some(LevelUpdate::UndoMove)
                } else {
                    None
                }
            }
            Action::Reload => {
                self.state.reload();
                self.record.reset_moves();
                self.snapshots.reset();
                Some(LevelUpdate::Reload)
            }
            Action::Exit => Some(LevelUpdate::Exit),
            _ => None,
        }
    }

    pub fn kind(&self) -> &LevelKind {
        &self.kind
    }

    pub fn is_stock(&self) -> bool {
        matches!(self.kind, LevelKind::Stock(_))
    }

    pub fn name(&self) -> String {
        match &self.kind {
            LevelKind::Stock(index) => (index + 1).to_string(),
            LevelKind::Custom(key) => {
                let parsed_key: Vec<&str> = key.split('$').collect();
                parsed_key[0].to_string()
            }
            LevelKind::Editable(_) => "Playtest".to_string(),
        }
    }

    // TODO: Move or change this
    pub fn loop_over_entity_and_position<F>(&self, mut f: F)
    where
        F: FnMut(&MapEntity, MapPosition),
    {
        for column in 0..MAP_COLS {
            for row in 0..MAP_ROWS {
                let position = MapPosition::new(column, row);
                let entity = self.state.get_entity(&position);
                f(entity, position);
            }
        }
    }

    pub fn state(&self) -> &LevelState {
        &self.state
    }

    pub fn get_entity(&self, position: &MapPosition) -> &MapEntity {
        self.state.get_entity(position)
    }

    pub fn set_entity(&mut self, position: &MapPosition, entity: MapEntity) {
        self.state.set_entity(position, entity);
    }

    pub fn character_position(&self) -> MapPosition {
        self.state.character_position()
    }

    pub fn move_character(&mut self, position: MapPosition) {
        self.state.move_character(position);
    }

    // TODO: This can be an actual direction
    pub fn character_facing_direction(&self) -> usize {
        self.state.character_facing_direction()
    }

    pub fn set_character_facing_direction(&mut self, direction: usize) {
        self.state.set_character_facing_direction(direction);
    }

    pub fn increment_remaining_zones(&mut self) {
        self.state.increment_remaining_zones();
    }

    pub fn decrement_remaining_zones(&mut self) {
        self.state.decrement_remaining_zones();
    }

    pub fn no_remaining_zones(&self) -> bool {
        self.state.no_remaining_zones()
    }

    pub fn undos_string(&self) -> String {
        self.snapshots.undos_string()
    }

    pub fn record(&self) -> &LevelRecord {
        &self.record
    }

    pub fn is_new_record(&self, other: &LevelRecord) -> bool {
        self.record.is_better_than(other)
    }

    pub fn moves_string(&self) -> String {
        self.record.moves_string()
    }

    pub fn time_string(&self) -> String {
        self.record.time_string()
    }

    pub fn moves_in_time(&self, separator: char) -> String {
        self.record.moves_in_time(separator)
    }

    pub fn tick_record(&mut self, delta: Duration) {
        self.record.tick(delta);
    }
}
