use bevy::{asset::LoadState, prelude::*};
use hashbrown::HashMap;
use uuid::Uuid;

use crate::{
    level::LevelState,
    resources::save_file::{SaveFileData, SaveFileHandle},
};

pub struct Levels {
    pub stock: Vec<Handle<LevelState>>,
    pub custom: HashMap<Uuid, Handle<LevelState>>,
}

impl Levels {
    pub fn load_stock(asset_server: &Res<AssetServer>) -> Levels {
        Levels {
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
            custom: HashMap::new(),
        }
    }

    pub fn load_custom(
        &mut self,
        save_file_handle: SaveFileHandle,
        asset_server: &Res<AssetServer>,
        save_file_data: &Res<Assets<SaveFileData>>,
    ) {
        let mut custom = HashMap::new();
        let load_state = asset_server.get_load_state(save_file_handle.save_file.clone());

        assert!(matches!(load_state, LoadState::Loaded | LoadState::Failed));

        let data = if matches!(load_state, LoadState::Loaded) {
            save_file_data
                .get(save_file_handle.save_file.clone())
                .unwrap()
                .clone()
        } else {
            SaveFileData::default()
        };

        for (uuid, _) in data.custom {
            custom.insert(
                uuid,
                asset_server.load(&format!("levels/custom/{}.lvl", uuid)),
            );
        }

        self.custom = custom;
    }

    pub fn all_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        let stock_levels = self.stock.iter().map(|level| level.clone_untyped());
        let custom_levels = self.custom.iter().map(|(_, level)| level.clone_untyped());

        for level_handle in stock_levels {
            if asset_server.get_load_state(level_handle) != LoadState::Loaded {
                return false;
            }
        }

        for level_handle in custom_levels {
            if asset_server.get_load_state(level_handle) != LoadState::Loaded {
                return false;
            }
        }

        true
    }
}
