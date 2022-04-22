use super::{state::LevelState, MAX_LEVEL_STATES};

pub type LevelSnapshots = [Option<LevelState>; MAX_LEVEL_STATES];
