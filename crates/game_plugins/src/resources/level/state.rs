use bevy::{prelude::*, reflect::TypePath};
use serde::{Deserialize, Serialize};

use super::map::{MAP_COLS, MAP_ROWS, Map, MapEntity, MapPosition};

#[derive(Asset, TypePath, Serialize, Deserialize, Clone, Copy, Default)]
pub struct LevelState {
    pub animation_row: usize,
    pub map: Map,
    pub character_position: MapPosition,
    pub remaining_zones: usize,
}

impl LevelState {
    pub fn editor() -> Self {
        LevelState {
            map: [[MapEntity::F; MAP_COLS]; MAP_ROWS],
            character_position: MapPosition::new(4, 4),
            ..default()
        }
    }
}
