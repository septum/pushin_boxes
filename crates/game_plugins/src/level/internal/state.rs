use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::level::internal::map::{Map, MapPosition};

#[derive(Asset, TypePath, Serialize, Deserialize, Default, Clone, Copy)]
pub struct LevelState {
    pub(crate) map: Map,
    pub(crate) character_position: MapPosition,
    pub(crate) character_facing_direction: usize,
    pub(crate) remaining_zones: usize,
}
