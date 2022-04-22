use bevy::prelude::*;

use crate::{
    level::{Level, LevelTag},
    resources::{AssetsHandles, Colors},
    ui,
};

use super::{CleanupMarker, CounterKind, CounterMarker};

pub fn spawn(commands: &mut Commands, assets: &AssetsHandles, level: &Level) {
    let mut overlay = ui::Overlay::new();

    let mut top = ui::Housing::new(Val::Percent(97.0), Val::Percent(10.0));
    let mut top_right = ui::Housing::new(Val::Percent(50.0), Val::Percent(100.0));
    let mut bottom = ui::Housing::new(Val::Percent(97.0), Val::Percent(10.0));
    let mut bottom_left = ui::Housing::new(Val::Percent(50.0), Val::Percent(100.0));
    let mut bottom_right = ui::Housing::new(Val::Percent(50.0), Val::Percent(100.0));

    let level_id = match level.tag {
        LevelTag::Stock(index) => (index + 1).to_string(),
        LevelTag::Custom(uuid) => uuid.to_string(),
        LevelTag::Test(_) => "Test".to_string(),
    };

    let level_number = ui::SimpleText::new(
        format!("Level {}", level_id),
        TextStyle {
            font: assets.fonts.fredoka.clone(),
            font_size: if matches!(level.tag, LevelTag::Custom(_)) {
                24.0
            } else {
                42.0
            },
            color: Colors::LIGHT,
        },
    );
    let moves = ui::DynamicText::new(
        "Moves: ".to_string(),
        String::new(),
        TextStyle {
            font: assets.fonts.fredoka.clone(),
            font_size: 28.0,
            color: Colors::LIGHT,
        },
    );
    let mut record_or_new_level = ui::SimpleText::new(
        if level.record > 0 {
            format!("Record: {}", level.record)
        } else {
            "New Level!".to_string()
        },
        TextStyle {
            font: assets.fonts.fredoka.clone(),
            font_size: 19.0,
            color: Colors::SECONDARY,
        },
    );
    let reload = ui::SimpleText::new(
        "[R] - Reload Level".to_string(),
        TextStyle {
            font: assets.fonts.fredoka.clone(),
            font_size: 19.0,
            color: Colors::PRIMARY,
        },
    );
    let selection = ui::SimpleText::new(
        "[L] - Level Selection".to_string(),
        TextStyle {
            font: assets.fonts.fredoka.clone(),
            font_size: 19.0,
            color: Colors::PRIMARY,
        },
    );
    let undos_left = ui::DynamicText::new(
        "Undos: ".to_string(),
        String::new(),
        TextStyle {
            font: assets.fonts.fredoka.clone(),
            font_size: 28.0,
            color: Colors::LIGHT,
        },
    );
    let mut undo = ui::SimpleText::new(
        "[U] - Undo Movement".to_string(),
        TextStyle {
            font: assets.fonts.fredoka.clone(),
            font_size: 19.0,
            color: Colors::PRIMARY,
        },
    );

    overlay.set_justify_content(JustifyContent::SpaceBetween);

    top.set_flex_direction(FlexDirection::Row);
    top.set_justify_content(JustifyContent::SpaceBetween);
    top_right.set_align_items(AlignItems::FlexEnd);

    bottom.set_flex_direction(FlexDirection::Row);
    bottom.set_justify_content(JustifyContent::SpaceBetween);
    bottom_left.set_align_items(AlignItems::FlexStart);
    bottom_right.set_align_items(AlignItems::FlexEnd);

    record_or_new_level.bundle.style.position = Rect {
        top: Val::Px(-4.0),
        ..Default::default()
    };

    undo.bundle.style.position = Rect {
        top: Val::Px(-4.0),
        ..Default::default()
    };

    assets.images.spawn_background(commands, CleanupMarker);

    overlay.spawn(commands, CleanupMarker, |parent| {
        top.spawn(parent, |parent| {
            level_number.spawn(parent);
            top_right.spawn(parent, |parent| {
                moves.spawn(parent, CounterMarker::new(CounterKind::Moves));
                record_or_new_level.spawn(parent);
            });
        });
        bottom.spawn(parent, |parent| {
            bottom_left.spawn(parent, |parent| {
                reload.spawn(parent);
                selection.spawn(parent);
            });
            bottom_right.spawn(parent, |parent| {
                undos_left.spawn(parent, CounterMarker::new(CounterKind::Undos));
                undo.spawn(parent);
            });
        });
    });
}
