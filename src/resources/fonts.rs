use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct Fonts {
    #[asset(path = "fonts/upheaval/upheaval.ttf")]
    upheaval: Handle<Font>,
}

impl Fonts {
    pub fn primary(&self) -> &Handle<Font> {
        &self.upheaval
    }
}
