use super::state::LevelState;

// TODO: Maybe change to `LevelKind`
#[derive(Clone)]
pub enum LevelTag {
    Stock(usize),
    Editable,
    Playtest(LevelState),
}

impl Default for LevelTag {
    fn default() -> Self {
        LevelTag::Stock(0)
    }
}
