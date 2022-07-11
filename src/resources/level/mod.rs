mod handles;
mod snapshots;
mod state;
mod tag;

pub mod map;
pub mod prelude;

use map::{MAP_COLS, MAP_ROWS};
use prelude::*;
use snapshots::{LevelSnapshots, MAX_SNAPSHOTS};

pub const TOTAL_STOCK_LEVELS: usize = 16;

pub struct Level {
    pub tag: LevelTag,
    pub state: LevelState,
    pub record: usize,
    pub snapshots: LevelSnapshots,
    pub undos: usize,
    pub moves: usize,
}

impl Default for Level {
    fn default() -> Level {
        let state = LevelState::default();
        let tag = LevelTag::Stock(0);
        let record = 0;
        Level::new(tag, state, record)
    }
}

impl Level {
    #[must_use]
    pub fn new(tag: LevelTag, state: LevelState, record: usize) -> Level {
        Level {
            tag,
            state,
            record,
            snapshots: [None; MAX_SNAPSHOTS],
            undos: 4,
            moves: 0,
        }
    }

    pub fn set_state(&mut self, state: LevelState) {
        self.snapshots = [None; MAX_SNAPSHOTS];
        self.state = state;
        self.undos = 4;
        self.moves = 0;
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

    #[must_use]
    pub fn get_entity(&self, position: &MapPosition) -> &MapEntity {
        &self.state.map[position.y][position.x]
    }

    pub fn set_entity(&mut self, position: &MapPosition, entity: MapEntity) {
        self.state.map[position.y][position.x] = entity;
    }

    pub fn player_in(&mut self, position: &MapPosition) -> bool {
        self.state.player_position.equals(position)
    }

    #[must_use]
    pub fn get_player_position(&self) -> &MapPosition {
        &self.state.player_position
    }

    pub fn increment_moves(&mut self) {
        self.moves += 1;
    }

    pub fn decrement_moves(&mut self) {
        self.moves -= 1;
    }

    pub fn decrement_undos(&mut self) {
        self.undos -= 1;
    }

    pub fn move_player(&mut self, position: MapPosition) {
        self.state.player_position = position;
    }

    pub fn increment_remaining_zones(&mut self) {
        self.state.remaining_zones += 1;
    }

    pub fn decrement_remaining_zones(&mut self) {
        self.state.remaining_zones -= 1;
    }

    #[must_use]
    pub fn no_remaining_zones(&self) -> bool {
        self.state.remaining_zones == 0
    }

    #[must_use]
    pub fn is_record_set(&self) -> bool {
        self.record > 0
    }

    #[must_use]
    pub fn is_new_record(&self) -> bool {
        self.record == 0 || self.moves < self.record
    }

    #[must_use]
    pub fn is_stock(&self) -> bool {
        matches!(self.tag, LevelTag::Stock(_))
    }

    #[must_use]
    pub fn get_name(&self) -> String {
        match self.tag {
            LevelTag::Stock(index) => (index + 1).to_string(),
        }
    }

    pub fn save_snapshot(&mut self) {
        self.snapshots.rotate_right(1);
        self.snapshots[0] = Some(self.state);
    }

    pub fn undo(&mut self) -> bool {
        if self.undos > 0 {
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
        return false;
    }
}
