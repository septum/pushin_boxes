use bevy::prelude::*;

use crate::{
    resources::prelude::*,
    ui::{DynamicText, Housing, Overlay, SimpleText, TextMarker},
};

#[derive(Component)]
pub struct UiMarker;

fn spawn_ui_camera(commands: &mut Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiMarker);
}

pub fn spawn_ui(commands: &mut Commands, level: &Level, fonts: &Fonts) {
    let font = &fonts.fredoka;
    let record_new_level = if level.is_record_set() {
        format!("Record: {}", level.record)
    } else {
        "New Level!".to_string()
    };

    let mut overlay = Overlay::new();
    let mut top = Housing::percent(97.0, 10.0);
    let mut top_right = Housing::percent(50.0, 100.0);
    let mut bottom = Housing::percent(97.0, 10.0);
    let mut bottom_left = Housing::percent(50.0, 100.0);
    let mut bottom_right = Housing::percent(50.0, 100.0);

    let mut level_number = SimpleText::medium(format!("Level {}", level.get_name()), font);
    let mut moves = DynamicText::small("Moves: ", font);
    let mut record_new_level = SimpleText::small(record_new_level, font);
    let mut undos_left = DynamicText::small("Undos: ", font);
    let mut undo = SimpleText::small("[U] - Undo Movement", font);
    let mut reload = SimpleText::small("[R] - Reload Level", font);
    let mut selection = SimpleText::small("[L] - Level Selection", font);

    if level.is_custom() {
        level_number.size(24.0);
    }

    moves.size(28.0);
    undos_left.size(28.0);
    undo.color(Colors::PRIMARY).top_position(-4.0);
    reload.color(Colors::PRIMARY);
    selection.color(Colors::PRIMARY);
    record_new_level.color(Colors::SECONDARY).top_position(-4.0);

    overlay.justify_content(JustifyContent::SpaceBetween);
    top_right.align_items(AlignItems::FlexEnd);
    bottom_left.align_items(AlignItems::FlexStart);
    bottom_right.align_items(AlignItems::FlexEnd);
    top.flex_direction(FlexDirection::Row)
        .justify_content(JustifyContent::SpaceBetween);
    bottom
        .flex_direction(FlexDirection::Row)
        .justify_content(JustifyContent::SpaceBetween);

    overlay.spawn(
        commands,
        |parent| {
            top.spawn(parent, |parent| {
                level_number.spawn(parent);
                top_right.spawn(parent, |parent| {
                    moves.spawn(parent, TextMarker::moves());
                    record_new_level.spawn(parent);
                });
            });
            bottom.spawn(parent, |parent| {
                bottom_left.spawn(parent, |parent| {
                    reload.spawn(parent);
                    selection.spawn(parent);
                });
                bottom_right.spawn(parent, |parent| {
                    undos_left.spawn(parent, TextMarker::undos());
                    undo.spawn(parent);
                });
            });
        },
        UiMarker,
    );

    spawn_ui_camera(commands);
}
