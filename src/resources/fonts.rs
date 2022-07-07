use bevy::{asset::LoadState, prelude::*};

pub struct Fonts {
    pub upheavtt: Handle<Font>,
}

impl Fonts {
    #[must_use]
    pub fn load(asset_server: &Res<AssetServer>) -> Fonts {
        Fonts {
            upheavtt: asset_server.load("fonts/upheavtt/upheavtt.ttf"),
        }
    }

    #[must_use]
    pub fn all_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        asset_server.get_load_state(self.upheavtt.clone()) == LoadState::Loaded
    }
}
