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

    let record_or_empty = if level.new_record() {
        let duration = level.stopwatch_elapsed();
        let milliseconds = duration.subsec_millis();
        let seconds = duration.as_secs() % 60;
        let minutes = (duration.as_secs() / 60) % 60;
        let time = format!("{:02}:{:02}:{:03}", minutes, seconds, milliseconds);

        format!("NEW RECORD:\n{} moves in {}", level.moves, time)
    } else {
        "".to_string()
    };

    let overlay = Overlay::new();
    let center = Housing::percent(100.0, 100.0);
    let mut floating = Housing::percent(100.0, 20.0);
    let title_housing = Housing::new(Val::Percent(100.0), Val::Px(98.0));
    let press_button_housing = Housing::new(Val::Percent(100.0), Val::Px(32.0));

    let mut record_or_empty = SimpleText::medium(record_or_empty, font);
    let mut title = EmbossedText::big("You Win!    ", font);
    let press_button = EmbossedText::small("Press (any button) to continue", font);

    title.size(90.0);

    record_or_empty.color(Colors::SECONDARY);

    floating
        .position_type(PositionType::Absolute)
        .top(240.0)
        .justify_content(JustifyContent::FlexEnd);

    overlay.spawn(
        commands,
        |parent| {
            floating.spawn(parent, |parent| {
                record_or_empty.spawn(parent);
            });
            center.spawn(parent, |parent| {
                title_housing.spawn(parent, |parent| {
                    title.spawn(parent);
                });
                press_button_housing.spawn(parent, |parent| {
                    press_button.spawn(parent);
                });
            });
        },
        UiMarker,
    );

    spawn_ui_camera(commands);
}
