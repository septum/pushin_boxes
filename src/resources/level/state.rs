use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};

use super::map::{Map, MapPosition};

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Default)]
#[uuid = "d1e78377-22a5-49f7-a675-60d348abc837"]
pub struct LevelState {
    pub animation_row: usize,
    pub map: Map,
    pub character_position: MapPosition,
    pub remaining_zones: usize,
}
