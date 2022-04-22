mod colors;
mod fonts;
mod images;
mod levels;
mod sounds;

use bevy::prelude::*;

pub use colors::Colors;
pub use fonts::Fonts;
pub use images::Images;
pub use levels::Levels;
pub use sounds::Sounds;

pub struct AssetsHandles {
    pub fonts: Fonts,
    pub images: Images,
    pub levels: Levels,
    pub sounds: Sounds,
}

impl AssetsHandles {
    pub fn load(asset_server: &Res<AssetServer>) -> AssetsHandles {
        let fonts = Fonts::load(asset_server);
        let images = Images::load(asset_server);
        let sounds = Sounds::load(asset_server);
        let levels = Levels::load_stock(asset_server);

        AssetsHandles {
            fonts,
            images,
            sounds,
            levels,
        }
    }

    pub fn all_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        self.fonts.all_loaded(asset_server)
            && self.images.all_loaded(asset_server)
            && self.sounds.all_loaded(asset_server)
            && self.levels.all_loaded(asset_server)
    }
}
