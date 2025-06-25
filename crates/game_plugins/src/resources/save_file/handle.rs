use bevy::{asset::LoadState, prelude::*};

use super::SaveFile;

#[derive(Asset, TypePath, Clone, Resource)]
pub struct SaveFileHandle {
    pub value: Handle<SaveFile>,
}

impl SaveFileHandle {
    pub fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(SaveFileHandle {
            value: asset_server.load("game.dat"),
        });
    }

    pub fn check_loaded_or_failed(
        save_file_handle: Res<SaveFileHandle>,
        asset_server: Res<AssetServer>,
    ) -> bool {
        matches!(
            asset_server.get_load_state(save_file_handle.value.id()),
            Some(LoadState::Loaded | LoadState::Failed(_))
        )
    }
}
