use bevy::prelude::*;

use super::CleanupMarker;
use crate::assets::{Colors, GameAssets};

pub fn spawn(commands: &mut Commands, assets: &GameAssets) {
    let container = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: Colors::TRANSPARENT.into(),
        ..Default::default()
    };

    let loading_text = TextBundle {
        text: Text::with_section(
            "Loading...",
            TextStyle {
                font: assets.fonts.fredoka.clone(),
                font_size: 70.0,
                color: Colors::PRIMARY,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    };

    commands
        .spawn_bundle(container)
        .with_children(|parent| {
            parent.spawn_bundle(loading_text);
        })
        .insert(CleanupMarker);
}
