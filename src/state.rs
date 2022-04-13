#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Startup,
    Loading,
    Title,
    Selection,
    Level,
    Win,
}
