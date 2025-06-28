use bevy::prelude::*;

use crate::level::internal::LevelKind;

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
