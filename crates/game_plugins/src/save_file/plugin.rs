use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_common_assets::ron::RonAssetPlugin;

use uuid::Uuid;

use crate::{
    level::LevelHandles,
    save_file::{SaveFile, handle::SaveFileHandle},
    state::GameState,
};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RonAssetPlugin::<SaveFile>::new(&["dat"]),))
            .add_systems(OnEnter(GameState::Loading), SaveFileHandle::load)
            .add_systems(
                Update,
                SaveFile::insert
                    .run_if(SaveFileHandle::check_loaded_or_failed)
                    .run_if(in_state(GameState::Loading)),
            )
            .add_systems(
                OnExit(GameState::Loading),
                (insert_custom_level_handles.run_if(resource_added::<SaveFile>),),
            );
    }
}

fn insert_custom_level_handles(
    save_file: Res<SaveFile>,
    mut level_handles: ResMut<LevelHandles>,
    asset_server: Res<AssetServer>,
) {
    for (_, (key, _)) in save_file.ordered_custom_records() {
        let split_key: Vec<&str> = key.split('$').collect();
        let uuid = Uuid::parse_str(split_key[1]).expect("Cannot parse uuid");
        let path = format!("levels/custom/{}.lvl", &split_key[1]);
        level_handles.insert_custom(uuid, asset_server.load(path));
    }
}
