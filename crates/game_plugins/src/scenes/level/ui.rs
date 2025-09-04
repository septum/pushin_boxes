use bevy::prelude::*;
use bevy_ui_bits::{Container, DynamicTextBuilder, Root, SimpleText, UiText};

use crate::{assets::prelude::*, level::LevelResource};

pub const STOPWATCH_COUNTER_ID: usize = 0;
pub const MOVES_COUNTER_ID: usize = 1;
pub const UNDOS_COUNTER_ID: usize = 2;

pub fn spawn(mut commands: Commands, level: Res<LevelResource>, fonts: Res<Fonts>) {
    let font = fonts.primary();
    let level_record = level.record();
    let record_new_level = if level_record.is_set() {
        level_record.moves_in_time(' ')
    } else {
        "New Level!".to_string()
    };

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

    let stopwatch_housing = Container::size(Val::Px(152.0), Val::Auto).items_start();

    let level_name = SimpleText::medium(&format!("Level {}", level.name()), font);
    let record_new_level =
        SimpleText::small(&record_new_level, font).color(crate::theme::SECONDARY.into());
    let stopwatch = DynamicTextBuilder::small("Time: ", font).id(STOPWATCH_COUNTER_ID);
    let moves = DynamicTextBuilder::medium("Moves: ", font).id(MOVES_COUNTER_ID);
    let undos_left = DynamicTextBuilder::medium("Undos: ", font).id(UNDOS_COUNTER_ID);
    let undo = SimpleText::small("(Z) - Undo Movement", font).color(crate::theme::PRIMARY.into());
    let reload = SimpleText::small("(F5) - Reload Level", font).color(crate::theme::PRIMARY.into());
    let selection =
        SimpleText::small("(ESC) - Level Selection", font).color(crate::theme::PRIMARY.into());

    commands.spawn((
        root,
        children![
            (
                top,
                children![
                    (top_left, children![level_name, record_new_level]),
                    (
                        top_right,
                        children![
                            moves.build(),
                            (stopwatch_housing, children![stopwatch.build()])
                        ]
                    )
                ]
            ),
            (
                bottom,
                children![
                    (bottom_left, children![reload, selection]),
                    (bottom_right, children![undos_left.build(), undo])
                ]
            )
        ],
    ));
}
