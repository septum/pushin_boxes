use std::time::Duration;

use super::{
    kind::LevelKind,
    map::{MAP_COLS, MAP_ROWS, MapEntity, MapPosition},
    record::LevelRecord,
    snapshots::LevelSnapshots,
    state::LevelState,
};

pub const TOTAL_STOCK_LEVELS: usize = 16;
pub const TOTAL_CUSTOM_LEVELS: usize = 16;

#[derive(Default)]
pub struct Level {
    kind: LevelKind,
    state: LevelState,
    record: LevelRecord,
    snapshots: LevelSnapshots,
}

impl Level {
    pub fn new(kind: LevelKind, state: LevelState) -> Level {
        Level {
            kind,
            state,
            ..Level::default()
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

    pub fn is_last(&self) -> bool {
        match self.kind {
            LevelKind::Stock(index) => index + 1 == TOTAL_STOCK_LEVELS,
            _ => unreachable!("There is no last level in other level kinds"),
        }
    }

    pub fn loop_over_entity_and_position<F>(&self, mut f: F)
    where
        F: FnMut(&MapEntity, MapPosition),
    {
        for column in 0..MAP_COLS {
            for row in 0..MAP_ROWS {
                let position = MapPosition::new(column, row);
                let entity = self.get_entity(&position);
                f(entity, position);
            }
        }
    }

    pub fn state(&self) -> &LevelState {
        &self.state
    }

    pub fn set_state(&mut self, state: LevelState) {
        self.state = state;
        self.snapshots.reset();
        self.record.reset_moves();
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

    pub fn max_undos_available(&self) -> bool {
        self.snapshots.max_undos_available()
    }

    pub fn save_snapshot(&mut self) {
        self.snapshots.capture(self.state);
    }

    pub fn undo(&mut self) -> bool {
        if let Some(state) = self.snapshots.shift() {
            self.state = state;
            self.decrement_moves();
            return true;
        }
        false
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

    pub fn record_is_set(&self) -> bool {
        self.record.is_set()
    }

    pub fn increment_moves(&mut self) {
        self.record.increment_moves();
    }

    pub fn decrement_moves(&mut self) {
        self.record.decrement_moves();
    }

    pub fn moves_string(&self) -> String {
        self.record.moves_string()
    }

    pub fn tick_stopwatch(&mut self, delta: Duration) {
        self.record.tick(delta);
    }

    pub fn stopwatch_string(&self) -> String {
        self.record.stopwatch_string()
    }

    pub fn moves_in_time(&self, separator: &str) -> String {
        self.record.moves_in_time(separator)
    }
}
