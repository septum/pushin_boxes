use super::state::LevelState;

// TODO: Maybe change to `LevelKind`
#[derive(Clone)]
pub enum LevelTag {
    Stock(usize),
    Editable,
    Custom(String),
    Playtest(LevelState),
}

impl Default for LevelTag {
    fn default() -> Self {
        LevelTag::Stock(0)
    }
}

impl LevelTag {
    pub fn get_playtest_state(&self) -> LevelState {
        match &self {
            LevelTag::Playtest(state) => *state,
            _ => unreachable!("Cannot get an state from tag that is not playtest"),
        }
    }
}
