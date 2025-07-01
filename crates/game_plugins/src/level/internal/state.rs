use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::level::internal::map::{Map, MapPosition};

#[derive(Asset, TypePath, Serialize, Deserialize, Default, Clone, Copy)]
pub struct LevelState {
    pub(super) map: Map,
    pub(super) character_position: MapPosition,
    pub(super) character_facing_direction: usize,
    pub(super) remaining_zones: usize,
}
