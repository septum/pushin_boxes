use bevy::prelude::*;

use crate::resources::{level::TOTAL_STOCK_LEVELS, prelude::*, save_file::SaveFile};

pub fn insert(
    commands: &mut Commands,
    index: usize,
    save_file: &SaveFile,
    level_handles: &LevelHandles,
    level_states_assets: &Assets<LevelState>,
) {
    let handle = level_handles.stock[index].clone();
    let tag = LevelTag::Stock(index);
    let state = *level_states_assets.get(handle).unwrap();
    let record = save_file.get_stock_level_record(&index);
    let level = Level::new(tag, state, record);

    commands.insert_resource(level);
}

pub fn is_last(tag: &LevelTag) -> bool {
    match tag {
        LevelTag::Stock(index) => index + 1 == TOTAL_STOCK_LEVELS,
        _ => false,
    }
}
