use bevy::prelude::*;

pub struct ConfigPlugin;

const GAME_TITLE: &str = "Pushin' Boxes";

pub const GAME_WIDTH: f32 = 640.0;
pub const GAME_HEIGHT: f32 = 640.0;

pub const MAP_ROWS: usize = 10;
pub const MAP_COLS: usize = 10;
pub const SPRITE_SIZE: usize = 64;
pub const SPRITE_OFFSET: usize = 32;

pub const MAX_TOTAL_LEVELS: usize = 16;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: GAME_TITLE.to_string(),
            width: GAME_WIDTH,
            height: GAME_HEIGHT,
            ..Default::default()
        });
    }
}
