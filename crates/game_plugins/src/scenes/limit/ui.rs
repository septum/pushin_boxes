use bevy::prelude::*;
use bevy_ui_bits::{Container, Root, SimpleText, UiText};

use crate::assets::prelude::*;

pub fn spawn(mut commands: Commands, fonts: Res<Fonts>) {
    let font = fonts.primary();

    let root = Root::new();
    let center = Container::height(Val::Px(140.0)).justify_between();

    let reached_limit = SimpleText::medium("You reached the limit\nfor the custom levels", font)
        .color(crate::theme::PRIMARY.into());
    let press_button = SimpleText::small(
        "Press SPACE to continue to the custom level selection",
        font,
    );

    commands.spawn((
        root,
        children![(center, children![reached_limit, press_button])],
    ));
}
