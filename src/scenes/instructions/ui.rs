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

pub fn spawn_ui(commands: &mut Commands, fonts: &Fonts) {
    let font = &fonts.upheavtt;

    let overlay = Overlay::new();
    let housing_a = Housing::percent(100.0, 10.0);
    let housing_b = Housing::percent(100.0, 10.0);

    let how_to_play = EmbossedText::medium("How to Play", font);
    let press_button = EmbossedText::small("Press any button to continue", font);

    overlay.spawn(
        commands,
        |parent| {
            housing_a.spawn(parent, |parent| {
                how_to_play.spawn(parent);
            });
            housing_b.spawn(parent, |parent| {
                press_button.spawn(parent);
            });
        },
        UiMarker,
    );

    spawn_ui_camera(commands);
}
