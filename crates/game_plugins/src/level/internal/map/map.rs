use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};

use super::{MapEntity, MapPosition};

pub const MAP_COLS: usize = 10;
pub const MAP_ROWS: usize = 10;

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
