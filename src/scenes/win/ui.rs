use bevy::prelude::*;

use crate::{
    resources::prelude::*,
    ui::{EmbossedText, Housing, Overlay, SimpleText},
};

#[derive(Component)]
pub struct UiMarker;

fn spawn_ui_camera(commands: &mut Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiMarker);
}

pub fn spawn_ui(commands: &mut Commands, fonts: &Fonts, level: &Level) {
    let font = &fonts.upheavtt;

    let record_or_empty = if level.is_new_record() {
        format!("NEW RECORD: {}", level.moves)
    } else {
        " ".to_string()
    };

    let overlay = Overlay::new();
    let title_housing = Housing::new(Val::Percent(100.0), Val::Px(98.0));
    let press_button_housing = Housing::new(Val::Percent(100.0), Val::Px(32.0));

    let mut record_or_empty = SimpleText::medium(record_or_empty, font);
    let title = EmbossedText::big("You Win!   ", font);
    let press_button = EmbossedText::small("Press (any button) to continue", font);

    record_or_empty.color(Colors::SECONDARY);

    overlay.spawn(
        commands,
        |parent| {
            record_or_empty.spawn(parent);
            title_housing.spawn(parent, |parent| {
                title.spawn(parent);
            });
            press_button_housing.spawn(parent, |parent| {
                press_button.spawn(parent);
            });
        },
        UiMarker,
    );

    spawn_ui_camera(commands);
}
