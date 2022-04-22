use bevy::prelude::*;

use crate::{
    resources::{AssetsHandles, Colors, SaveFile},
    state::SelectionKind,
    ui::{self, ButtonKind, ButtonMarker, LevelKind},
};

use super::CleanupMarker;

pub fn spawn(
    commands: &mut Commands,
    assets: &AssetsHandles,
    save_file: &SaveFile,
    selection_kind: &SelectionKind,
) {
    let stock_levels = matches!(selection_kind, SelectionKind::Stock);

    let overlay = ui::Overlay::new();

    let mut top = ui::Housing::new(Val::Percent(100.0), Val::Percent(10.0));
    let mut bottom = ui::Housing::new(Val::Percent(100.0), Val::Percent(90.0));

    let title = ui::EmbossedText::new(
        "Select a Level".to_string(),
        2.0,
        TextStyle {
            font_size: 42.0,
            color: Colors::PRIMARY,
            font: assets.fonts.fredoka.clone(),
        },
    );

    let levels = ui::Button::new(
        ui::SimpleText::new(
            if stock_levels {
                "Custom Levels".to_string()
            } else {
                "Stock Levels".to_string()
            },
            TextStyle {
                font_size: 28.0,
                color: Colors::DARK,
                font: assets.fonts.fredoka.clone(),
            },
        ),
        Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Px(200.0), Val::Px(36.0)),
            ..Default::default()
        },
        Colors::PRIMARY.into(),
    );

    top.set_flex_direction(FlexDirection::Row);
    top.set_justify_content(JustifyContent::SpaceBetween);
    top.set_padding(Rect {
        left: Val::Px(43.0),
        right: Val::Px(43.0),
        ..Default::default()
    });
    bottom.set_flex_wrap(FlexWrap::WrapReverse);
    bottom.set_flex_direction(FlexDirection::Row);
    bottom.set_justify_content(JustifyContent::FlexStart);
    bottom.set_align_items(AlignItems::FlexStart);
    bottom.set_align_content(AlignContent::FlexStart);

    assets.images.spawn_background(commands, CleanupMarker);

    overlay.spawn(commands, CleanupMarker, |parent| {
        top.spawn(parent, |parent| {
            title.spawn(parent);
            levels.spawn(parent, ButtonMarker::new(ButtonKind::Levels));
        });
        bottom.spawn(parent, |parent| {
            // TODO: DRY this
            if stock_levels {
                for (index, record) in save_file.stock.iter().enumerate() {
                    let housing = ui::Housing::new(Val::Percent(25.0), Val::Percent(25.0));
                    let button = ui::Button::new(
                        ui::SimpleText::new(
                            format!("{}", index + 1),
                            TextStyle {
                                font_size: 49.0,
                                color: Colors::DARK,
                                font: assets.fonts.fredoka.clone(),
                            },
                        ),
                        Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            size: Size::new(Val::Px(56.0), Val::Px(56.0)),
                            ..Default::default()
                        },
                        Colors::PRIMARY.into(),
                    );
                    let mut record_or_new_level = ui::SimpleText::new(
                        if *record > 0 {
                            format!("Record: {}", record)
                        } else {
                            "New Level!".to_string()
                        },
                        TextStyle {
                            font: assets.fonts.fredoka.clone(),
                            font_size: 19.0,
                            color: Colors::LIGHT,
                        },
                    );

                    record_or_new_level.set_node_style(Style {
                        position: Rect {
                            top: Val::Px(3.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    });

                    housing.spawn(parent, |parent| {
                        button.spawn(
                            parent,
                            ButtonMarker::new(ButtonKind::Level(LevelKind::Stock(index))),
                        );
                        record_or_new_level.spawn(parent);
                    });
                }
            } else {
                for (uuid, record) in save_file.custom.iter() {
                    let housing = ui::Housing::new(Val::Percent(100.0), Val::Percent(20.0));
                    let button = ui::Button::new(
                        ui::SimpleText::new(
                            format!("{}", uuid),
                            TextStyle {
                                font_size: 21.0,
                                color: Colors::DARK,
                                font: assets.fonts.fredoka.clone(),
                            },
                        ),
                        Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            size: Size::new(Val::Px(560.0), Val::Px(40.0)),
                            ..Default::default()
                        },
                        Colors::PRIMARY.into(),
                    );
                    let mut record_or_new_level = ui::SimpleText::new(
                        if *record > 0 {
                            format!("Record: {}", record)
                        } else {
                            "New Level!".to_string()
                        },
                        TextStyle {
                            font: assets.fonts.fredoka.clone(),
                            font_size: 19.0,
                            color: Colors::LIGHT,
                        },
                    );

                    record_or_new_level.set_node_style(Style {
                        position: Rect {
                            top: Val::Px(3.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    });

                    housing.spawn(parent, |parent| {
                        button.spawn(
                            parent,
                            ButtonMarker::new(ButtonKind::Level(LevelKind::Custom(*uuid))),
                        );
                        record_or_new_level.spawn(parent);
                    });
                }
            }
        });
    });
}
