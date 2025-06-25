use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct Images {
    #[asset(path = "images/entities/box.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub entity_box: Handle<Image>,
    #[asset(path = "images/entities/placed_box.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub entity_placed_box: Handle<Image>,
    #[asset(path = "images/entities/void.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub entity_void: Handle<Image>,
    #[asset(path = "images/entities/floor.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub entity_floor: Handle<Image>,
    #[asset(path = "images/entities/zone.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub entity_zone: Handle<Image>,
    #[asset(path = "images/brushes/box.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub brush_box: Handle<Image>,
    #[asset(path = "images/brushes/placed_box.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub brush_placed_box: Handle<Image>,
    #[asset(path = "images/brushes/void.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub brush_void: Handle<Image>,
    #[asset(path = "images/brushes/floor.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub brush_floor: Handle<Image>,
    #[asset(path = "images/brushes/zone.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub brush_zone: Handle<Image>,
    #[asset(path = "images/brushes/character.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub brush_character: Handle<Image>,
    #[asset(texture_atlas_layout(
        tile_size_x = 64,
        tile_size_y = 96,
        columns = 4,
        rows = 7,
        padding_x = 4,
        padding_y = 4
    ))]
    pub character_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "images/character/spritesheet.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub character: Handle<Image>,
    #[asset(path = "images/instructions.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub instructions: Handle<Image>,
}
