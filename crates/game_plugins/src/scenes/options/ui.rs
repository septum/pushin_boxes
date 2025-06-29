use bevy::prelude::*;
use game_ui::{Container, DynamicText, GameText, Overlay, SimpleText};

use crate::assets::prelude::*;

use super::VOLUME_ID;

pub fn spawn(mut commands: Commands, fonts: Res<Fonts>) {
    let font = fonts.primary();

    let overlay = Overlay::extended();
    let top = Container::auto();
    let center = Container::auto();
    let bottom = Container::auto();

    let mut how_to_play = SimpleText::medium("Options", font);
    let mut volume = DynamicText::medium("Volume: ", font);
    let mut press_button = SimpleText::small("Press ESC to return to the title screen", font);

    how_to_play.primary();
    volume.id(VOLUME_ID).secondary();
    press_button.primary();

    overlay.spawn(&mut commands, |parent| {
        top.spawn(parent, |parent| {
            how_to_play.spawn(parent);
        });
        center.spawn(parent, |parent| {
            volume.spawn(parent);
        });
        bottom.spawn(parent, |parent| {
            press_button.spawn(parent);
        });
    });
}
