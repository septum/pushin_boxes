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
    let overlay = ui::Overlay::new();

    let top = ui::Housing::new(Val::Percent(100.0), Val::Percent(50.0));
    let bottom = ui::Housing::new(Val::Percent(100.0), Val::Percent(50.0));
    let mut actions = ui::Housing::new(Val::Percent(100.0), Val::Percent(90.0));
    actions.set_justify_content(JustifyContent::SpaceEvenly);
    let footer = ui::Housing::new(Val::Percent(100.0), Val::Percent(10.0));

    let title = ui::EmbossedText::new(
        "Pushin'\nBoxes".to_string(),
        TextStyle {
            font_size: 120.0,
            color: Colors::PRIMARY,
            font: assets.fonts.fredoka.clone(),
        },
    );

    let notice = ui::Text::new(
        "Created by septum | https://septum.io".to_string(),
        TextStyle {
            font_size: 21.0,
            color: Colors::LIGHT,
            font: assets.fonts.fredoka.clone(),
        },
    );

    let buttons = ["Play", "Options", "Quit"].iter().map(|text| {
        ui::Button::new(
            ui::Text::new(
                text.to_string(),
                TextStyle {
                    font_size: 35.0,
                    color: Colors::DARK,
                    font: assets.fonts.fredoka.clone(),
                },
            ),
            Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size::new(Val::Px(280.0), Val::Px(56.0)),
                ..Default::default()
            },
            Colors::PRIMARY.into(),
        )
    });

    spawn_background(commands, &assets);

    overlay.spawn(commands, CleanupMarker, |parent| {
        top.spawn(parent, |parent| {
            title.spawn(parent);
        });
        bottom.spawn(parent, |parent| {
            actions.spawn(parent, |parent| {
                for button in buttons {
                    button.spawn(parent);
                }
            });
            footer.spawn(parent, |parent| {
                notice.spawn(parent);
            });
        });
    });
}
