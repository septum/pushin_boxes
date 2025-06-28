use serde::{Deserialize, Serialize};

use super::{MAP_COLS, MAP_ROWS};

#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Copy)]
pub struct MapPosition {
    pub(super) x: usize,
    pub(super) y: usize,
}

impl MapPosition {
    pub fn new(x: usize, y: usize) -> MapPosition {
        assert!(x < MAP_COLS);
        assert!(y < MAP_ROWS);
        MapPosition { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn increment_x(&mut self) {
        if self.x < MAP_COLS - 1 {
            self.x += 1;
        }
    }

    pub fn increment_y(&mut self) {
        if self.y < MAP_ROWS - 1 {
            self.y += 1;
        }
    }

    pub fn decrement_x(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn decrement_y(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }
}
