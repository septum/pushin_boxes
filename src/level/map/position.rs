use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    config::{GAME_HEIGHT, GAME_WIDTH, MAP_COLS, MAP_ROWS, SPRITE_OFFSET, SPRITE_SIZE},
    input::DirectionKind,
};

#[derive(Component, Serialize, Deserialize, Clone, Copy)]
pub struct MapPosition {
    pub x: usize,
    pub y: usize,
}

impl MapPosition {
    pub fn new(x: usize, y: usize) -> MapPosition {
        MapPosition { x, y }
    }

    fn increment_x(&mut self) {
        if self.x < MAP_COLS - 1 {
            self.x = self.x.saturating_add(1);
        }
    }

    fn increment_y(&mut self) {
        if self.y < MAP_ROWS - 1 {
            self.y = self.y.saturating_add(1);
        }
    }

    fn decrement_x(&mut self) {
        self.x = self.x.saturating_sub(1);
    }

    fn decrement_y(&mut self) {
        self.y = self.y.saturating_sub(1);
    }

    pub fn advance(&mut self, direction: &DirectionKind) {
        match direction {
            DirectionKind::Up => self.decrement_y(),
            DirectionKind::Left => self.decrement_x(),
            DirectionKind::Down => self.increment_y(),
            DirectionKind::Right => self.increment_x(),
        };
    }

    pub fn apply_to_transform(&self, transform: &mut Transform) {
        // calculate coords with the correct sprite dimension
        // and moving the origin/pivot from the center to the top-left
        let x = ((self.x * SPRITE_SIZE) + SPRITE_OFFSET) as f32;
        let y = (((MAP_ROWS - self.y) * SPRITE_SIZE) - SPRITE_OFFSET) as f32;

        // take into account the camera default position (0, 0)
        transform.translation.x = x - (GAME_WIDTH / 2.0);
        transform.translation.y = y - (GAME_HEIGHT / 2.0);
    }
}
