use bevy::prelude::*;
use game_ui::{Container, DynamicText, GameText, Overlay, SimpleText};

use crate::assets::prelude::*;

use super::LEVEL_NAME_ID;

pub fn spawn(mut commands: Commands, fonts: Res<Fonts>) {
    let font = fonts.primary();

    let overlay = Overlay::default();
    let mut center = Container::size(600.0, 300.0);
    let level_name = Container::auto();

    let mut title = SimpleText::large("Level Passed!", font);
    let level_name_title = SimpleText::medium("Give this level a name:", font);
    let mut level_name_input = DynamicText::medium("", font);
    let press_button = SimpleText::small("Press ENTER to save the level", font);

    center.justify_between();

    title.primary();
    level_name_input
        .id(LEVEL_NAME_ID)
        .secondary()
        .dynamic_text_value("_");

    overlay.spawn(&mut commands, |parent| {
        center.spawn(parent, |parent| {
            title.spawn(parent);
            level_name.spawn(parent, |parent| {
                level_name_title.spawn(parent);
                level_name_input.spawn(parent);
            });
            press_button.spawn(parent);
        });
    });
}
