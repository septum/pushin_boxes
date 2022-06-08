use bevy::{asset::LoadState, prelude::*};
use hashbrown::HashMap;
use std::{
    env,
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};
use uuid::Uuid;

use crate::{game::save_file, resources::prelude::*};

pub fn insert(
    commands: &mut Commands,
    uuid: &Uuid,
    save_file: &SaveFile,
    level_handles: &LevelHandles,
    level_states_assets: &Assets<LevelState>,
) {
    let tag = LevelTag::Custom(*uuid);
    let handle = level_handles.custom.get(uuid).unwrap().clone();
    let state = *level_states_assets.get(handle).unwrap();
    let record = save_file.get_custom_level_record(uuid);
    let level = Level::new(tag, state, record);

    commands.insert_resource(level);
}

pub fn write(
    level: &Level,
    level_handles: &mut LevelHandles,
    level_state: &LevelState,
    asset_server: &AssetServer,
    save_file: &mut SaveFile,
) {
    let uuid = Uuid::new_v4();
    let serialized_string = ron::ser::to_string(&level_state).unwrap();
    let levels_path = format!("levels/custom/{}.lvl", &uuid);
    let assets_path = format!("assets/{}", &levels_path);
    let path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(manifest_dir).join(assets_path)
    } else {
        PathBuf::from(assets_path)
    };

    let parent_path = path.parent().unwrap();
    create_dir_all(parent_path).unwrap();

    let mut file = File::create(path).unwrap();
    file.write_all(serialized_string.as_bytes()).unwrap();

    level_handles
        .custom
        .insert(uuid, asset_server.load(&levels_path));

    save_file.insert_custom_level_record(uuid, level.moves);

    save_file::save(save_file);
}

pub fn load_all(
    level_handles: &mut LevelHandles,
    save_file_handle: &SaveFileHandle,
    asset_server: &Res<AssetServer>,
    save_file: &Res<Assets<SaveFile>>,
) {
    let mut custom = HashMap::new();
    let load_state = asset_server.get_load_state(save_file_handle.save_file.clone());

    let data = if matches!(load_state, LoadState::Loaded) {
        save_file
            .get(save_file_handle.save_file.clone())
            .unwrap()
            .clone()
    } else {
        SaveFile::default()
    };

    for (uuid, _) in data.custom {
        let path = format!("levels/custom/{}.lvl", &uuid);
        custom.insert(uuid, asset_server.load(&path));
    }

    level_handles.custom = custom;
}
