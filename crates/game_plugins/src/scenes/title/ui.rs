use bevy::prelude::*;
use game_ui::{Container, GameButton, GameText, Overlay, SimpleText};

use crate::assets::prelude::*;

use super::{EDITOR_ID, INSTRUCTIONS_ID, OPTIONS_ID, PLAY_ID, QUIT_ID};

pub fn spawn(mut commands: Commands, fonts: Res<Fonts>) {
    let font = fonts.primary();

    let overlay = Overlay::default();
    let mut center: Container;
    #[cfg(not(target_family = "wasm"))]
    {
        center = Container::height(700.0);
    }
    #[cfg(target_family = "wasm")]
    {
        center = Container::height(600.0);
    }
    let top = Container::auto();
    let mut bottom: Container;
    #[cfg(not(target_family = "wasm"))]
    {
        bottom = Container::height(380.0);
    }
    #[cfg(target_family = "wasm")]
    {
        bottom = Container::height(280.0);
    }
    let actions = Container::auto();
    let footer = Container::auto();

    let mut title = SimpleText::extra_large("Pushin'\nBoxes", font);
    let notice = SimpleText::small("By @septum\nand @weymanator", font);

    let mut play = GameButton::new("Play", font);
    let mut instructions = GameButton::new("Instructions", font);
    let mut options = GameButton::new("Options", font);

    title.primary();

    center.justify_between();
    bottom.justify_between();

    play.id(PLAY_ID).selected();
    instructions.id(INSTRUCTIONS_ID);

    options.id(OPTIONS_ID);

    overlay.spawn(&mut commands, |parent| {
        center.spawn(parent, |parent| {
            top.spawn(parent, |parent| {
                title.spawn(parent);
            });
            bottom.spawn(parent, |parent| {
                actions.spawn(parent, |parent| {
                    play.spawn(parent);
                    instructions.spawn(parent);
                    #[cfg(not(target_family = "wasm"))]
                    {
                        let mut editor = GameButton::new("Editor", font);
                        editor.id(EDITOR_ID);
                        editor.spawn(parent);
                    }
                    options.spawn(parent);
                    #[cfg(not(target_family = "wasm"))]
                    {
                        let mut quit = GameButton::new("Quit", font);
                        quit.id(QUIT_ID);
                        quit.spawn(parent);
                    }
                });
                footer.spawn(parent, |parent| {
                    notice.spawn(parent);

                    #[cfg(target_family = "wasm")]
                    {
                        let mut editor_available = SimpleText::small(
                            "\n> Hey! The level editor is not ready for the web version, yet :) <",
                            font,
                        );
                        editor_available.primary();
                        editor_available.spawn(parent);
                    }
                });
            });
        });
    });
}
