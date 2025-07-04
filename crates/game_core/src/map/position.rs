use serde::{Deserialize, Serialize};

use crate::input::Direction;

use super::{MAP_COLS, MAP_ROWS};

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct MapPosition {
    pub(super) x: usize,
    pub(super) y: usize,
}

impl Default for MapPosition {
    fn default() -> Self {
        Self {
            x: (MAP_COLS - 1) / 2,
            y: (MAP_COLS - 1) / 2,
        }
    }
}

impl MapPosition {
    pub fn new(x: usize, y: usize) -> MapPosition {
        assert!(x < MAP_COLS);
        assert!(y < MAP_ROWS);
        MapPosition { x, y }
    }

    pub fn update(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.decrement_y(),
            Direction::Left => self.decrement_x(),
            Direction::Down => self.increment_y(),
            Direction::Right => self.increment_x(),
        }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    fn increment_x(&mut self) {
        if self.x < MAP_COLS - 1 {
            self.x += 1;
        }
    }

    fn increment_y(&mut self) {
        if self.y < MAP_ROWS - 1 {
            self.y += 1;
        }
    }

    fn decrement_x(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    fn decrement_y(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }
}
