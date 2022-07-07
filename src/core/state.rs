#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SelectionState {
    Stock,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Startup,
    Loading,
    Title,
    Instructions,
    Selection(SelectionState),
    Level,
    Win,
}

impl GameState {
    #[must_use]
    pub fn stock_selection() -> GameState {
        GameState::Selection(SelectionState::Stock)
    }
}
