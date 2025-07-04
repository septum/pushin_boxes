use bevy::prelude::*;

use game_core::map::{MAP_ROWS, MapPosition};

const SPRITE_SIZE: usize = 64;
const SPRITE_OFFSET: usize = 32;

const ENTITY_SURFACE: usize = 36;
const ENTITY_SURFACE_OFFSET: usize = 18;

const MAP_WIDTH: f32 = 640.0;
const MAP_HEIGHT: f32 = 388.0;

pub fn apply_position_to_translation(position: &MapPosition, translation: &mut Vec3) {
    // calculate coords with the correct sprite dimension
    // and moving the origin/pivot from the center to the top-left
    let x = ((position.x() * SPRITE_SIZE) + SPRITE_OFFSET) as f32;
    let y = (((MAP_ROWS - position.y()) * ENTITY_SURFACE) - ENTITY_SURFACE_OFFSET) as f32;

    // take into account the camera's default position (0, 0)
    translation.x = x - (MAP_WIDTH / 2.0);
    translation.y = y - (MAP_HEIGHT / 2.0);

    // adaptation of depthness in a 2D plane
    translation.z = position.y() as f32;
}
