// TODO: MAP_COLS, MAP_ROWS, and MapEntity are part of the game domain
// we need to find a way to make them generic

use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};

pub const MAP_COLS: usize = 10;
pub const MAP_ROWS: usize = 10;

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub enum MapEntity {
    #[default]
    /// Floor
    F,
    /// Zone
    Z,
    /// Box in Floor
    B,
    /// Box in Zone
    P,
    /// Void
    V,
}

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct Map([[MapEntity; MAP_COLS]; MAP_ROWS]);

impl Index<&MapPosition> for Map {
    type Output = MapEntity;

    fn index(&self, index: &MapPosition) -> &Self::Output {
        &self.0[index.y][index.x]
    }
}

impl IndexMut<&MapPosition> for Map {
    fn index_mut(&mut self, index: &MapPosition) -> &mut Self::Output {
        &mut self.0[index.y][index.x]
    }
}

impl Map {
    pub fn get_entity(&self, position: &MapPosition) -> &MapEntity {
        &self[position]
    }

    pub fn set_entity(&mut self, position: &MapPosition, entity: MapEntity) {
        self[position] = entity;
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct MapPosition {
    x: usize,
    y: usize,
}

impl PartialEq for MapPosition {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl MapPosition {
    pub fn new(x: usize, y: usize) -> MapPosition {
        assert!(x <= MAP_COLS);
        assert!(y <= MAP_ROWS);
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
