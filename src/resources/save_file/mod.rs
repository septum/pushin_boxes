mod data;
mod handle;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use hashbrown::HashMap;
use ron;
use uuid::Uuid;

use crate::config::MAX_TOTAL_LEVELS;
use crate::level::LevelTag;

pub use data::SaveFileData;
pub use handle::SaveFileHandle;

pub struct SaveFile {
    pub stock: Vec<usize>,
    pub custom: HashMap<Uuid, usize>,
}

impl SaveFile {
    pub fn new(data: &SaveFileData) -> SaveFile {
        SaveFile {
            stock: data.stock.clone(),
            custom: data.custom.clone(),
        }
    }

    pub fn no_records(&self) -> bool {
        self.stock.is_empty() && self.custom.is_empty()
    }

    pub fn set_record(&mut self, tag: &LevelTag, moves: usize) {
        match tag {
            LevelTag::Stock(index) => self.stock[*index] = moves,
            LevelTag::Custom(uuid) => {
                if let Some(record) = self.custom.get_mut(uuid) {
                    *record = moves;
                } else {
                    self.custom.insert(*uuid, moves);
                }
            }
            LevelTag::Test(_) => (),
        }
    }

    pub fn get_record(&self, tag: &LevelTag) -> usize {
        match tag {
            LevelTag::Stock(index) => self.stock[*index],
            LevelTag::Custom(uuid) => self.custom[uuid],
            LevelTag::Test(_) => 0,
        }
    }

    pub fn set_if_new_record(&mut self, tag: &LevelTag, moves: &usize) {
        let record = self.get_record(tag);
        if record == 0 || record > *moves {
            self.set_record(tag, *moves);
        }
    }

    pub fn unlock_next_stock_level(&mut self, tag: &LevelTag) {
        if let LevelTag::Stock(index) = tag {
            let next_index = index + 1;
            if next_index < MAX_TOTAL_LEVELS && next_index >= self.stock.len() {
                self.stock.push(0);
            }
        }
    }

    pub fn final_stock_levels_record(&self) -> usize {
        self.stock.iter().sum()
    }

    pub fn save(&self) {
        let data = SaveFileData::new(self.stock.clone(), self.custom.clone());
        let path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            PathBuf::from(manifest_dir).join("assets").join("game.dat")
        } else {
            PathBuf::from("./assets").join("game.dat")
        };
        if let Ok(serialized_string) = ron::ser::to_string(&data) {
            let mut file = File::create(path).unwrap();
            file.write_all(serialized_string.as_bytes()).unwrap();
        }
    }
}
