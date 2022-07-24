mod handle;

use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};

pub use handle::SaveFileHandle;

#[derive(TypeUuid, Serialize, Deserialize, Clone)]
#[uuid = "2e5bbfc2-8dfd-4547-8c85-cbaf27533998"]
pub struct SaveFile {
    pub stock: Vec<(usize, f32)>,
}

impl Default for SaveFile {
    fn default() -> SaveFile {
        SaveFile {
            stock: vec![(0, 0.0)],
        }
    }
}

impl SaveFile {
    #[must_use]
    pub fn new(stock: Vec<(usize, f32)>) -> SaveFile {
        SaveFile { stock }
    }

    #[must_use]
    pub fn stock_levels_len(&self) -> usize {
        self.stock.len()
    }

    pub fn insert_stock_level_record(&mut self, record: (usize, f32)) {
        self.stock.push(record);
    }

    pub fn set_stock_level_record(&mut self, index: &usize, record: (usize, f32)) {
        self.stock[*index] = record;
    }

    #[must_use]
    pub fn get_stock_level_record(&self, index: &usize) -> (usize, f32) {
        self.stock[*index]
    }
}
