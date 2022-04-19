#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Startup,
    Loading,
    Title,
    Instructions,
    Selection,
    Level,
    Win,
}
