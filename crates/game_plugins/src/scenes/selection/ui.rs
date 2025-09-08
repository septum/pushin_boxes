use bevy::{ecs::spawn::SpawnIter, prelude::*};
use bevy_ui_bits::{Container, EmbossedText, Root, SimpleText, UiButton, UiText};

use crate::{assets::prelude::*, save_file::SaveFile, state::GameState};

fn spawn_stock_buttons(
    save_file: &SaveFile,
    font: &Handle<Font>,
) -> Vec<(Container, UiButton, EmbossedText, SimpleText)> {
    let mut buttons = vec![];

    for (index, record) in save_file.enumerated_stock_records() {
        let housing = Container::size(Val::Percent(25.0), Val::Percent(25.0));
        let mut button = UiButton::square().id(index);
        let button_text = EmbossedText::medium(&format!("{}", index + 1), font);
        let record_new_level = if record.is_set() {
            SimpleText::small(&format!("Record: {}", record.moves_in_time('\n')), font)
        } else {
            SimpleText::small("New Level!\n ", font).color(crate::theme::SECONDARY.into())
        };

        if index == save_file.unlocked_levels() - 1 {
            button = button.background_color(crate::theme::PRIMARY_DARK);
        }

        buttons.push((housing, button, button_text, record_new_level));
    }

    buttons
}

fn spawn_custom_buttons(
    save_file: &SaveFile,
    font: &Handle<Font>,
) -> Vec<(Container, UiButton, EmbossedText, SimpleText)> {
    let mut buttons = vec![];

    for (index, (key, record)) in save_file.ordered_custom_records() {
        let housing = Container::size(Val::Percent(25.0), Val::Percent(25.0));
        let split_key: Vec<&str> = key.split('$').collect();
        let mut button = UiButton::rectangle().id(index).payload(key);
        let button_text = EmbossedText::medium(split_key[0], font);
        let record_new_level = if record.is_set() {
            SimpleText::small(&format!("Record: {}", record.moves_in_time('\n')), font)
        } else {
            SimpleText::small("New Level!\n ", font).color(crate::theme::SECONDARY.into())
        };

        if index == save_file.number_custom_levels() - 1 {
            button = button.background_color(crate::theme::PRIMARY_DARK);
        }

        buttons.push((housing, button, button_text, record_new_level));
    }

    buttons
}

pub fn spawn(
    mut commands: Commands,
    game_state: Res<State<GameState>>,
    fonts: Res<Fonts>,
    save_file: Res<SaveFile>,
) {
    let font = fonts.primary();

    let root = Root::new()
        .padding(UiRect::all(Val::Px(20.0)))
        .justify_between();
    let top = Container::new();
    let middle = Container::size(Val::Percent(100.0), Val::Percent(100.0))
        .row()
        .wrap()
        .justify_start()
        .items_start()
        .content_start();

    let kind = game_state.get_selection_kind();
    let title = SimpleText::medium(&format!("Select a {} Level", kind.to_str()), font)
        .color(crate::theme::PRIMARY.into());

    #[cfg(target_family = "wasm")]
    {
        commands.spawn((
            root,
            children![
                (top, children![title]),
                (
                    middle,
                    Children::spawn(SpawnIter(
                        spawn_stock_buttons(&save_file, font).into_iter().map(
                            |(housing, button, button_text, record_new_level)| {
                                (
                                    housing,
                                    children![(button, children![button_text]), record_new_level],
                                )
                            }
                        ),
                    ))
                )
            ],
        ));
    }

    #[cfg(not(target_family = "wasm"))]
    {
        let bottom = Container::width(Val::Percent(100.0))
            .row()
            .justify_between();
        let enter = SimpleText::small(
            &format!("(ENTER) - Switch to {} levels", kind.toggle().to_str()),
            font,
        )
        .color(crate::theme::PRIMARY.into());
        let delete = SimpleText::small("(DELETE) - Remove a custom level", font)
            .color(crate::theme::PRIMARY.into());

        commands.spawn((
            root,
            children![
                (top, children![title]),
                (
                    middle,
                    Children::spawn(SpawnIter(
                        (if kind.is_stock() {
                            spawn_stock_buttons(&save_file, font)
                        } else {
                            spawn_custom_buttons(&save_file, font)
                        })
                        .into_iter()
                        .map(
                            |(housing, button, button_text, record_new_level)| {
                                (
                                    housing,
                                    children![(button, children![button_text]), record_new_level],
                                )
                            }
                        ),
                    ))
                ),
                (bottom, children![enter, delete])
            ],
        ));
    }
}
