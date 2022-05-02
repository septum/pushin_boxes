use bevy::{asset::LoadState, prelude::*};

use super::SaveFile;

#[derive(Clone)]
pub struct SaveFileHandle {
    pub save_file: Handle<SaveFile>,
}

impl SaveFileHandle {
    pub fn load(asset_server: &Res<AssetServer>) -> SaveFileHandle {
        SaveFileHandle {
            save_file: asset_server.load("game.dat"),
        }
    }

    pub fn check_loaded_or_failed(&self, asset_server: &Res<AssetServer>) -> bool {
        let load_state = asset_server.get_load_state(self.save_file.clone());

        matches!(load_state, LoadState::Loaded | LoadState::Failed)
    }
}
