use uuid::Uuid;

use super::LevelState;

pub enum LevelTag {
    Stock(usize),
    Custom(Uuid),
    Test(LevelState),
}
