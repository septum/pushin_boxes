use bevy::prelude::*;

use crate::{
    resources::prelude::*,
    ui::{Overlay, SimpleText},
};

#[derive(Component)]
pub struct UiMarker;

fn spawn_ui_camera(commands: &mut Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiMarker);
}

pub fn spawn_ui(commands: &mut Commands, fonts: &Fonts) {
    let overlay = Overlay::new();
    let mut loading_text = SimpleText::big("Loading...", &fonts.fredoka);

    loading_text.color(Colors::PRIMARY);

    overlay.spawn(
        commands,
        |parent| {
            loading_text.spawn(parent);
        },
        UiMarker,
    );

    spawn_ui_camera(commands);
}
