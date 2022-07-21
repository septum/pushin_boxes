use bevy::prelude::*;

use crate::{
    resources::prelude::*,
    ui::{DynamicText, EmbossedText, Housing, Overlay, TextMarker},
};

#[derive(Component)]
pub struct UiMarker;

fn spawn_ui_camera(commands: &mut Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiMarker);
}

pub fn spawn_ui(commands: &mut Commands, level: &Level, fonts: &Fonts) {
    let font = &fonts.upheavtt;
    let record_new_level = if level.is_record_set() {
        format!("Record: {}", level.record)
    } else {
        "New Level!".to_string()
    };

    let mut overlay = Overlay::new();
    let mut top = Housing::percent(98.0, 7.0);
    let mut bottom = Housing::percent(98.0, 8.0);

    let mut top_left = Housing::percent(50.0, 100.0);
    let mut top_right = Housing::percent(50.0, 100.0);
    let bottom_left = Housing::percent(50.0, 100.0);
    let bottom_right = Housing::percent(50.0, 100.0);

    let mut moves_housing = Housing::percent(100.0, 50.0);
    let mut record_housing = Housing::percent(100.0, 50.0);
    let mut reload_housing = Housing::percent(100.0, 50.0);
    let mut selection_housing = Housing::percent(100.0, 50.0);
    let mut undos_left_housing = Housing::percent(100.0, 50.0);
    let mut undo_housing = Housing::percent(100.0, 50.0);

    let mut level_name = EmbossedText::medium(format!("Level {}", level.get_name()), font);
    let mut moves = DynamicText::small("Moves: ", font);
    let mut record_new_level = EmbossedText::small(record_new_level, font);
    let mut undos_left = DynamicText::small("Undos: ", font);
    let undo = EmbossedText::small("(B) - Undo Movement", font);
    let reload = EmbossedText::small("(X) - Reload Level", font);
    let selection = EmbossedText::small("(L3) - Level Selection", font);

    moves.size(35.0);
    undos_left.size(35.0);

    level_name.foreground_color(Colors::LIGHT);
    record_new_level.foreground_color(Colors::SECONDARY);

    overlay.justify_content(JustifyContent::SpaceBetween);
    top.flex_direction(FlexDirection::Row);
    bottom.flex_direction(FlexDirection::Row);

    top_left.align_items(AlignItems::FlexStart);
    top_right.align_items(AlignItems::FlexEnd);
    moves_housing.align_items(AlignItems::FlexEnd);
    record_housing
        .align_items(AlignItems::FlexEnd)
        .position_type(PositionType::Absolute)
        .top(44.0);
    undo_housing.align_items(AlignItems::FlexEnd);
    reload_housing
        .align_items(AlignItems::FlexStart)
        .position_type(PositionType::Absolute)
        .top(-4.0);
    selection_housing.align_items(AlignItems::FlexStart);
    undos_left_housing
        .align_items(AlignItems::FlexEnd)
        .position_type(PositionType::Absolute)
        .top(-4.0);
    undo_housing.align_items(AlignItems::FlexEnd);

    overlay.spawn(
        commands,
        |parent| {
            top.spawn(parent, |parent| {
                top_left.spawn(parent, |parent| {
                    level_name.spawn(parent);
                });
                top_right.spawn(parent, |parent| {
                    moves_housing.spawn(parent, |parent| {
                        moves.spawn(parent, TextMarker::moves());
                    });
                    record_housing.spawn(parent, |parent| {
                        record_new_level.spawn(parent);
                    });
                });
            });
            bottom.spawn(parent, |parent| {
                bottom_left.spawn(parent, |parent| {
                    reload_housing.spawn(parent, |parent| {
                        reload.spawn(parent);
                    });
                    selection_housing.spawn(parent, |parent| {
                        selection.spawn(parent);
                    });
                });
                bottom_right.spawn(parent, |parent| {
                    undos_left_housing.spawn(parent, |parent| {
                        undos_left.spawn(parent, TextMarker::undos());
                    });
                    undo_housing.spawn(parent, |parent| {
                        undo.spawn(parent);
                    });
                });
            });
        },
        UiMarker,
    );

    spawn_ui_camera(commands);
}
