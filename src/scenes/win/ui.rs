use bevy::prelude::*;

use crate::{
    resources::prelude::*,
    ui::{Container, GameText, Overlay, SimpleText},
};

pub fn spawn(mut commands: Commands, fonts: Res<Fonts>, level: Res<Level>) {
    let font = fonts.primary();

    let record = if level.new_record() {
        format!("NEW RECORD:\n{}", level.moves_in_time(" "))
    } else {
        " \n ".to_string()
    };

    let overlay = Overlay::default();
    let mut center = Container::size(540.0, 200.0);

    let mut record = SimpleText::medium(record, font);
    let mut title = SimpleText::large("You Win!   ", font);
    let press_button = SimpleText::small("Press space to continue", font);

    center.margin_bottom(100.0).justify_between();
    record.secondary();
    title.primary();

    overlay.spawn(&mut commands, |parent| {
        center.spawn(parent, |parent| {
            record.spawn(parent);
            title.spawn(parent);
            press_button.spawn(parent);
        });
    });
}
