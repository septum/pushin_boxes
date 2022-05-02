use super::state::LevelState;

pub const MAX_SNAPSHOTS: usize = 4;

pub type LevelSnapshots = [Option<LevelState>; MAX_SNAPSHOTS];
