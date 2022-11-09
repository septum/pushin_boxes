use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::state::LevelState;

#[derive(AssetCollection)]
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
    stock: Vec<Handle<LevelState>>,
}

impl LevelHandles {
    pub fn get(&self, index: &usize) -> &Handle<LevelState> {
        &self.stock[*index]
    }
}
