mod entity;
mod position;

pub use entity::MapEntity;
pub use position::MapPosition;

use super::{MAP_COLS, MAP_ROWS};

pub type Map = [[MapEntity; MAP_COLS]; MAP_ROWS];
