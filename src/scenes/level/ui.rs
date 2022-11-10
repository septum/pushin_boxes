use bevy::prelude::*;

use crate::{
    resources::prelude::*,
    ui::{Container, DynamicText, GameText, Overlay, SimpleText},
};

use super::{MOVES_COUNTER_ID, STOPWATCH_COUNTER_ID, UNDOS_COUNTER_ID};

pub fn spawn(mut commands: Commands, level: Res<Level>, fonts: Res<Fonts>) {
    let font = fonts.primary();
    let level_record = level.get_current_record();
    let record_new_level = if level_record.is_set() {
        level_record.moves_in_time(" ")
    } else {
        "New Level!".to_string()
    };

    let overlay = Overlay::extended();
    let mut top = Container::auto_height();
    let mut bottom = Container::auto_height();

    let mut top_left = Container::half();
    let mut top_right = Container::half();
    let mut bottom_left = Container::half();
    let mut bottom_right = Container::half();

    let mut stopwatch_housing = Container::auto_height_with_width(152.0);

    let level_name = SimpleText::medium(format!("Level {}", level.name()), font);
    let mut record_new_level = SimpleText::small(record_new_level, font);
    let mut stopwatch = DynamicText::small("Time: ", font);
    let mut moves = DynamicText::medium("Moves: ", font);
    let mut undos_left = DynamicText::medium("Undos: ", font);
    let mut undo = SimpleText::small("(U) - Undo Movement", font);
    let mut reload = SimpleText::small("(R) - Reload Level", font);
    let mut selection = SimpleText::small("(L) - Level Selection", font);

    top.row().justify_between();
    bottom.row().justify_between();

    top_left.justify_start().align_start();
    top_right.justify_start().align_end();
    bottom_left.justify_end().align_start();
    bottom_right.justify_end().align_end();

    stopwatch_housing.align_start();

    stopwatch.id(STOPWATCH_COUNTER_ID);
    moves.id(MOVES_COUNTER_ID);
    undos_left.id(UNDOS_COUNTER_ID);

    undo.primary();
    reload.primary();
    selection.primary();
    record_new_level.secondary();

    overlay.spawn(&mut commands, |parent| {
        top.spawn(parent, |parent| {
            top_left.spawn(parent, |parent| {
                level_name.spawn(parent);
                record_new_level.spawn(parent);
            });
            top_right.spawn(parent, |parent| {
                moves.spawn(parent);
                stopwatch_housing.spawn(parent, |parent| {
                    stopwatch.spawn(parent);
                });
            });
        });
        bottom.spawn(parent, |parent| {
            bottom_left.spawn(parent, |parent| {
                reload.spawn(parent);
                selection.spawn(parent);
            });
            bottom_right.spawn(parent, |parent| {
                undos_left.spawn(parent);
                undo.spawn(parent);
            });
        });
    });
}
