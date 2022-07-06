use bevy::prelude::*;

use crate::{
    resources::prelude::*,
    ui::{EmbossedText, Housing, Overlay},
};

#[derive(Component)]
pub struct UiMarker;

fn spawn_ui_camera(commands: &mut Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiMarker);
}

pub fn spawn_ui(commands: &mut Commands, _images: &Images, fonts: &Fonts) {
    let font = &fonts.upheavtt;

    let overlay = Overlay::new();

    let line_a = Housing::percent(100.0, 10.0);

    let title = EmbossedText::medium("Options", font);

    overlay.spawn(
        commands,
        |parent| {
            line_a.spawn(parent, |parent| {
                title.spawn(parent);
            });
        },
        UiMarker,
    );

    spawn_ui_camera(commands);
}
