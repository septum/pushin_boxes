use uuid::Uuid;

use super::state::LevelState;

// does this make sense?
pub enum LevelTag {
    Stock(usize),
    Custom(Uuid),
    Test(LevelState),
}
