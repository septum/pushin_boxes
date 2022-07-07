use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::{MAP_COLS, MAP_ROWS};

#[derive(Component, Serialize, Deserialize, Clone, Copy)]
pub struct MapPosition {
    pub x: usize,
    pub y: usize,
}

impl MapPosition {
    #[must_use]
    pub fn new(x: usize, y: usize) -> MapPosition {
        MapPosition { x, y }
    }

    #[must_use]
    pub fn equals(&self, other: &MapPosition) -> bool {
        self.x == other.x && self.y == other.y
    }

    pub fn increment_x(&mut self) {
        if self.x < MAP_COLS - 1 {
            self.x = self.x.saturating_add(1);
        }
    }

    pub fn increment_y(&mut self) {
        if self.y < MAP_ROWS - 1 {
            self.y = self.y.saturating_add(1);
        }
    }

    pub fn decrement_x(&mut self) {
        if self.x > 0 {
            self.x = self.x.saturating_sub(1);
        }
    }

    pub fn decrement_y(&mut self) {
        if self.y > 0 {
            self.y = self.y.saturating_sub(1);
        }
    }
}
