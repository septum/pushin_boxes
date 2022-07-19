pub mod input;
pub mod level;
pub mod save_file;
pub mod state;
pub mod ui;

pub const SPRITE_SIZE: usize = 64;
pub const SPRITE_OFFSET: usize = 32;

pub const ENTITY_SURFACE: usize = 36;
pub const ENTITY_EDGE: usize = 28;
pub const ENTITY_SURFACE_OFFSET: usize = 18;
pub const ENTITY_ON_TOP_OFFSET: usize = 28;
pub const BOX_ENTITY_OFFSET: usize = 14;

pub const MAP_WIDTH: f32 = 640.0;
pub const MAP_HEIGHT: f32 = 388.0;
