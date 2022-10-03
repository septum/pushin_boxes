use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct Images {
    #[asset(path = "images/entities/box.png")]
    pub entity_box: Handle<Image>,
    #[asset(path = "images/entities/floor.png")]
    pub entity_floor: Handle<Image>,
    #[asset(path = "images/entities/zone.png")]
    pub entity_zone: Handle<Image>,
    #[asset(texture_atlas(
        tile_size_x = 64.0,
        tile_size_y = 64.0,
        columns = 4,
        rows = 7,
        padding_x = 4.,
        padding_y = 4.
    ))]
    #[asset(path = "images/player/spritesheet.png")]
    pub player_atlas: Handle<TextureAtlas>,
    #[asset(path = "images/instructions.png")]
    pub instructions: Handle<Image>,
}
