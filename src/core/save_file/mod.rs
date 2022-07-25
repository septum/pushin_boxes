pub mod stock;

use bevy::{asset::LoadState, prelude::*};
use ron::ser as serialize_ron;
use std::{env, fs::File, io::Write, path::PathBuf};

use crate::resources::prelude::*;

/// # Panics
///
/// Will panic if file cannot be created in path
pub fn save(save_file: &SaveFile) {
    let path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(manifest_dir).join("assets").join("game.dat")
    } else {
        PathBuf::from("./assets").join("game.dat")
    };
    if let Ok(serialized_string) = serialize_ron::to_string(save_file) {
        let mut file = File::create(path).unwrap();
        file.write_all(serialized_string.as_bytes()).unwrap();
    }
}

#[must_use]
pub fn get_record(save_file: &SaveFile, tag: &LevelTag) -> (usize, f32) {
    match tag {
        LevelTag::Stock(index) => save_file.get_stock_level_record(index),
    }
}

pub fn set_record(save_file: &mut SaveFile, tag: &LevelTag, record: (usize, f32)) {
    match tag {
        LevelTag::Stock(index) => save_file.set_stock_level_record(index, record),
    };
}

pub fn set_if_new_record(save_file: &mut SaveFile, tag: &LevelTag, moves: usize, time: f32) {
    let record = get_record(save_file, tag);
    if record.0 == 0 || record.0 > moves || record.0 >= moves && record.1 > time {
        set_record(save_file, tag, (moves, time));
    }
}

#[must_use]
pub fn is_default(save_file: &SaveFile) -> bool {
    let default = SaveFile::default();
    default.stock.eq(&save_file.stock)
}

/// # Panics
///
/// Will panic if no save file asset is found
pub fn insert(
    commands: &mut Commands,
    handle: &Res<SaveFileHandle>,
    asset_server: &Res<AssetServer>,
    save_file: &Res<Assets<SaveFile>>,
) {
    let load_state = asset_server.get_load_state(handle.save_file.clone());

    let file = if matches!(load_state, LoadState::Loaded) {
        save_file.get(handle.save_file.clone()).unwrap().clone()
    } else {
        SaveFile::default()
    };

    commands.insert_resource(file);
}
