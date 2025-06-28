use crate::level::internal::state::LevelState;

#[derive(Clone)]
pub enum LevelKind {
    Stock(usize),
    Custom(String),
    Playtest(LevelState),
    // TODO: This should be the one used in playtest
    Editable,
}

impl Default for LevelKind {
    fn default() -> Self {
        LevelKind::Stock(0)
    }
}
