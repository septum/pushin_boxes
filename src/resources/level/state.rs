use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};

use super::{
    map::{Map, MapEntity, MapPosition},
    MAP_COLS, MAP_ROWS,
};

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy)]
#[uuid = "d1e78377-22a5-49f7-a675-60d348abc837"]
pub struct LevelState {
    pub map: Map,
    pub player_position: MapPosition,
    pub sprite_index: usize,
    pub remaining_zones: usize,
}

impl Default for LevelState {
    fn default() -> LevelState {
        LevelState {
            map: [[MapEntity::F; MAP_COLS]; MAP_ROWS],
            player_position: MapPosition::new(0, 0),
            sprite_index: 0,
            remaining_zones: 0,
        }
    }
}
