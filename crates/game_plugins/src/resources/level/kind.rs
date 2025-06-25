use crate::resources::prelude::*;

#[derive(Clone)]
pub enum LevelKind {
    Stock(usize),
    Editable,
    Custom(String),
    Playtest(LevelState),
}

impl LevelKind {
    pub fn get_playtest_state(&self) -> LevelState {
        match &self {
            LevelKind::Playtest(state) => *state,
            _ => unreachable!("Cannot get an state from a level kind that is not playtest"),
        }
    }
}
