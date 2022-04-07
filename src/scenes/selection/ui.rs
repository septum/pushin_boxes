use bevy::prelude::*;

use crate::{assets::GameAssets, ui};

use super::CleanupMarker;

fn spawn_background(commands: &mut Commands, assets: &GameAssets) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.images.background.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(CleanupMarker);
}

pub fn spawn(commands: &mut Commands, assets: &GameAssets) {
    let overlay = ui::Overlay::new();
    spawn_background(commands, assets);
    overlay.spawn(commands, CleanupMarker, |_| {});
}
