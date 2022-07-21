use bevy::prelude::*;

use crate::{
    core,
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

pub fn spawn_ui(commands: &mut Commands, fonts: &Fonts, level: &Level, save_file: &SaveFile) {
    let font = &fonts.upheavtt;
    let is_last_level = core::level::stock::is_last(&level.tag);
    let title_housing_height = if is_last_level {
        Val::Px(196.0)
    } else {
        Val::Px(98.0)
    };

    let final_or_new = if is_last_level {
        format!("FINAL RECORD: {}", core::save_file::stock::total(save_file))
    } else if level.is_new_record() {
        format!("NEW RECORD: {}", level.moves)
    } else {
        String::new()
    };
    let title = if is_last_level {
        "Thank you\nfor playing!"
    } else {
        "You Win!"
    };

    let overlay = Overlay::new();
    let title_housing = Housing::new(Val::Percent(100.0), title_housing_height);
    let press_button_housing = Housing::new(Val::Percent(100.0), Val::Px(32.0));

    let mut final_or_new = SimpleText::medium(final_or_new, font);
    let title = EmbossedText::big(title, font);
    let press_button = EmbossedText::small("Press (any button) to continue", font);

    final_or_new.color(Colors::SECONDARY);

    overlay.spawn(
        commands,
        |parent| {
            final_or_new.spawn(parent);
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
