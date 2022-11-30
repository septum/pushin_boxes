mod handle;

use std::{env, fs::File, io::Write, iter::Enumerate, path::PathBuf, slice::Iter};

use bevy::{asset::LoadState, prelude::*, reflect::TypeUuid};
use ron::ser as serialize_ron;
use serde::{Deserialize, Serialize};

pub use self::handle::SaveFileHandle;

use super::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone)]
#[uuid = "2e5bbfc2-8dfd-4547-8c85-cbaf27533998"]
pub struct SaveFile {
    stock_records: Vec<LevelRecord>,
}

impl Default for SaveFile {
    fn default() -> SaveFile {
        SaveFile {
            stock_records: vec![LevelRecord::default()],
        }
    }
}

impl SaveFile {
    pub fn insert(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        save_file_handle: Res<SaveFileHandle>,
        save_file: Res<Assets<SaveFile>>,
    ) {
        let load_state = asset_server.get_load_state(save_file_handle.value.clone());

        let file = if matches!(load_state, LoadState::Loaded) {
            save_file.get(&save_file_handle.value).unwrap().clone()
        } else {
            SaveFile::default()
        };

        commands.insert_resource(file);
    }

    pub fn save(&self) {
        let path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            PathBuf::from(manifest_dir).join("assets").join("game.dat")
        } else {
            PathBuf::from("./assets").join("game.dat")
        };

        if let Ok(serialized_string) = serialize_ron::to_string(self) {
            let mut file = File::create(path).unwrap();
            file.write_all(serialized_string.as_bytes()).unwrap();
        }
    }

    pub fn get_record(&self, tag: &LevelTag) -> LevelRecord {
        match tag {
            LevelTag::Stock(index) => self.stock_records[*index],
            LevelTag::Playtest(_) => LevelRecord::default(),
            LevelTag::Editable => unreachable!("An editable level does not have a record"),
        }
    }

    pub fn set_new_record(&mut self, level: &Level) {
        let new_record = level.get_set_record();
        let current_record = self.get_record(&level.tag);
        if new_record.is_better_than(&current_record) {
            match level.tag {
                LevelTag::Stock(index) => {
                    self.stock_records[index] = new_record;
                }
                LevelTag::Playtest(_) => unreachable!("Cannot set a record for an playtest level"),
                LevelTag::Editable => {
                    unreachable!("Cannot set a record for an editable level")
                }
            };
        }
    }

    pub fn unlock_new_level(&mut self, level: &Level) {
        match level.tag {
            LevelTag::Stock(index) => {
                let unlocked_levels = self.unlocked_levels();
                if unlocked_levels == index + 1 && unlocked_levels < TOTAL_STOCK_LEVELS {
                    self.stock_records.push(LevelRecord::default());
                }
            }
            _ => {
                unreachable!("Cannot unlock a level for a non-stock level")
            }
        }
    }

    pub fn unlocked_levels(&self) -> usize {
        self.stock_records.len()
    }

    pub fn enumerated_stock_records(&self) -> Enumerate<Iter<LevelRecord>> {
        self.stock_records.iter().enumerate()
    }
}
