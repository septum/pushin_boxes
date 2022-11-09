mod entity;
mod position;

pub use entity::MapEntity;
pub use position::MapPosition;

pub const MAP_COLS: usize = 10;
pub const MAP_ROWS: usize = 10;

pub type Map = [[MapEntity; MAP_COLS]; MAP_ROWS];
