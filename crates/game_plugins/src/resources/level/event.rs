use bevy::prelude::*;

use super::LevelKind;

#[derive(Event)]
pub struct LevelInsertionEvent {
    pub kind: LevelKind,
}

impl LevelInsertionEvent {
    pub fn new(kind: LevelKind) -> LevelInsertionEvent {
        LevelInsertionEvent { kind }
    }
}
