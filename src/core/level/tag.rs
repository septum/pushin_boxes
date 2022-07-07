use bevy::prelude::*;

use crate::resources::prelude::*;

/// # Panics
///
/// Will panic if no level states asset is found
#[must_use]
pub fn to_default_state(
    tag: &LevelTag,
    levels: &LevelHandles,
    level_states_assets: &Assets<LevelState>,
) -> LevelState {
    match tag {
        LevelTag::Stock(index) => {
            let handle = &levels.stock[*index];
            *level_states_assets.get(handle).unwrap()
        }
    }
}
