mod handle;

use std::{env, fs::File, io::Write, iter::Enumerate, path::PathBuf, slice::Iter, vec::IntoIter};

use bevy::{asset::LoadState, prelude::*, reflect::TypePath};
use hashbrown::HashMap;
use ron::ser as serialize_ron;
use serde::{Deserialize, Serialize};

pub use self::handle::SaveFileHandle;

use super::prelude::*;

#[derive(Asset, TypePath, Serialize, Deserialize, Clone, Resource)]
pub struct SaveFile {
    volume: f64,
    stock_records: Vec<LevelRecord>,
    custom_records: HashMap<String, LevelRecord>,
}

impl Default for SaveFile {
    fn default() -> SaveFile {
        SaveFile {
            volume: INITIAL_VOLUME,
            stock_records: vec![LevelRecord::default()],
            custom_records: HashMap::default(),
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

        let file = if matches!(load_state, Some(LoadState::Loaded)) {
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

    pub fn get_record(&self, kind: &LevelKind) -> LevelRecord {
        match kind {
            LevelKind::Stock(index) => self.stock_records[*index],
            LevelKind::Playtest(_) => LevelRecord::default(),
            LevelKind::Custom(payload) => *self
                .custom_records
                .get(payload)
                .expect("Cannot get custom record"),
            LevelKind::Editable => unreachable!("An editable level does not have a record"),
        }
    }

    pub fn set_new_record(&mut self, level: &Level) {
        let new_record = level.get_set_record();
        let current_record = self.get_record(&level.kind);
        if new_record.is_better_than(&current_record) {
            match &level.kind {
                LevelKind::Stock(index) => {
                    self.stock_records[*index] = new_record;
                }
                LevelKind::Custom(payload) => {
                    self.custom_records.insert(payload.clone(), new_record);
                }
                LevelKind::Playtest(_) => unreachable!("Cannot set a record for an playtest level"),
                LevelKind::Editable => {
                    unreachable!("Cannot set a record for an editable level")
                }
            };
        }
    }

    pub fn insert_custom_level_record(&mut self, key: String, level_record: LevelRecord) {
        self.custom_records.insert(key, level_record);
    }

    pub fn delete_custom_level_record(&mut self, key: &str) {
        self.custom_records.remove(key);
    }

    pub fn unlock_new_level(&mut self, level: &Level) {
        if let LevelKind::Stock(index) = level.kind {
            let unlocked_levels = self.unlocked_levels();
            if unlocked_levels == index + 1 && unlocked_levels < TOTAL_STOCK_LEVELS {
                self.stock_records.push(LevelRecord::default());
            }
        }
    }

    pub fn set_volume(&mut self, volume: f64) {
        self.volume = volume;
    }

    pub fn get_volume(&self) -> f64 {
        self.volume
    }

    pub fn unlocked_levels(&self) -> usize {
        self.stock_records.len()
    }

    pub fn number_custom_levels(&self) -> usize {
        self.custom_records.len()
    }

    pub fn enumerated_stock_records(&self) -> Enumerate<Iter<LevelRecord>> {
        self.stock_records.iter().enumerate()
    }

    pub fn ordered_custom_records(&self) -> Enumerate<IntoIter<(&String, &LevelRecord)>> {
        let mut x = self
            .custom_records
            .iter()
            .collect::<Vec<(&String, &LevelRecord)>>();
        x.sort_by(|(a_key, _), (b_key, _)| a_key.cmp(b_key));

        x.into_iter().enumerate()
    }
}
