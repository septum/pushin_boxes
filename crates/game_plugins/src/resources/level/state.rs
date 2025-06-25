use bevy::{prelude::*, reflect::TypePath};
use serde::{Deserialize, Serialize};

use game_map::{Map, MapPosition};

#[derive(Asset, TypePath, Serialize, Deserialize, Default, Clone, Copy)]
pub struct LevelState {
    pub map: Map,
    pub character_position: MapPosition,
    pub animation_row: usize,
    pub remaining_zones: usize,
}
