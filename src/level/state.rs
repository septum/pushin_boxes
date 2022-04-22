use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};

use super::map::{LevelMap, MapPosition};

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy)]
#[uuid = "d1e78377-22a5-49f7-a675-60d348abc837"]
pub struct LevelState {
    pub map: LevelMap,
    pub remaining_zones: usize,
    pub player_position: MapPosition,
}
