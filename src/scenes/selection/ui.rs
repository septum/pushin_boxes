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
    let font = fonts.primary();

    let overlay = Overlay::extended();
    let top = Container::auto_height();
    let mut middle = Container::default();
    let bottom = Container::auto_height();

    let kind = game_state.0.get_selection_kind();
    let mut title = SimpleText::medium(format!("Select a {} Level", kind.to_str()), font);
    let press_button = SimpleText::small(
        format!("Press ENTER to switch to {} levels", kind.toggle().to_str()),
        font,
    );

    title.primary();
    middle
        .row()
        .wrap()
        .justify_start()
        .items_start()
        .content_start();

    overlay.spawn(&mut commands, |parent| {
        top.spawn(parent, |parent| {
            title.spawn(parent);
        });
        middle.spawn(parent, |parent| {
            if kind.is_stock() {
                spawn_stock_buttons(parent, &save_file, font);
            } else {
                spawn_custom_buttons(parent, &save_file, font);
            }
        });
        bottom.spawn(parent, |parent| {
            press_button.spawn(parent);
        });
    });
}
