mod entity;
mod position;

use crate::config::{MAP_COLS, MAP_ROWS};

pub use entity::MapEntity;
pub use position::MapPosition;

pub type LevelMap = [[MapEntity; MAP_COLS]; MAP_ROWS];
