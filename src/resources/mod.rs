mod assets;
mod handles;
mod save_file;

use bevy::prelude::*;
use bevy_asset_ron::RonAssetPlugin;

use crate::{level::LevelState, state::GameState};

use save_file::SaveFileData;

pub use assets::{AssetsHandles, Colors, Images, Levels};
pub use handles::ResourcesHandles;
pub use save_file::SaveFile;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<SaveFileData>::new(&["dat"]))
            .add_plugin(RonAssetPlugin::<LevelState>::new(&["lvl"]))
            .add_system_set(SystemSet::on_enter(GameState::Startup).with_system(startup))
            .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_loading));
    }
}

fn startup(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(ResourcesHandles::load(&asset_server));
    state.set(GameState::Loading).unwrap();
}

fn check_loading(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    mut resources: ResMut<ResourcesHandles>,
    asset_server: Res<AssetServer>,
    save_file_data: Res<Assets<SaveFileData>>,
) {
    if resources.all_loaded(&asset_server) {
        resources
            .save_file
            .insert(&mut commands, &asset_server, &save_file_data);

        let save_file_handle = resources.save_file.clone();
        resources
            .assets
            .levels
            .load_custom(save_file_handle, &asset_server, &save_file_data);

        state.set(GameState::Title).unwrap();
    }
}
