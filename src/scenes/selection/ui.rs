use bevy::prelude::*;

use crate::{
    resources::prelude::*,
    ui::{Container, GameButton, GameText, Overlay, SimpleText},
};

fn spawn_stock_buttons(parent: &mut ChildBuilder, save_file: &SaveFile, font: &Handle<Font>) {
    let last_index = save_file.stock_levels_len();
    for (index, record) in save_file.stock.iter().enumerate() {
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

        if last_index == index + 1 {
            button.selected();
        }

        housing.spawn(parent, |parent| {
            button.spawn(parent);
            record_new_level.spawn(parent);
        });
    }
}

pub fn spawn(mut commands: Commands, fonts: Res<Fonts>, save_file: Res<SaveFile>) {
    let font = fonts.primary();

    let overlay = Overlay::extended();
    let top = Container::auto_height();
    let mut bottom = Container::default();

    let mut title = SimpleText::medium("Select a Level", font);

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
            spawn_stock_buttons(parent, &save_file, font);
        });
    });
}
