use bevy::prelude::*;

use crate::{
    resources::prelude::*,
    ui::{Container, DynamicText, GameText, Overlay, SimpleText},
};

use super::VALID_ID;

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
    let mut switch = SimpleText::small("(SPACE) - Switch Entity", font);
    let mut set = SimpleText::small("(ENTER) - Playtest Level", font);
    let mut playtest = SimpleText::small("(ENTER) - Playtest Level", font);
    let instructions = SimpleText::small(
        "A valid level has a player, at least one box and a zone per box",
        font,
    );

    top.row().justify_between();
    bottom.row().justify_between();

    top_left.justify_start().align_start();
    top_right.justify_start().align_end();
    bottom_left.justify_end().align_start();
    bottom_right.justify_end().align_end();

    valid.id(VALID_ID);

    switch.primary();
    set.primary();
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
            });
        });
        bottom.spawn(parent, |parent| {
            bottom_left.spawn(parent, |parent| {
                switch.spawn(parent);
                set.spawn(parent);
            });
            bottom_right.spawn(parent, |parent| {
                playtest.spawn(parent);
                instructions.spawn(parent);
            });
        });
    });
}
