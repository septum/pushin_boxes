use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use game_core::level::LevelState;

#[derive(Asset, TypePath, Serialize, Deserialize, Deref, DerefMut)]
pub struct LevelStateAsset(LevelState);

impl LevelStateAsset {
    pub fn new(state: LevelState) -> LevelStateAsset {
        LevelStateAsset(state)
    }
}

#[derive(AssetCollection, Resource)]
pub struct LevelHandles {
    #[asset(
        paths(
            "levels/stock/1.lvl",
            "levels/stock/2.lvl",
            "levels/stock/3.lvl",
            "levels/stock/4.lvl",
            "levels/stock/5.lvl",
            "levels/stock/6.lvl",
            "levels/stock/7.lvl",
            "levels/stock/8.lvl",
            "levels/stock/9.lvl",
            "levels/stock/10.lvl",
            "levels/stock/11.lvl",
            "levels/stock/12.lvl",
            "levels/stock/13.lvl",
            "levels/stock/14.lvl",
            "levels/stock/15.lvl",
            "levels/stock/16.lvl",
        ),
        collection(typed)
    )]
    stock: Vec<Handle<LevelStateAsset>>,
    custom: HashMap<Uuid, Handle<LevelStateAsset>>,
}

impl LevelHandles {
    pub fn get_stock(&self, index: usize) -> &Handle<LevelStateAsset> {
        &self.stock[index]
    }

    pub fn get_custom(&self, uuid: &Uuid) -> Option<&Handle<LevelStateAsset>> {
        self.custom.get(uuid)
    }

    pub fn insert_custom(&mut self, uuid: Uuid, handle: Handle<LevelStateAsset>) {
        self.custom.insert(uuid, handle);
    }
}
