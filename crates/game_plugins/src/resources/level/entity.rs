use bevy::prelude::*;

use game_map::MapEntity;

use crate::resources::prelude::*;

pub trait MapEntityToImage {
    fn to_image(&self, images: &Images) -> Handle<Image>;
}

impl MapEntityToImage for MapEntity {
    fn to_image(&self, images: &Images) -> Handle<Image> {
        match self {
            MapEntity::V => images.entity_void.clone(),
            MapEntity::F => images.entity_floor.clone(),
            MapEntity::Z => images.entity_zone.clone(),
            MapEntity::B => images.entity_box.clone(),
            MapEntity::P => images.entity_placed_box.clone(),
        }
    }
}
