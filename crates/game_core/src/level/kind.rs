use super::state::LevelState;

#[derive(Clone)]
pub enum LevelKind {
    Stock(usize),
    Custom(String),
    Editable(LevelState),
}

impl Default for LevelKind {
    fn default() -> Self {
        LevelKind::Editable(LevelState::default())
    }
}
