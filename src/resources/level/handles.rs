use bevy::{asset::LoadState, prelude::*};

use super::state::LevelState;

pub struct LevelHandles {
    pub stock: Vec<Handle<LevelState>>,
}

impl LevelHandles {
    #[must_use]
    pub fn load_stock(asset_server: &Res<AssetServer>) -> LevelHandles {
        LevelHandles {
            stock: vec![
                asset_server.load("levels/stock/1.lvl"),
                asset_server.load("levels/stock/2.lvl"),
                asset_server.load("levels/stock/3.lvl"),
                asset_server.load("levels/stock/4.lvl"),
                asset_server.load("levels/stock/5.lvl"),
                asset_server.load("levels/stock/6.lvl"),
                asset_server.load("levels/stock/7.lvl"),
                asset_server.load("levels/stock/8.lvl"),
                asset_server.load("levels/stock/9.lvl"),
                asset_server.load("levels/stock/10.lvl"),
                asset_server.load("levels/stock/11.lvl"),
                asset_server.load("levels/stock/12.lvl"),
                asset_server.load("levels/stock/13.lvl"),
                asset_server.load("levels/stock/14.lvl"),
                asset_server.load("levels/stock/15.lvl"),
                asset_server.load("levels/stock/16.lvl"),
            ],
        }
    }

    #[must_use]
    pub fn all_stock_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        let stock_levels = self.stock.iter().map(bevy::prelude::Handle::clone_untyped);

        for level_handle in stock_levels {
            if asset_server.get_load_state(level_handle) != LoadState::Loaded {
                return false;
            }
        }

        true
    }
}
