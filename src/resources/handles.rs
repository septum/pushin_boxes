use bevy::prelude::*;

use super::{assets::AssetsHandles, save_file::SaveFileHandle};

pub struct ResourcesHandles {
    pub assets: AssetsHandles,
    pub save_file: SaveFileHandle,
}

impl ResourcesHandles {
    pub fn load(asset_server: &Res<AssetServer>) -> ResourcesHandles {
        let assets = AssetsHandles::load(asset_server);
        let save_file = SaveFileHandle::load(asset_server);

        ResourcesHandles { assets, save_file }
    }

    pub fn all_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        let assets_loaded = self.assets.all_loaded(asset_server);
        let save_file_loaded = self.save_file.check_loaded_or_failed(asset_server);

        assets_loaded && save_file_loaded
    }
}
