use bevy::prelude::*;
use bevy_ui_bits::{Container, EmbossedText, Root, SimpleText, UiButton, UiText};

use crate::{assets::prelude::*, scenes::title::plugin::SelectedButton};

pub const PLAY_ID: usize = 0;
pub const INSTRUCTIONS_ID: usize = 1;
pub const EDITOR_ID: usize = 2;
pub const OPTIONS_ID: usize = 3;
pub const QUIT_ID: usize = 4;

#[allow(clippy::too_many_lines)]
pub fn spawn(mut commands: Commands, fonts: Res<Fonts>, selected_button: Res<SelectedButton>) {
    let font = fonts.primary();

    let root = Root::default();
    let center: Container;
    #[cfg(not(target_family = "wasm"))]
    {
        center = Container::height(Val::Px(700.0)).justify_between();
    }
    #[cfg(target_family = "wasm")]
    {
        center = Container::height(Val::Px(600.0)).justify_between();
    }
    let top = Container::size(Val::Auto, Val::Auto);
    let bottom: Container;
    #[cfg(not(target_family = "wasm"))]
    {
        bottom = Container::height(Val::Px(380.0)).justify_between();
    }
    #[cfg(target_family = "wasm")]
    {
        bottom = Container::height(Val::Px(280.0)).justify_between();
    }
    let actions = Container::size(Val::Auto, Val::Auto);
    let footer = Container::size(Val::Auto, Val::Auto);

    let title = SimpleText::extra_large("Pushin'\nBoxes", font).color(crate::theme::PRIMARY.into());
    let notice = SimpleText::small("By @septum\nand @weymanator", font);

    let mut play = UiButton::rectangle().id(PLAY_ID);
    let play_text = EmbossedText::medium("Play", font);
    let mut instructions = UiButton::rectangle().id(INSTRUCTIONS_ID);
    let instructions_text = EmbossedText::medium("Instructions", font);
    let mut options = UiButton::rectangle().id(OPTIONS_ID);
    let options_text = EmbossedText::medium("Options", font);

    if let Some(id) = selected_button.0 {
        match id {
            PLAY_ID => {
                play = play.background_color(crate::theme::PRIMARY_DARK);
            }
            INSTRUCTIONS_ID => {
                instructions = instructions.background_color(crate::theme::PRIMARY_DARK);
            }
            OPTIONS_ID => {
                options = options.background_color(crate::theme::PRIMARY_DARK);
            }
            _ => {}
        }
    }

    #[cfg(target_family = "wasm")]
    {
        let editor_available = SimpleText::small(
            "\n> Hey! The level editor is not ready for the web version, yet :) <",
            font,
        )
        .color(crate::theme::PRIMARY.into());

        commands.spawn((
            root,
            children![(
                center,
                children![
                    (top, children![title]),
                    (
                        bottom,
                        children![
                            (
                                actions,
                                children![
                                    (play, children![play_text]),
                                    (instructions, children![instructions_text]),
                                    (options, children![options_text]),
                                ]
                            ),
                            (footer, children![notice, editor_available])
                        ]
                    )
                ]
            )],
        ));
    }

    #[cfg(not(target_family = "wasm"))]
    {
        let mut editor = UiButton::rectangle().id(EDITOR_ID);
        let editor_text = EmbossedText::medium("Editor", font);
        let mut quit = UiButton::rectangle().id(QUIT_ID);
        let quit_text = EmbossedText::medium("Quit", font);

        if let Some(id) = selected_button.0 {
            match id {
                EDITOR_ID => {
                    editor = editor.background_color(crate::theme::PRIMARY_DARK);
                }
                QUIT_ID => {
                    quit = quit.background_color(crate::theme::PRIMARY_DARK);
                }
                _ => {}
            }
        }

        commands.spawn((
            root,
            children![(
                center,
                children![
                    (top, children![title]),
                    (
                        bottom,
                        children![
                            (
                                actions,
                                children![
                                    (play, children![play_text]),
                                    (instructions, children![instructions_text]),
                                    (editor, children![editor_text]),
                                    (options, children![options_text]),
                                    (quit, children![quit_text]),
                                ]
                            ),
                            (footer, children![notice])
                        ]
                    )
                ]
            )],
        ));
    }
}
