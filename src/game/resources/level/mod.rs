mod components;

pub mod custom;
pub mod entity;
pub mod position;
pub mod stock;
pub mod tag;

use bevy::prelude::*;

use crate::resources::prelude::*;

pub use components::{CameraMarker, PlayerMarker};

pub fn reload(level: &mut Level, levels: &LevelHandles, level_states: &Res<Assets<LevelState>>) {
    let state = tag::to_default_state(&level.tag, levels, level_states);
    level.set_state(state);
}

pub fn spawn(commands: &mut Commands, level: &Level, images: &Images) {
    let position = level.get_player_position();
    let texture = images.player.idle.clone();
    spawn_entity(commands, position, texture, PlayerMarker);

    level.loop_over_entity_and_position(|entity, position| {
        let texture = entity::to_image(entity, images);
        spawn_entity(commands, &position, texture, position);
    });

    spawn_camera(commands);
}

fn spawn_entity(
    commands: &mut Commands,
    position: &MapPosition,
    texture: Handle<Image>,
    component: impl Component,
) {
    let mut translation = Vec3::default();
    position::update_entity_translation(position, &mut translation);

    commands
        .spawn_bundle(SpriteBundle {
            texture,
            transform: Transform::from_translation(translation),
            ..Default::default()
        })
        .insert(component);
}

// TODO: Move this
fn spawn_camera(commands: &mut Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(CameraMarker);
}
