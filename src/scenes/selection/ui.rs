use std::time::Duration;

use bevy::prelude::*;

use crate::{
    resources::prelude::*,
    ui::{ActionButton, ButtonMarker, EmbossedText, Housing, Overlay, SimpleText},
};

#[derive(Component)]
pub struct UiMarker;

fn spawn_ui_camera(commands: &mut Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiMarker);
}

fn spawn_stock_buttons(parent: &mut ChildBuilder, save_file: &SaveFile, font: &Handle<Font>) {
    let last_index = save_file.stock_levels_len();
    for (index, record) in save_file.stock.iter().enumerate() {
        let (text, color) = if record.0 > 0 {
            let duration = Duration::from_secs_f32(record.1);
            let milliseconds = duration.subsec_millis();
            let seconds = duration.as_secs() % 60;
            let minutes = (duration.as_secs() / 60) % 60;
            let time = format!("{:02}:{:02}:{:03}", minutes, seconds, milliseconds);
            (
                format!("Record: {}\nTime: {}", record.0, time),
                Colors::LIGHT,
            )
        } else {
            ("New Level!\n ".to_string(), Colors::SECONDARY)
        };

        let housing = Housing::percent(25.0, 25.0);
        let button = ActionButton::square(format!("{}", index + 1), font);
        let marker = ButtonMarker::stock_level(index, last_index == index + 1);
        let mut record_new_level = SimpleText::small(text, font);

        record_new_level.color(color);

        housing.spawn(parent, |parent| {
            button.spawn(parent, marker);
            record_new_level.spawn(parent);
        });
    }
}

pub fn spawn_ui(commands: &mut Commands, fonts: &Fonts, save_file: &SaveFile) {
    let font = &fonts.upheavtt;
    let overlay = Overlay::new();
    let mut top = Housing::percent(100.0, 10.0);
    let mut bottom = Housing::percent(100.0, 90.0);

    let title = EmbossedText::medium("Select a Level", font);

    top.flex_direction(FlexDirection::Row)
        .justify_content(JustifyContent::SpaceBetween)
        .left_padding(Val::Px(43.0))
        .right_padding(Val::Px(43.0));

    bottom
        .flex_wrap(FlexWrap::WrapReverse)
        .flex_direction(FlexDirection::Row)
        .justify_content(JustifyContent::FlexStart)
        .align_items(AlignItems::FlexStart)
        .align_content(AlignContent::FlexStart);

    overlay.spawn(
        commands,
        |parent| {
            top.spawn(parent, |parent| {
                title.spawn(parent);
            });
            bottom.spawn(parent, |parent| {
                spawn_stock_buttons(parent, save_file, font);
            });
        },
        UiMarker,
    );

    spawn_ui_camera(commands);
}
