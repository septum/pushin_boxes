use bevy::prelude::*;

use crate::assets::GAME_COLORS;

use super::CleanupMarker;

pub fn spawn(commands: &mut Commands, font: Handle<Font>) {
    let container = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: GAME_COLORS.transparent.into(),
        ..Default::default()
    };

    let loading_text = TextBundle {
        text: Text::with_section(
            "Loading...",
            TextStyle {
                font,
                font_size: 70.0,
                color: GAME_COLORS.primary,
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
