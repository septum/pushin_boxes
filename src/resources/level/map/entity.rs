use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::resources::prelude::Images;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub enum MapEntity {
    #[default]
    /// Wall
    W,
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
    pub fn to_image(&self, images: &Images) -> Option<Handle<Image>> {
        match self {
            MapEntity::W => None,
            MapEntity::F => Some(images.entity_floor.clone()),
            MapEntity::Z => Some(images.entity_zone.clone()),
            MapEntity::B | MapEntity::P => Some(images.entity_box.clone()),
        }
    }
}
