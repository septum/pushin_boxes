use std::ops::{Deref, DerefMut};

use crate::level::LevelState;

#[derive(Default)]
pub struct LevelData {
    state: LevelState,
    initial_state: LevelState,
}

impl From<LevelState> for LevelData {
    fn from(value: LevelState) -> Self {
        LevelData {
            state: value,
            initial_state: value,
        }
    }
}

impl Deref for LevelData {
    type Target = LevelState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl DerefMut for LevelData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl LevelData {
    pub fn reload(&mut self) {
        self.state = self.initial_state;
    }
}
