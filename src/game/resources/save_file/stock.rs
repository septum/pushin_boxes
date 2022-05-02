use crate::resources::prelude::*;

pub fn total(save_file: &SaveFile) -> usize {
    save_file.stock.iter().sum()
}

pub fn unlock(save_file: &mut SaveFile, level: &Level) {
    if let LevelTag::Stock(index) = level.tag {
        if save_file.stock_levels_len() == index + 1 {
            save_file.insert_stock_level_record(0);
        }
    };
}
