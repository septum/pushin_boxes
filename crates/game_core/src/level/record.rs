use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct LevelRecord {
    moves: usize,
    time: f32,
}

impl LevelRecord {
    pub fn is_set(&self) -> bool {
        self.moves > 0
    }

    pub fn reset_moves(&mut self) {
        self.moves = 0;
    }

    pub fn increment_moves(&mut self) {
        self.moves = self.moves.saturating_add(1);
    }

    pub fn decrement_moves(&mut self) {
        self.moves = self.moves.saturating_sub(1);
    }

    pub fn time_string(&self) -> String {
        let time = Duration::from_secs_f32(self.time);
        let milliseconds = time.subsec_millis();
        let seconds = time.as_secs() % 60;
        let minutes = (time.as_secs() / 60) % 60;
        format!("{minutes:02}:{seconds:02}:{milliseconds:03}")
    }

    pub fn moves_string(&self) -> String {
        self.moves.to_string()
    }

    pub fn tick(&mut self, delta: Duration) {
        self.time += delta.as_secs_f32();
    }

    pub fn moves_in_time(&self, separator: char) -> String {
        let moves = self.moves_string();
        let time = self.time_string();
        format!("{moves} moves{separator}in {time}")
    }

    pub fn is_better_than(&self, other: &LevelRecord) -> bool {
        !other.is_set()
            || self.moves < other.moves
            || self.moves <= other.moves && self.time < other.time
    }
}
