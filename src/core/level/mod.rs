mod components;

pub mod entity;
pub mod position;
pub mod stock;
pub mod tag;

use bevy::prelude::*;

use crate::resources::prelude::*;

pub use components::{CameraMarker, OnTopMarker, PlayerMarker};

use super::BOX_ENTITY_OFFSET;

pub fn reload(level: &mut Level, levels: &LevelHandles, level_states: &Res<Assets<LevelState>>) {
    let state = tag::to_default_state(&level.tag, levels, level_states);
    level.set_state(state);
}

pub fn spawn(
    commands: &mut Commands,
    texture_atlases: &mut Assets<TextureAtlas>,
    level: &Level,
    images: &Images,
) {
    let position = level.get_player_position();
    let index = level.get_sprite_index();
    let texture = images.player.spritesheet.clone();
    spawn_entity(
        commands,
        texture_atlases,
        position,
        texture,
        PlayerMarker,
        true,
        true,
        index,
    );

    level.loop_over_entity_and_position(|entity, position| {
        let on_top = matches!(entity, &MapEntity::B | &MapEntity::P);
        let texture = entity::to_image(entity, images);
        spawn_entity(
            commands,
            texture_atlases,
            &position,
            texture,
            position,
            on_top,
            false,
            index,
        );
    });

    spawn_camera(commands);
}

pub fn spawn_entity(
    commands: &mut Commands,
    texture_atlases: &mut Assets<TextureAtlas>,
    position: &MapPosition,
    texture: Handle<Image>,
    component: impl Component,
    on_top: bool,
    is_player: bool,
    index: usize,
) {
    let mut translation = Vec3::default();
    position::update_entity_translation(position, &mut translation);

    if on_top {
        translation.y += BOX_ENTITY_OFFSET as f32;
        translation.z += 1.0;
    }

    if is_player {
        let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(64.0, 64.0), 4, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index,
                    ..TextureAtlasSprite::default()
                },
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_translation(translation),
                ..SpriteSheetBundle::default()
            })
            .insert(component);
    } else {
        commands
            .spawn_bundle(SpriteBundle {
                texture,
                transform: Transform::from_translation(translation),
                ..SpriteBundle::default()
            })
            .insert(component);
    }
}

fn spawn_camera(commands: &mut Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale *= 0.50;
    commands.spawn_bundle(camera_bundle).insert(CameraMarker);
}
