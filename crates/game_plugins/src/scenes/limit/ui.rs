use bevy::prelude::*;
use game_ui::{Container, GameText, Overlay, SimpleText};

use crate::assets::prelude::*;

pub fn spawn(mut commands: Commands, fonts: Res<Fonts>) {
    let font = fonts.primary();

    let overlay = Overlay::default();
    let mut center = Container::height(140.0);

    let mut reached_limit =
        SimpleText::medium("You reached the limit\nfor the custom levels", font);
    let press_button = SimpleText::small(
        "Press SPACE to continue to the custom level selection",
        font,
    );

    center.justify_between();
    reached_limit.primary();

    overlay.spawn(&mut commands, |parent| {
        center.spawn(parent, |parent| {
            reached_limit.spawn(parent);
            press_button.spawn(parent);
        });
    });
}
