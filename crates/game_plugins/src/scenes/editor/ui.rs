use bevy::prelude::*;
use game_ui::{Container, DynamicText, GameText, Overlay, SimpleText};

use crate::assets::prelude::*;

pub const VALID_ID: usize = 0;

pub fn spawn(mut commands: Commands, fonts: Res<Fonts>) {
    let font = fonts.primary();

    let overlay = Overlay::extended();
    let mut top = Container::auto_height();
    let mut bottom = Container::auto_height();

    let mut top_left = Container::half();
    let mut top_right = Container::half();
    let mut bottom_left = Container::half();
    let mut bottom_right = Container::half();

    let title = SimpleText::medium("Editor", font);
    let mut subtitle = SimpleText::small("Custom Level Creation", font);
    let mut valid = DynamicText::medium("Valid: ", font);
    let instructions = SimpleText::small(
        "A valid level has at least one box and a zone per box",
        font,
    );
    let mut toggle = SimpleText::small("(ENTER) - Toggle EntityComponent", font);
    let mut playtest = SimpleText::small("(SPACE) - Playtest Level", font);

    top.row().justify_between();
    bottom.row().justify_between();

    top_left.justify_start().align_start();
    top_right.justify_start().align_end();
    bottom_left.justify_end().align_start();
    bottom_right.justify_end().align_end();

    valid.id(VALID_ID);

    toggle.primary();
    playtest.primary();
    subtitle.secondary();

    overlay.spawn(&mut commands, |parent| {
        top.spawn(parent, |parent| {
            top_left.spawn(parent, |parent| {
                title.spawn(parent);
                subtitle.spawn(parent);
            });
            top_right.spawn(parent, |parent| {
                valid.spawn(parent);
                instructions.spawn(parent);
            });
        });
        bottom.spawn(parent, |parent| {
            bottom_left.spawn(parent, |parent| {
                toggle.spawn(parent);
            });
            bottom_right.spawn(parent, |parent| {
                playtest.spawn(parent);
            });
        });
    });
}
