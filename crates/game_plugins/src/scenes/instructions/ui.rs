use bevy::prelude::*;
use bevy_ui_bits::{Container, Root, SimpleText, UiText};

use crate::assets::prelude::*;

pub fn spawn(mut commands: Commands, fonts: Res<Fonts>, images: Res<Images>) {
    let font = fonts.primary();

    let root = Root::new()
        .padding(UiRect::all(Val::Px(20.0)))
        .justify_between();
    let top = Container::new();
    let bottom = Container::new();

    let how_to_play = SimpleText::medium("How to Play", font).color(crate::theme::PRIMARY.into());
    let press_button = SimpleText::small("Press ESC to return to the title screen", font)
        .color(crate::theme::PRIMARY.into());

    let instructions = ImageNode {
        image: images.instructions.clone(),
        ..default()
    };

    commands.spawn((
        root,
        children![
            (top, children![how_to_play]),
            instructions,
            (bottom, children![press_button])
        ],
    ));
}
