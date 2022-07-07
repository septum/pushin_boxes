use bevy::prelude::*;

use crate::resources::prelude::*;

#[must_use] pub fn to_image(entity: &MapEntity, images: &Images) -> Handle<Image> {
    match entity {
        MapEntity::W => images.entities.wall.clone(),
        MapEntity::F => images.entities.floor.clone(),
        MapEntity::Z => images.entities.zone.clone(),
        MapEntity::B | MapEntity::P => images.entities.pbox.clone(),
    }
}
