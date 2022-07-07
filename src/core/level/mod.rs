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

pub fn spawn(commands: &mut Commands, level: &Level, images: &Images) {
    let position = level.get_player_position();
    let texture = images.player.idle.clone();
    spawn_entity(commands, position, texture, PlayerMarker, true);

    level.loop_over_entity_and_position(|entity, position| {
        let on_top = matches!(entity, &MapEntity::B | &MapEntity::P);
        let texture = entity::to_image(entity, images);
        spawn_entity(commands, &position, texture, position, on_top);
    });

    spawn_camera(commands);
}

pub fn spawn_entity(
    commands: &mut Commands,
    position: &MapPosition,
    texture: Handle<Image>,
    component: impl Component,
    on_top: bool,
) {
    let mut translation = Vec3::default();
    position::update_entity_translation(position, &mut translation);

    if on_top {
        translation.y += BOX_ENTITY_OFFSET as f32;
        translation.z += 1.0;
    }

    commands
        .spawn_bundle(SpriteBundle {
            texture,
            transform: Transform::from_translation(translation),
            ..Default::default()
        })
        .insert(component);
}

fn spawn_camera(commands: &mut Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale *= 0.50;
    commands.spawn_bundle(camera_bundle).insert(CameraMarker);
}
