use bevy::prelude::*;

use crate::{
    assets::{Colors, GameAssets},
    ui,
};

use super::CleanupMarker;

fn spawn_background(commands: &mut Commands, assets: &GameAssets) {
    commands.spawn_bundle(SpriteBundle {
        texture: assets.images.background.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}

pub fn spawn(commands: &mut Commands, assets: &GameAssets) {
    let mut overlay = ui::Overlay::new();

    let mut top = ui::Housing::new(Val::Percent(97.0), Val::Percent(10.0));
    let mut top_right = ui::Housing::new(Val::Percent(50.0), Val::Percent(100.0));
    let mut bottom = ui::Housing::new(Val::Percent(97.0), Val::Percent(10.0));
    let mut bottom_left = ui::Housing::new(Val::Percent(50.0), Val::Percent(100.0));
    let mut bottom_right = ui::Housing::new(Val::Percent(50.0), Val::Percent(100.0));

    let level = ui::SimpleText::new(
        "Level 1".to_string(),
        TextStyle {
            font: assets.fonts.fredoka.clone(),
            font_size: 42.0,
            color: Colors::LIGHT,
        },
    );
    let moves = ui::SimpleText::new(
        "Moves: 0".to_string(),
        TextStyle {
            font: assets.fonts.fredoka.clone(),
            font_size: 28.0,
            color: Colors::LIGHT,
        },
    );
    let mut new_level_or_record = ui::SimpleText::new(
        "New Level!".to_string(),
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
    let undos_left = ui::SimpleText::new(
        "Undos: 4".to_string(),
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

    new_level_or_record.bundle.style.position = Rect {
        top: Val::Px(-4.0),
        ..Default::default()
    };

    undo.bundle.style.position = Rect {
        top: Val::Px(-4.0),
        ..Default::default()
    };

    spawn_background(commands, assets);

    overlay.spawn(commands, CleanupMarker, |parent| {
        top.spawn(parent, |parent| {
            level.spawn(parent);
            top_right.spawn(parent, |parent| {
                moves.spawn(parent);
                new_level_or_record.spawn(parent);
            });
        });
        bottom.spawn(parent, |parent| {
            bottom_left.spawn(parent, |parent| {
                reload.spawn(parent);
                selection.spawn(parent);
            });
            bottom_right.spawn(parent, |parent| {
                undos_left.spawn(parent);
                undo.spawn(parent);
            });
        });
    });
}
