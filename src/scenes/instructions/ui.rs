use bevy::prelude::*;

use crate::{
    resources::{AssetsHandles, Colors},
    ui,
};

use super::CleanupMarker;

pub fn spawn(commands: &mut Commands, assets: &AssetsHandles) {
    let overlay = ui::Overlay::new();

    let housing_a = ui::Housing::new(Val::Percent(100.0), Val::Percent(10.0));
    let housing_b = ui::Housing::new(Val::Percent(100.0), Val::Percent(20.0));
    let housing_c = ui::Housing::new(Val::Percent(100.0), Val::Percent(20.0));
    let housing_d = ui::Housing::new(Val::Percent(100.0), Val::Percent(20.0));
    let housing_e = ui::Housing::new(Val::Percent(100.0), Val::Percent(20.0));
    let housing_f = ui::Housing::new(Val::Percent(100.0), Val::Percent(10.0));

    let how_to_play = ui::EmbossedText::new(
        "How to Play".to_string(),
        2.0,
        TextStyle {
            font_size: 44.0,
            color: Colors::PRIMARY,
            font: assets.fonts.fredoka.clone(),
        },
    );

    let press = ui::SimpleText::new(
        "Press".to_string(),
        TextStyle {
            font_size: 44.0,
            color: Colors::LIGHT,
            font: assets.fonts.fredoka.clone(),
        },
    );

    let or = ui::SimpleText::new(
        "or".to_string(),
        TextStyle {
            font_size: 44.0,
            color: Colors::LIGHT,
            font: assets.fonts.fredoka.clone(),
        },
    );

    let to_move = ui::SimpleText::new(
        "to move        ,".to_string(),
        TextStyle {
            font_size: 44.0,
            color: Colors::LIGHT,
            font: assets.fonts.fredoka.clone(),
        },
    );

    let to_win = ui::SimpleText::new(
        "pushing all         into         to win!".to_string(),
        TextStyle {
            font_size: 44.0,
            color: Colors::LIGHT,
            font: assets.fonts.fredoka.clone(),
        },
    );

    let press_space = ui::EmbossedText::new(
        "Press [SPACE] to continue".to_string(),
        2.0,
        TextStyle {
            font_size: 24.0,
            color: Colors::PRIMARY,
            font: assets.fonts.fredoka.clone(),
        },
    );

    assets.images.spawn_background(commands, CleanupMarker);

    overlay.spawn(commands, CleanupMarker, |parent| {
        housing_a.spawn(parent, |parent| {
            how_to_play.spawn(parent);
        });
        housing_b.spawn(parent, |parent| {
            press.spawn(parent);
        });
        housing_c.spawn(parent, |parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..Default::default()
                },
                image: assets.images.controls.clone().into(),
                ..Default::default()
            });
            or.spawn(parent);
        });
        housing_d.spawn(parent, |parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Percent(56.0),
                        ..Default::default()
                    },
                    size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                    ..Default::default()
                },
                image: assets.images.player.idle.clone().into(),
                ..Default::default()
            });
            to_move.spawn(parent);
        });
        housing_e.spawn(parent, |parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Percent(39.0),
                        ..Default::default()
                    },
                    size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                    ..Default::default()
                },
                image: assets.images.entities.pbox.clone().into(),
                ..Default::default()
            });
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Percent(63.0),
                        ..Default::default()
                    },
                    size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                    ..Default::default()
                },
                image: assets.images.entities.zone.clone().into(),
                ..Default::default()
            });
            to_win.spawn(parent);
        });
        housing_f.spawn(parent, |parent| {
            press_space.spawn(parent);
        });
    });
}
