use bevy::prelude::*;

use crate::{
    assets::{Colors, GameAssets},
    ui,
};

use super::{ButtonKind, ButtonMarker, CleanupMarker};

fn spawn_background(commands: &mut Commands, assets: &GameAssets) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.images.background.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(CleanupMarker);
}

fn create_button(text: &str, font: Handle<Font>) -> ui::Button {
    ui::Button::new(
        ui::SimpleText::new(
            text.to_string(),
            TextStyle {
                font_size: 35.0,
                color: Colors::DARK,
                font,
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
}

pub fn spawn(commands: &mut Commands, assets: &GameAssets) {
    let overlay = ui::Overlay::new();

    let top = ui::Housing::new(Val::Percent(100.0), Val::Percent(50.0));
    let bottom = ui::Housing::new(Val::Percent(100.0), Val::Percent(50.0));
    let mut actions = ui::Housing::new(Val::Percent(100.0), Val::Percent(90.0));
    let footer = ui::Housing::new(Val::Percent(100.0), Val::Percent(10.0));

    let title = ui::EmbossedText::new(
        "Pushin'\nBoxes".to_string(),
        4.0,
        TextStyle {
            font_size: 120.0,
            color: Colors::PRIMARY,
            font: assets.fonts.fredoka.clone(),
        },
    );

    let notice = ui::SimpleText::new(
        "Created by septum | https://septum.io".to_string(),
        TextStyle {
            font_size: 21.0,
            color: Colors::LIGHT,
            font: assets.fonts.fredoka.clone(),
        },
    );

    let play = create_button("Play", assets.fonts.fredoka.clone());
    let options = create_button("Options", assets.fonts.fredoka.clone());
    let quit = create_button("Quit", assets.fonts.fredoka.clone());

    actions.set_justify_content(JustifyContent::SpaceEvenly);

    spawn_background(commands, assets);

    overlay.spawn(commands, CleanupMarker, |parent| {
        top.spawn(parent, |parent| {
            title.spawn(parent);
        });
        bottom.spawn(parent, |parent| {
            actions.spawn(parent, |parent| {
                play.spawn(parent, ButtonMarker::new(ButtonKind::Play));
                options.spawn(parent, ButtonMarker::new(ButtonKind::Options));
                quit.spawn(parent, ButtonMarker::new(ButtonKind::Quit));
            });
            footer.spawn(parent, |parent| {
                notice.spawn(parent);
            });
        });
    });
}
