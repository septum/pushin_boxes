use bevy::prelude::*;

use super::CleanupMarker;
use crate::assets::{Colors, GameAssets};

fn spawn_background(commands: &mut Commands, assets: &GameAssets) {
    commands.spawn_bundle(SpriteBundle {
        texture: assets.images.background.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}

fn spawn_title(parent: &mut ChildBuilder, assets: &GameAssets) {
    let housing = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            size: Size::new(Val::Px(341.0), Val::Px(244.0)),
            ..Default::default()
        },
        color: Colors::TRANSPARENT.into(),
        ..Default::default()
    };

    let surface = TextBundle {
        style: Style {
            position: Rect {
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                ..Default::default()
            },
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        text: Text::with_section(
            "Pushin'\nBoxes",
            TextStyle {
                font_size: 120.0,
                color: Colors::PRIMARY,
                font: assets.fonts.fredoka.clone(),
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    };

    let mut relief = surface.clone();

    relief.text.sections[0].style.color = Colors::DARK;
    relief.style.position = Rect {
        bottom: Val::Px(0.0),
        right: Val::Px(0.0),
        ..Default::default()
    };

    parent.spawn_bundle(housing).with_children(|parent| {
        parent.spawn_bundle(relief);
        parent.spawn_bundle(surface);
    });
}

fn spawn_play_button(parent: &mut ChildBuilder, assets: &GameAssets) {
    let background = NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Px(280.0), Val::Px(56.0)),
            ..Default::default()
        },
        color: Colors::PRIMARY.into(),
        ..Default::default()
    };

    let text = TextBundle {
        text: Text::with_section(
            "Play",
            TextStyle {
                font_size: 35.0,
                color: Colors::DARK,
                font: assets.fonts.fredoka.clone(),
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    };

    parent.spawn_bundle(background).with_children(|parent| {
        parent.spawn_bundle(text);
    });
}

fn spawn_options_button(parent: &mut ChildBuilder, assets: &GameAssets) {
    let background = NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Px(280.0), Val::Px(56.0)),
            ..Default::default()
        },
        color: Colors::PRIMARY.into(),
        ..Default::default()
    };

    let text = TextBundle {
        text: Text::with_section(
            "Options",
            TextStyle {
                font_size: 35.0,
                color: Colors::DARK,
                font: assets.fonts.fredoka.clone(),
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    };

    parent.spawn_bundle(background).with_children(|parent| {
        parent.spawn_bundle(text);
    });
}

fn spawn_quit_button(parent: &mut ChildBuilder, assets: &GameAssets) {
    let background = NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Px(280.0), Val::Px(56.0)),
            ..Default::default()
        },
        color: Colors::PRIMARY.into(),
        ..Default::default()
    };

    let text = TextBundle {
        text: Text::with_section(
            "Quit",
            TextStyle {
                font_size: 35.0,
                color: Colors::DARK,
                font: assets.fonts.fredoka.clone(),
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    };

    parent.spawn_bundle(background).with_children(|parent| {
        parent.spawn_bundle(text);
    });
}

fn spawn_notice(parent: &mut ChildBuilder, assets: &GameAssets) {
    let text = TextBundle {
        text: Text::with_section(
            "Created by septum | https://septum.io",
            TextStyle {
                font_size: 21.0,
                color: Colors::LIGHT,
                font: assets.fonts.fredoka.clone(),
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    };

    parent.spawn_bundle(text);
}

pub fn spawn(commands: &mut Commands, assets: &GameAssets) {
    let container = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        color: Colors::TRANSPARENT.into(),
        ..Default::default()
    };

    let housing = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: Colors::TRANSPARENT.into(),
        ..Default::default()
    };

    let mut buttons_housing = housing.clone();
    buttons_housing.style.size = Size::new(Val::Percent(100.0), Val::Percent(90.0));
    buttons_housing.style.justify_content = JustifyContent::SpaceEvenly;

    let mut notice_housing = housing.clone();
    notice_housing.style.size = Size::new(Val::Percent(100.0), Val::Percent(10.0));

    spawn_background(commands, &assets);

    commands
        .spawn_bundle(container)
        .with_children(|parent| {
            parent
                .spawn_bundle(housing.clone())
                .with_children(|parent| {
                    spawn_title(parent, &assets);
                });
            parent.spawn_bundle(housing).with_children(|parent| {
                parent
                    .spawn_bundle(buttons_housing)
                    .with_children(|parent| {
                        spawn_play_button(parent, &assets);
                        spawn_options_button(parent, &assets);
                        spawn_quit_button(parent, &assets);
                    });
                parent.spawn_bundle(notice_housing).with_children(|parent| {
                    spawn_notice(parent, &assets);
                });
            });
        })
        .insert(CleanupMarker);
}
