use bevy::prelude::*;
use game_ui::{Container, GameButton, GameText, Overlay, SimpleText};

use crate::resources::prelude::*;

fn spawn_stock_buttons(
    parent: &mut ChildSpawnerCommands,
    save_file: &SaveFile,
    font: &Handle<Font>,
) {
    let last_unlocked_index = save_file.unlocked_levels() - 1;
    for (index, record) in save_file.enumerated_stock_records() {
        let housing = Container::size_percentage(25.0, 25.0);
        let mut button = GameButton::square(format!("{}", index + 1), font);
        let record_new_level = if record.is_set() {
            SimpleText::small(format!("Record: {}", record.moves_in_time("\n")), font)
        } else {
            let mut simple_text = SimpleText::small("New Level!\n ".to_string(), font);
            simple_text.secondary();
            simple_text
        };

        button.id(index);

        if last_unlocked_index == index {
            button.selected();
        }

        housing.spawn(parent, |parent| {
            button.spawn(parent);
            record_new_level.spawn(parent);
        });
    }
}

fn spawn_custom_buttons(
    parent: &mut ChildSpawnerCommands,
    save_file: &SaveFile,
    font: &Handle<Font>,
) {
    for (index, (key, record)) in save_file.ordered_custom_records() {
        let housing = Container::size_percentage(25.0, 25.0);
        let split_key: Vec<&str> = key.split('$').collect();
        let mut button = GameButton::new(split_key[0], font);
        let record_new_level = if record.is_set() {
            SimpleText::small(format!("Record: {}", record.moves_in_time("\n")), font)
        } else {
            let mut simple_text = SimpleText::small("New Level!\n ".to_string(), font);
            simple_text.secondary();
            simple_text
        };

        button.id(index);
        button.payload(key.clone().clone());

        if index == 0 {
            button.selected();
        }

        housing.spawn(parent, |parent| {
            button.spawn(parent);
            record_new_level.spawn(parent);
        });
    }
}

pub fn spawn(
    mut commands: Commands,
    game_state: Res<State<GameState>>,
    fonts: Res<Fonts>,
    save_file: Res<SaveFile>,
) {
    let font = fonts.primary();

    let overlay = Overlay::extended();
    let top = Container::auto_height();
    let mut middle = Container::default();
    let mut bottom = Container::auto_height();

    let kind = game_state.get_selection_kind();
    let mut title = SimpleText::medium(format!("Select a {} Level", kind.to_str()), font);

    title.primary();
    middle
        .row()
        .wrap()
        .justify_start()
        .items_start()
        .content_start();

    bottom.row().justify_between();

    overlay.spawn(&mut commands, |parent| {
        top.spawn(parent, |parent| {
            title.spawn(parent);
        });
        middle.spawn(parent, |parent| {
            #[cfg(target_family = "wasm")]
            {
                spawn_stock_buttons(parent, &save_file, font);
            }
            #[cfg(not(target_family = "wasm"))]
            {
                if kind.is_stock() {
                    spawn_stock_buttons(parent, &save_file, font);
                } else {
                    spawn_custom_buttons(parent, &save_file, font);
                }
            }
        });
        #[cfg(not(target_family = "wasm"))]
        {
            bottom.spawn(parent, |parent| {
                let mut enter = SimpleText::small(
                    format!("(ENTER) - Switch to {} levels", kind.toggle().to_str()),
                    font,
                );
                let mut delete = SimpleText::small("(DELETE) - Remove a custom level", font);

                enter.primary();
                delete.primary();

                enter.spawn(parent);
                delete.spawn(parent);
            });
        }
    });
}
