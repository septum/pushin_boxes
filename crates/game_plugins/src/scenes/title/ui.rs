use bevy::{prelude::*, text::LineHeight};
use bevy_ui_bits::{Container, EmbossedText, Root, SimpleText, UiButton, UiText};

use crate::assets::prelude::*;

pub const PLAY_ID: usize = 0;
pub const INSTRUCTIONS_ID: usize = 1;
pub const EDITOR_ID: usize = 2;
pub const OPTIONS_ID: usize = 3;
pub const QUIT_ID: usize = 4;

#[allow(clippy::too_many_lines)]
pub fn spawn(
    mut commands: Commands,
    fonts: Res<Fonts>,
    selected_button: Res<super::plugin::SelectedButton>,
) {
    let font = fonts.primary();

    let root = Root::new();
    let center: Container;
    #[cfg(not(target_family = "wasm"))]
    {
        center = Container::height(Val::Px(700.0)).justify_between();
    }
    #[cfg(target_family = "wasm")]
    {
        center = Container::height(Val::Px(600.0)).justify_between();
    }
    let top = Container::new();
    let bottom: Container;
    #[cfg(not(target_family = "wasm"))]
    {
        bottom = Container::height(Val::Px(380.0)).justify_between();
    }
    #[cfg(target_family = "wasm")]
    {
        bottom = Container::height(Val::Px(280.0)).justify_between();
    }
    let actions = Container::new();
    let footer = Container::new();

    let title = SimpleText::extra_large("Pushin'\nBoxes", font)
        .color(crate::theme::PRIMARY.into())
        .line_height(LineHeight::RelativeToFont(1.0));
    let notice = SimpleText::small("By @septum\nand @weymanator", font);

    let mut play = UiButton::rectangle().id(PLAY_ID);
    let play_text = EmbossedText::medium("Play", font);
    let mut instructions = UiButton::rectangle().id(INSTRUCTIONS_ID);
    let instructions_text = EmbossedText::medium("Instructions", font);
    let mut options = UiButton::rectangle().id(OPTIONS_ID);
    let options_text = EmbossedText::medium("Options", font);

    match selected_button.0 {
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

        match selected_button.0 {
            EDITOR_ID => {
                editor = editor.background_color(crate::theme::PRIMARY_DARK);
            }
            QUIT_ID => {
                quit = quit.background_color(crate::theme::PRIMARY_DARK);
            }
            _ => {}
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
