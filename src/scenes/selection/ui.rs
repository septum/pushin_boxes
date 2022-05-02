use bevy::prelude::*;

use crate::{
    game,
    resources::prelude::*,
    state::Selection,
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
    for (index, record) in save_file.stock.iter().enumerate() {
        let housing = Housing::percent(25.0, 25.0);
        let button = ActionButton::square(format!("{}", index + 1), font);
        let marker = ButtonMarker::stock_level(index);
        let text = if *record > 0 {
            format!("Record: {}", record)
        } else {
            "New Level!".to_string()
        };
        let record_new_level = SimpleText::small(text, font);

        housing.spawn(parent, |parent| {
            button.spawn(parent, marker);
            record_new_level.spawn(parent);
        });
    }
}

fn spawn_custom_buttons(parent: &mut ChildBuilder, save_file: &SaveFile, font: &Handle<Font>) {
    for (uuid, record) in save_file.custom.iter() {
        let housing = Housing::percent(100.0, 20.0);
        let button = ActionButton::full(format!("{}", uuid), font);
        let marker = ButtonMarker::custom_level(*uuid);
        let text = if *record > 0 {
            format!("Record: {}", record)
        } else {
            "New Level!".to_string()
        };
        let record_new_level = SimpleText::small(text, font);

        housing.spawn(parent, |parent| {
            button.spawn(parent, marker);
            record_new_level.spawn(parent);
        });
    }
}

pub fn spawn_ui(
    commands: &mut Commands,
    fonts: &Fonts,
    save_file: &SaveFile,
    selection_kind: &Selection,
) {
    let font = &fonts.fredoka;
    let is_stock = game::scenes::is_stock_selection(selection_kind);
    let levels_simple_text = if is_stock {
        "Custom Levels"
    } else {
        "Stock Levels"
    };

    let overlay = Overlay::new();
    let mut top = Housing::percent(100.0, 10.0);
    let mut bottom = Housing::percent(100.0, 90.0);

    let title = EmbossedText::medium("Select a Level", font);
    let levels = ActionButton::full(levels_simple_text, font);

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
                levels.spawn(parent, ButtonMarker::levels());
            });
            bottom.spawn(parent, |parent| {
                if is_stock {
                    spawn_stock_buttons(parent, save_file, font);
                } else {
                    spawn_custom_buttons(parent, save_file, font);
                }
            });
        },
        UiMarker,
    );

    spawn_ui_camera(commands);
}
