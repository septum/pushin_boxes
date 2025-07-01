use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use crate::level::internal::{
    kind::LevelKind,
    map::{MAP_COLS, MAP_ROWS, MapEntity, MapPosition},
    record::LevelRecord,
    state::LevelState,
};

pub const TOTAL_STOCK_LEVELS: usize = 16;
pub const TOTAL_CUSTOM_LEVELS: usize = 16;

const MAX_SNAPSHOTS: usize = 4;

struct LevelSnapshots {
    undos: usize,
    inner: [Option<LevelState>; MAX_SNAPSHOTS],
}

impl Default for LevelSnapshots {
    fn default() -> Self {
        Self {
            undos: MAX_SNAPSHOTS,
            inner: [None; MAX_SNAPSHOTS],
        }
    }
}

impl Deref for LevelSnapshots {
    type Target = [Option<LevelState>; MAX_SNAPSHOTS];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for LevelSnapshots {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

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

    pub fn state(&self) -> &LevelState {
        &self.state
    }

    pub fn record(&self) -> &LevelRecord {
        &self.record
    }

    pub fn is_new_record(&self, other: &LevelRecord) -> bool {
        self.record.is_better_than(other)
    }

    pub fn set_state(&mut self, state: LevelState) {
        self.snapshots = LevelSnapshots::default();
        self.state = state;
        self.snapshots.undos = 4;
        self.record.moves = 0;
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

    pub fn get_entity(&self, position: &MapPosition) -> &MapEntity {
        &self.state.map[position]
    }

    pub fn set_entity(&mut self, position: &MapPosition, entity: MapEntity) {
        self.state.map[position] = entity;
    }

    pub fn get_character_position(&self) -> &MapPosition {
        &self.state.character_position
    }

    pub fn get_character_facing_direction(&self) -> usize {
        self.state.character_facing_direction
    }

    pub fn set_character_facing_direction(&mut self, direction: usize) {
        self.state.character_facing_direction = direction;
    }

    pub fn move_character(&mut self, position: MapPosition) {
        self.state.character_position = position;
    }

    pub fn increment_remaining_zones(&mut self) {
        self.state.remaining_zones += 1;
    }

    pub fn decrement_remaining_zones(&mut self) {
        self.state.remaining_zones -= 1;
    }

    pub fn no_remaining_zones(&self) -> bool {
        self.state.remaining_zones == 0
    }

    pub fn character_position(&self) -> MapPosition {
        self.state.character_position
    }

    pub fn get_moves(&self) -> usize {
        self.record.moves
    }

    pub fn get_undos(&self) -> usize {
        self.snapshots.undos
    }

    pub fn increment_moves(&mut self) {
        self.record.moves += 1;
    }

    pub fn decrement_moves(&mut self) {
        self.record.moves = self.record.moves.saturating_sub(1);
    }

    pub fn decrement_undos(&mut self) {
        self.snapshots.undos = self.snapshots.undos.saturating_sub(1);
    }

    pub fn get_current_record(&self) -> &LevelRecord {
        &self.record
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

    pub fn save_snapshot(&mut self) {
        self.snapshots.rotate_right(1);
        self.snapshots[0] = Some(self.state);
    }

    pub fn undo(&mut self) -> bool {
        if self.snapshots.undos > 0 {
            if let Some(state) = self.snapshots[0] {
                self.state = state;
                self.snapshots.rotate_left(1);
                self.snapshots[MAX_SNAPSHOTS - 1] = None;
                self.decrement_undos();
                self.decrement_moves();
                return true;
            }
            return false;
        }
        false
    }

    pub fn tick_stopwatch(&mut self, delta: Duration) {
        self.record.stopwatch.tick(delta);
    }

    pub fn stopwatch_elapsed(&self) -> Duration {
        self.record.stopwatch.elapsed()
    }

    pub fn stopwatch_string(&self) -> String {
        let duration = self.stopwatch_elapsed();
        let milliseconds = duration.subsec_millis();
        let seconds = duration.as_secs() % 60;
        let minutes = (duration.as_secs() / 60) % 60;
        format!("{minutes:02}:{seconds:02}:{milliseconds:03}")
    }

    pub fn moves_string(&self) -> String {
        self.record.moves.to_string()
    }

    pub fn undos_string(&self) -> String {
        self.snapshots.undos.to_string()
    }

    pub fn moves_in_time(&self, separator: &str) -> String {
        let moves = self.record.moves.to_string();
        let time = self.stopwatch_string();
        format!("{moves} moves{separator}in {time}")
    }

    pub fn is_last(&self) -> bool {
        match self.kind {
            LevelKind::Stock(index) => index + 1 == TOTAL_STOCK_LEVELS,
            _ => unreachable!("There is no last level in other level kinds"),
        }
    }
}
