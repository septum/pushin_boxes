use bevy::prelude::*;

use crate::ui::Overlay;

#[derive(Component)]
pub struct UiMarker;

fn spawn_ui_camera(commands: &mut Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiMarker);
}

pub fn spawn_ui(commands: &mut Commands) {
    let overlay = Overlay::new();

    overlay.spawn(commands, |_parent| {}, UiMarker);

    spawn_ui_camera(commands);
}
