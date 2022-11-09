use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct LevelRecord {
    moves: usize,
    time: f32,
}

impl LevelRecord {
    pub fn new(moves: usize, time: f32) -> LevelRecord {
        LevelRecord { moves, time }
    }

    pub fn is_set(&self) -> bool {
        self.moves > 0
    }

    pub fn time_string(&self) -> String {
        let duration = Duration::from_secs_f32(self.time);
        let milliseconds = duration.subsec_millis();
        let seconds = duration.as_secs() % 60;
        let minutes = (duration.as_secs() / 60) % 60;
        format!("{minutes:02}:{seconds:02}:{milliseconds:03}")
    }

    pub fn moves_string(&self) -> String {
        self.moves.to_string()
    }

    pub fn moves_in_time(&self, separator: &str) -> String {
        let moves = self.moves_string();
        let time = self.time_string();
        format!("{moves} moves{separator}in {time}")
    }

    pub fn is_better_than(&self, other: &LevelRecord) -> bool {
        self.moves == 0
            || self.moves > other.moves
            || self.moves >= other.moves && self.time > other.time
    }
}
