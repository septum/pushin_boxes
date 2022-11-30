use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::resources::prelude::Images;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub enum MapEntity {
    #[default]
    /// Void
    V,
    /// Floor
    F,
    /// Zone
    Z,
    /// Box in Floor
    B,
    /// Box in Zone
    P,
}

impl MapEntity {
    pub fn to_image(&self, images: &Images) -> Handle<Image> {
        match self {
            MapEntity::V => images.entity_void.clone(),
            MapEntity::F => images.entity_floor.clone(),
            MapEntity::Z => images.entity_zone.clone(),
            MapEntity::B | MapEntity::P => images.entity_box.clone(),
        }
    }
}
