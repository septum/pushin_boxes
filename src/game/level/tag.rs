use bevy::prelude::*;

use crate::resources::prelude::*;

pub fn to_default_state(
    tag: &LevelTag,
    levels: &LevelHandles,
    level_states: &Res<Assets<LevelState>>,
) -> LevelState {
    match tag {
        LevelTag::Stock(index) => {
            let handle = &levels.stock[*index];
            *level_states.get(handle).unwrap()
        }
        LevelTag::Custom(uuid) => {
            let handle = &levels.custom[uuid];
            *level_states.get(handle).unwrap()
        }
        LevelTag::Test(state) => *state,
    }
}
