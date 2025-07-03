use bevy::prelude::*;
use game_core::level::LevelKind;

#[derive(Event)]
pub struct LevelInsertionEvent(LevelKind);

impl LevelInsertionEvent {
    pub fn new(kind: LevelKind) -> LevelInsertionEvent {
        LevelInsertionEvent(kind)
    }

    pub fn kind(&self) -> &LevelKind {
        &self.0
    }
}
