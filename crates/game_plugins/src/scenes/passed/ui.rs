use bevy::prelude::*;
use bevy_ui_bits::{Container, DynamicTextBuilder, Root, SimpleText, UiText};

use crate::assets::prelude::*;

pub const LEVEL_NAME_ID: usize = 1;

pub fn spawn(mut commands: Commands, fonts: Res<Fonts>) {
    let font = fonts.primary();

    let root = Root::default();
    let center = Container::size(Val::Px(600.0), Val::Px(300.0)).justify_between();
    let level_name = Container::size(Val::Auto, Val::Auto);

    let title = SimpleText::large("Level Passed!", font).color(crate::theme::PRIMARY.into());
    let level_name_title = SimpleText::medium("Give this level a name:", font);
    let level_name_input = DynamicTextBuilder::medium("", font)
        .id(LEVEL_NAME_ID)
        .color(crate::theme::SECONDARY.into())
        .initial_dynamic_text("_");
    let press_button = SimpleText::small("Press ENTER to save the level", font);

    commands.spawn((
        root,
        children![(
            center,
            children![
                title,
                (
                    level_name,
                    children![level_name_title, level_name_input.build()]
                ),
                press_button
            ]
        )],
    ));
}
