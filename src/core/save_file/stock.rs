use crate::resources::{level::TOTAL_STOCK_LEVELS, prelude::*};

#[must_use]
pub fn total(save_file: &SaveFile) -> usize {
    save_file.stock.iter().sum()
}

pub fn unlock(save_file: &mut SaveFile, level: &Level) {
    let LevelTag::Stock(index) = level.tag;
    if save_file.stock_levels_len() == index + 1
        && save_file.stock_levels_len() > TOTAL_STOCK_LEVELS
    {
        save_file.insert_stock_level_record(0);
    }
}
