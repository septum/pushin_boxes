use bevy::prelude::*;

use crate::{
    config::MAX_TOTAL_LEVELS,
    level::{Level, LevelTag},
    resources::{AssetsHandles, Colors, SaveFile},
    ui,
};

use super::CleanupMarker;

pub fn spawn(commands: &mut Commands, assets: &AssetsHandles, level: &Level, save_file: &SaveFile) {
    let last_stock_level = match level.tag {
        LevelTag::Stock(index) => index + 1 == MAX_TOTAL_LEVELS,
        _ => false,
    };
    let overlay = ui::Overlay::new();

    let title_housing_height = if last_stock_level {
        Val::Px(196.0)
    } else {
        Val::Px(98.0)
    };
    let title_housing = ui::Housing::new(Val::Percent(100.0), title_housing_height);
    let any_key_housing = ui::Housing::new(Val::Percent(100.0), Val::Px(32.0));

    let new_record = level.record == 0 || level.moves < level.record;
    let record_or_empty_text = if last_stock_level {
        format!("FINAL RECORD: {}", save_file.final_stock_levels_record())
    } else if new_record {
        format!("NEW RECORD: {}", level.moves)
    } else {
        String::new()
    };
    let record_or_empty = ui::SimpleText::new(
        record_or_empty_text,
        TextStyle {
            font_size: 40.0,
            color: Colors::SECONDARY,
            font: assets.fonts.fredoka.clone(),
        },
    );

    let title_text = if last_stock_level {
        "Thank you\nfor playing!".to_string()
    } else {
        if let LevelTag::Test(_) = level.tag {
            "Level Saved!".to_string()
        } else {
            "You Win!".to_string()
        }
    };
    let title = ui::EmbossedText::new(
        title_text,
        4.0,
        TextStyle {
            font_size: 96.0,
            color: Colors::PRIMARY,
            font: assets.fonts.fredoka.clone(),
        },
    );

    let press_space = ui::EmbossedText::new(
        "Press [SPACE] to continue".to_string(),
        2.0,
        TextStyle {
            font_size: 24.0,
            color: Colors::PRIMARY,
            font: assets.fonts.fredoka.clone(),
        },
    );

    assets.images.spawn_background(commands, CleanupMarker);

    overlay.spawn(commands, CleanupMarker, |parent| {
        record_or_empty.spawn(parent);
        title_housing.spawn(parent, |parent| {
            title.spawn(parent);
        });
        any_key_housing.spawn(parent, |parent| {
            press_space.spawn(parent);
        });
    });
}
