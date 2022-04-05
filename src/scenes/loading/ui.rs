use bevy::prelude::*;

use crate::{
    assets::{Colors, GameAssets},
    ui,
};

use super::CleanupMarker;

pub fn spawn(commands: &mut Commands, assets: &GameAssets) {
    let overlay = ui::Overlay::new();

    let loading_text = ui::Text::new(
        "Loading...".to_string(),
        TextStyle {
            font: assets.fonts.fredoka.clone(),
            font_size: 70.0,
            color: Colors::PRIMARY,
        },
    );

    overlay.spawn(commands, CleanupMarker, |parent| {
        loading_text.spawn(parent);
    });
}
