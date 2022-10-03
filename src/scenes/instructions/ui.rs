use bevy::prelude::*;

use crate::{
    resources::prelude::*,
    ui::{Container, GameText, Overlay, Picture, SimpleText},
};

pub fn spawn(mut commands: Commands, fonts: Res<Fonts>, images: Res<Images>) {
    let font = fonts.primary();

    let overlay = Overlay::extended();
    let top = Container::auto();
    let bottom = Container::auto();

    let mut how_to_play = SimpleText::medium("How to Play", font);
    let mut press_button = SimpleText::small("Press space to continue", font);

    let instructions = Picture::new(336.0, 463.0, &images.instructions);

    how_to_play.primary();
    press_button.primary();

    overlay.spawn(&mut commands, |parent| {
        top.spawn(parent, |parent| {
            how_to_play.spawn(parent);
        });
        instructions.spawn(parent);
        bottom.spawn(parent, |parent| {
            press_button.spawn(parent);
        });
    });
}
