pub mod stock;

use bevy::{asset::LoadState, prelude::*};
use ron::ser as serialize_ron;
use std::{env, fs::File, io::Write, path::PathBuf};

use crate::resources::prelude::*;

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

pub fn get_record(save_file: &SaveFile, tag: &LevelTag) -> usize {
    match tag {
        LevelTag::Stock(index) => save_file.get_stock_level_record(index),
        LevelTag::Custom(uuid) => save_file.get_custom_level_record(uuid),
        LevelTag::Test(_) => 0,
    }
}

pub fn set_record(save_file: &mut SaveFile, tag: &LevelTag, moves: usize) {
    match tag {
        LevelTag::Stock(index) => save_file.set_stock_level_record(index, moves),
        LevelTag::Custom(uuid) => save_file.set_custom_level_record(uuid, moves),
        _ => (),
    };
}

pub fn set_if_new_record(save_file: &mut SaveFile, tag: &LevelTag, moves: usize) {
    let record = get_record(save_file, tag);
    if record == 0 || record > moves {
        set_record(save_file, tag, moves);
    }
}

pub fn is_default(save_file: &SaveFile) -> bool {
    let default = SaveFile::default();
    default.stock.eq(&save_file.stock) && default.custom.eq(&save_file.custom)
}

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
