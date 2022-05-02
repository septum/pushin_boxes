use bevy::prelude::*;

use crate::{
    config::{GAME_HEIGHT, GAME_WIDTH},
    resources::{
        brush::{Brush, BrushSprite},
        level::map::MAP_COLS,
        prelude::*,
    },
};

use super::{level, SPRITE_SIZE};

pub fn to_image(brush: &Brush, images: &Images) -> Handle<Image> {
    match brush.current_sprite() {
        BrushSprite::Box => images.entities.pbox.clone(),
        BrushSprite::Player => images.player.idle.clone(),
        BrushSprite::Wall => images.entities.wall.clone(),
        BrushSprite::Floor => images.entities.floor.clone(),
        BrushSprite::Zone => images.entities.zone.clone(),
    }
}

pub fn spawn(commands: &mut Commands, images: &Images) {
    let brush = Brush::default();
    let image = to_image(&brush, images);
    // `z` is 20 to stand above the map
    let transform = Transform::from_xyz(0.0, 0.0, 20.0);

    commands
        .spawn_bundle(SpriteBundle {
            texture: image,
            transform,
            ..Default::default()
        })
        .insert(brush);
}

pub fn lock_brush_to_map_grid(position: &Vec2, translation: &mut Vec3) {
    if position.x > 0.0 && position.x < GAME_WIDTH && position.y > 0.0 && position.y < GAME_HEIGHT {
        let x = position.x as usize / SPRITE_SIZE;
        let y = (MAP_COLS - 1) - (position.y as usize / SPRITE_SIZE);
        let position = MapPosition::new(x, y);
        level::position::update_entity_translation(&position, translation);
    }
}

pub fn add_entity_to_map(position: &Vec2, level: &mut Level, brush: &Brush) {
    if position.x > 0.0 && position.x < GAME_WIDTH && position.y > 0.0 && position.y < GAME_HEIGHT {
        let x = position.x as usize / SPRITE_SIZE;
        let y = (MAP_COLS - 1) - (position.y as usize / SPRITE_SIZE);
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
                        _ => {}
                    }
                }
            }
        }
    }
}
