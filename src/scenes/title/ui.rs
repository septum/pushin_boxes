use bevy::prelude::*;

use crate::{
    resources::prelude::*,
    ui::{Container, GameButton, GameText, Overlay, SimpleText},
};

use super::{EDITOR_ID, INSTRUCTIONS_ID, PLAY_ID, QUIT_ID};

pub fn spawn(mut commands: Commands, fonts: Res<Fonts>) {
    let font = fonts.primary();

    let overlay = Overlay::default();
    let mut center = Container::height(650.0);
    let top = Container::auto();
    let mut bottom = Container::height(320.0);
    let actions = Container::auto();
    let footer = Container::auto();

    let mut title = SimpleText::extra_large("Pushin'\nBoxes", font);
    let notice = SimpleText::small("By @septum (gh)\nand @andresweyman (ig)", font);

    let mut play = GameButton::new("Play", font);
    let mut instructions = GameButton::new("Instructions", font);
    let mut editor = GameButton::new("Editor", font);
    let mut quit = GameButton::new("Quit", font);

    title.primary();

    center.justify_between();
    bottom.justify_between();

    play.id(PLAY_ID).selected();
    instructions.id(INSTRUCTIONS_ID);
    editor.id(EDITOR_ID);
    quit.id(QUIT_ID);

    overlay.spawn(&mut commands, |parent| {
        center.spawn(parent, |parent| {
            top.spawn(parent, |parent| {
                title.spawn(parent);
            });
            bottom.spawn(parent, |parent| {
                actions.spawn(parent, |parent| {
                    play.spawn(parent);
                    instructions.spawn(parent);
                    editor.spawn(parent);
                    quit.spawn(parent);
                });
                footer.spawn(parent, |parent| {
                    notice.spawn(parent);
                });
            });
        });
    });
}
