mod handle;

use bevy::{asset::LoadState, prelude::*, reflect::TypeUuid};
use ron::ser as serialize_ron;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, io::Write, path::PathBuf};

pub use handle::SaveFileHandle;

use super::{
    level::{LevelRecord, TOTAL_STOCK_LEVELS},
    prelude::*,
};

#[derive(TypeUuid, Serialize, Deserialize, Clone)]
#[uuid = "2e5bbfc2-8dfd-4547-8c85-cbaf27533998"]
pub struct SaveFile {
    pub stock: Vec<LevelRecord>,
}

impl Default for SaveFile {
    fn default() -> SaveFile {
        SaveFile {
            stock: vec![LevelRecord::default()],
        }
    }
}

impl SaveFile {
    pub fn new(stock: Vec<LevelRecord>) -> SaveFile {
        SaveFile { stock }
    }

    pub fn stock_levels_len(&self) -> usize {
        self.stock.len()
    }

    pub fn insert_stock_level_record(&mut self, record: LevelRecord) {
        self.stock.push(record);
    }

    pub fn set_stock_level_record(&mut self, index: &usize, record: LevelRecord) {
        self.stock[*index] = record;
    }

    pub fn get_stock_level_record(&self, index: &usize) -> LevelRecord {
        self.stock[*index]
    }

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

    pub fn insert_level(
        &self,
        commands: &mut Commands,
        index: usize,
        level_handles: &LevelHandles,
        level_states_assets: &Assets<LevelState>,
    ) {
        let tag = LevelTag::Stock(index);
        let state = *level_states_assets
            .get(&level_handles.stock[index])
            .unwrap();
        let record = self.get_stock_level_record(&index);
        let level = Level::new(tag, state, record);

        commands.insert_resource(level);
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
            LevelTag::Stock(index) => self.get_stock_level_record(index),
        }
    }

    pub fn set_record(&mut self, tag: &LevelTag, record: LevelRecord) {
        match tag {
            LevelTag::Stock(index) => self.set_stock_level_record(index, record),
        };
    }

    pub fn set_if_new_record(&mut self, level: &Level) {
        let new_record = level.get_record_set();
        if level.record.is_new_record(&new_record) {
            self.set_record(&level.tag, new_record);
        }
    }

    pub fn unlock_if_new_level(&mut self, level: &Level) {
        let LevelTag::Stock(index) = level.tag;
        if self.stock_levels_len() == index + 1 && self.stock_levels_len() < TOTAL_STOCK_LEVELS {
            self.insert_stock_level_record(default());
        }
    }
}
