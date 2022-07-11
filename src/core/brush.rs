use bevy::prelude::*;

use crate::resources::{
    brush::{Brush, BrushSprite},
    level::map::{MAP_COLS, MAP_ROWS},
    prelude::*,
};

use super::{
    level, BOX_ENTITY_OFFSET, ENTITY_ON_TOP_OFFSET, ENTITY_SURFACE, MAP_HEIGHT, MAP_WIDTH,
    SPRITE_SIZE,
};

#[must_use]
pub fn to_image(brush: &Brush, images: &Images) -> Handle<Image> {
    match brush.current_sprite() {
        BrushSprite::Box => images.entities.pbox.clone(),
        BrushSprite::Player => images.player.pushin.clone(),
        BrushSprite::Wall => images.entities.wall.clone(),
        BrushSprite::Floor => images.entities.floor.clone(),
        BrushSprite::Zone => images.entities.zone.clone(),
    }
}

pub fn spawn(commands: &mut Commands, images: &Images) {
    let brush = Brush::default();
    let image = to_image(&brush, images);

    commands
        .spawn_bundle(SpriteBundle {
            texture: image,
            ..Default::default()
        })
        .insert(brush);
}

/// # Panics
///
/// Will panic if no primary window is found
#[must_use]
pub fn cursor_to_world_coords(
    windows: &Windows,
    camera_transform: &GlobalTransform,
    camera: &Camera,
) -> Vec2 {
    // from: https://bevy-cheatbook.github.io/cookbook/cursor2world.html
    let window = windows.get_primary().unwrap();
    let cursor_position = window.cursor_position().unwrap();
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    let ndc = (cursor_position / window_size) * 2.0 - Vec2::ONE;
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();

    let x = world_pos.x + (MAP_WIDTH / 2.0);
    let y = world_pos.y + (MAP_HEIGHT / 2.0);

    let x = x / SPRITE_SIZE as f32;
    let y = y / ENTITY_SURFACE as f32;

    Vec2::new(x, y)
}

pub fn lock_to_grid(
    brush: &Brush,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    windows: &Windows,
    translation: &mut Vec3,
) {
    let coords = cursor_to_world_coords(windows, camera_transform, camera);
    let x = coords.x as usize;
    let y = coords.y as usize;

    if x < MAP_ROWS && y < MAP_COLS {
        let y = (MAP_COLS - 1) - y;
        let position = MapPosition::new(x, y);
        level::position::update_brush_translation(&position, translation);

        if matches!(brush.current_sprite(), &BrushSprite::Box) {
            translation.y += BOX_ENTITY_OFFSET as f32;
        } else if matches!(brush.current_sprite(), &BrushSprite::Player) {
            translation.y += ENTITY_ON_TOP_OFFSET as f32;
        }
    }
}

pub fn add_entity_to_map(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    windows: &Windows,
    level: &mut Level,
    brush: &Brush,
) {
    let coords = cursor_to_world_coords(windows, camera_transform, camera);
    let x = coords.x as usize;
    let y = coords.y as usize;

    if x < MAP_ROWS && y < MAP_COLS {
        let y = (MAP_COLS - 1) - y;
        let position = MapPosition::new(x, y);

        match brush.current_sprite() {
            BrushSprite::Player => match level.get_entity(&position) {
                MapEntity::F | MapEntity::Z => level.move_player(position),
                _ => {}
            },
            _ => {
                if !level.player_in(&position) {
                    if let MapEntity::Z = level.get_entity(&position) {
                        level.decrement_remaining_zones();
                    }

                    match brush.current_sprite() {
                        BrushSprite::Box => match level.get_entity(&position) {
                            MapEntity::Z => level.set_entity(&position, MapEntity::P),
                            _ => level.set_entity(&position, MapEntity::B),
                        },
                        BrushSprite::Wall => level.set_entity(&position, MapEntity::W),
                        BrushSprite::Floor => level.set_entity(&position, MapEntity::F),
                        BrushSprite::Zone => {
                            level.set_entity(&position, MapEntity::Z);
                            level.increment_remaining_zones();
                        }
                        BrushSprite::Player => {}
                    }
                }
            }
        }
    }
}
