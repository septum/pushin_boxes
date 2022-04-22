#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SelectionKind {
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
    Selection(SelectionKind),
    Level,
    Win,
}
