use bevy::prelude::*;

use super::kind::LevelKind;

#[derive(Event)]
pub struct LevelInsertionEvent {
    pub kind: LevelKind,
}

impl LevelInsertionEvent {
    pub fn new(kind: LevelKind) -> LevelInsertionEvent {
        LevelInsertionEvent { kind }
    }
}
