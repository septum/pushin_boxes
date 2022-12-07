use bevy::prelude::*;
use iyes_loopless::state::CurrentState;

use crate::{
    resources::prelude::*,
    ui::{Container, GameButton, GameText, Overlay, SimpleText},
};

fn spawn_stock_buttons(parent: &mut ChildBuilder, save_file: &SaveFile, font: &Handle<Font>) {
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

fn spawn_custom_buttons(parent: &mut ChildBuilder, save_file: &SaveFile, font: &Handle<Font>) {
    for (index, (key, record)) in save_file.enumerated_custom_records() {
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
        button.payload(key.clone());

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
    game_state: Res<CurrentState<GameState>>,
    fonts: Res<Fonts>,
    save_file: Res<SaveFile>,
) {
    let is_custom_selection = if let GameState::Selection(custom) = game_state.0 {
        custom
    } else {
        unreachable!("The current game state is invalid, it should be Selection");
    };

    let font = fonts.primary();

    let overlay = Overlay::extended();
    let top = Container::auto_height();
    let mut bottom = Container::default();

    let selection_kind = if is_custom_selection {
        "Custom"
    } else {
        "Stock"
    };
    let mut title = SimpleText::medium(format!("Select a {selection_kind} Level"), font);

    title.primary();
    bottom
        .row()
        .wrap_reverse()
        .justify_start()
        .items_start()
        .content_start();

    overlay.spawn(&mut commands, |parent| {
        top.spawn(parent, |parent| {
            title.spawn(parent);
        });
        bottom.spawn(parent, |parent| {
            if is_custom_selection {
                spawn_custom_buttons(parent, &save_file, font);
            } else {
                spawn_stock_buttons(parent, &save_file, font);
            }
        });
    });
}
