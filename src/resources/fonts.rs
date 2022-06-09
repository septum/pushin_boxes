use bevy::{asset::LoadState, prelude::*};

pub struct Fonts {
    pub fredoka: Handle<Font>,
    pub upheavtt: Handle<Font>,
}

impl Fonts {
    pub fn load(asset_server: &Res<AssetServer>) -> Fonts {
        Fonts {
            fredoka: asset_server.load("fonts/fredoka/FredokaOne-Regular.ttf"),
            upheavtt: asset_server.load("fonts/upheavtt/upheavtt.ttf"),
        }
    }

    pub fn all_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        asset_server.get_load_state(self.fredoka.clone()) == LoadState::Loaded
        && asset_server.get_load_state(self.upheavtt.clone()) == LoadState::Loaded
    }
}
