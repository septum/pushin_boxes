#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Selection {
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
    Selection(Selection),
    Level,
    Win,
}

impl GameState {
    pub fn custom_selection() -> GameState {
        GameState::Selection(Selection::Custom)
    }

    pub fn stock_selection() -> GameState {
        GameState::Selection(Selection::Stock)
    }
}
