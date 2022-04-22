use bevy::{asset::LoadState, prelude::*};

use super::{data::SaveFileData, SaveFile};

#[derive(Clone)]
pub struct SaveFileHandle {
    pub save_file: Handle<SaveFileData>,
}

impl SaveFileHandle {
    pub fn load(asset_server: &Res<AssetServer>) -> SaveFileHandle {
        SaveFileHandle {
            save_file: asset_server.load("game.dat"),
        }
    }

    pub fn insert(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        save_file_data: &Res<Assets<SaveFileData>>,
    ) {
        let load_state = asset_server.get_load_state(self.save_file.clone());

        assert!(matches!(load_state, LoadState::Loaded | LoadState::Failed));

        let data = if matches!(load_state, LoadState::Loaded) {
            save_file_data.get(self.save_file.clone()).unwrap().clone()
        } else {
            SaveFileData::default()
        };

        commands.insert_resource(SaveFile::new(&data));
    }

    pub fn check_loaded_or_failed(&self, asset_server: &Res<AssetServer>) -> bool {
        let load_state = asset_server.get_load_state(self.save_file.clone());

        matches!(load_state, LoadState::Loaded | LoadState::Failed)
    }
}
