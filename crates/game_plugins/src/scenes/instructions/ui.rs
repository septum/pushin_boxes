use bevy::prelude::*;
use game_ui::{Container, GameText, Overlay, Picture, SimpleText};

use crate::resources::prelude::*;

pub fn spawn(mut commands: Commands, fonts: Res<Fonts>, images: Res<Images>) {
    let font = fonts.primary();

    let overlay = Overlay::extended();
    let top = Container::auto();
    let bottom = Container::auto();

    let mut how_to_play = SimpleText::medium("How to Play", font);
    let mut press_button = SimpleText::small("Press ESC to return to the title screen", font);

    // 714.0, 326.0
    let instructions = Picture::new(&images.instructions);

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
