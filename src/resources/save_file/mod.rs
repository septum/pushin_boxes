mod handle;

use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};

pub use handle::SaveFileHandle;

#[derive(TypeUuid, Serialize, Deserialize, Clone)]
#[uuid = "2e5bbfc2-8dfd-4547-8c85-cbaf27533998"]
pub struct SaveFile {
    pub stock: Vec<usize>,
}

impl Default for SaveFile {
    fn default() -> SaveFile {
        SaveFile { stock: vec![0] }
    }
}

impl SaveFile {
    #[must_use]
    pub fn new(stock: Vec<usize>) -> SaveFile {
        SaveFile { stock }
    }

    #[must_use]
    pub fn stock_levels_len(&self) -> usize {
        self.stock.len()
    }

    pub fn insert_stock_level_record(&mut self, moves: usize) {
        self.stock.push(moves);
    }

    pub fn set_stock_level_record(&mut self, index: &usize, moves: usize) {
        self.stock[*index] = moves;
    }

    #[must_use]
    pub fn get_stock_level_record(&self, index: &usize) -> usize {
        self.stock[*index]
    }
}
