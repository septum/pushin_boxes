use bevy::prelude::*;

use crate::resources::prelude::*;

pub const MAX_SNAPSHOTS: usize = 4;

#[derive(Default, Deref, DerefMut)]
pub struct LevelSnapshots([Option<LevelState>; MAX_SNAPSHOTS]);
