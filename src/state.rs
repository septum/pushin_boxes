#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SelectionState {
    Stock,
    Custom,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Startup,
    Loading,
    Title,
    Editor,
    Instructions,
    Selection(SelectionState),
    Level,
    Win,
}

impl GameState {
    pub fn custom_selection() -> GameState {
        GameState::Selection(SelectionState::Custom)
    }

    pub fn stock_selection() -> GameState {
        GameState::Selection(SelectionState::Stock)
    }
}
