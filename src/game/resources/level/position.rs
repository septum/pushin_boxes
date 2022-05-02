use bevy::prelude::*;

use crate::{
    config::{GAME_HEIGHT, GAME_WIDTH},
    game::{SPRITE_OFFSET, SPRITE_SIZE},
    resources::{level::map::MAP_ROWS, prelude::*},
};

pub fn update_entity_translation(position: &MapPosition, translation: &mut Vec3) {
    // calculate coords with the correct sprite dimension
    // and moving the origin/pivot from the center to the top-left
    let x = ((position.x * SPRITE_SIZE) + SPRITE_OFFSET) as f32;
    let y = (((MAP_ROWS - position.y) * SPRITE_SIZE) - SPRITE_OFFSET) as f32;

    // take into account the camera's default position (0, 0)
    translation.x = x - (GAME_WIDTH / 2.0);
    translation.y = y - (GAME_HEIGHT / 2.0);

    // adaptation of depthness in a 2D plane
    translation.z = position.y as f32;
}

pub fn update_player_translation(position: &MapPosition, translation: &mut Vec3) {
    update_entity_translation(position, translation);

    // put it above the map
    translation.z = (position.y + 1) as f32;
}

pub fn update_brush_translation(position: &MapPosition, translation: &mut Vec3) {
    update_entity_translation(position, translation);

    // put it above the level
    translation.z = (position.y + 2) as f32;
}
