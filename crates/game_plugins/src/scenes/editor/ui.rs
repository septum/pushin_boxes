use bevy::prelude::*;
use bevy_ui_bits::{Container, DynamicTextBuilder, Root, SimpleText, UiText};

use crate::assets::prelude::*;

pub const VALID_ID: usize = 0;

pub fn spawn(mut commands: Commands, fonts: Res<Fonts>) {
    let font = fonts.primary();

    let root = Root::new();
    let top = Container::height(Val::Auto).row().justify_between();
    let bottom = Container::height(Val::Auto).row().justify_between();

    let top_left = Container::width(Val::Percent(50.0))
        .justify_start()
        .items_start();
    let top_right = Container::width(Val::Percent(50.0))
        .justify_start()
        .items_end();
    let bottom_left = Container::width(Val::Percent(50.0))
        .justify_end()
        .items_start();
    let bottom_right = Container::width(Val::Percent(50.0))
        .justify_end()
        .items_end();

    let title = SimpleText::medium("Editor", font);
    let subtitle =
        SimpleText::small("Custom Level Creation", font).color(crate::theme::SECONDARY.into());
    let valid = DynamicTextBuilder::medium("Valid: ", font).id(VALID_ID);
    let instructions = SimpleText::small(
        "A valid level has at least one box and a zone per box",
        font,
    );
    let toggle = SimpleText::small("(ENTER) - Toggle EntityComponent", font)
        .color(crate::theme::PRIMARY.into());
    let playtest =
        SimpleText::small("(SPACE) - Playtest Level", font).color(crate::theme::PRIMARY.into());

    commands.spawn((
        root,
        children![
            (
                top,
                children![
                    (top_left, children![title, subtitle]),
                    (top_right, children![valid.build(), instructions])
                ]
            ),
            (
                bottom,
                children![
                    (bottom_left, children![toggle]),
                    (bottom_right, children![playtest])
                ]
            )
        ],
    ));
}
