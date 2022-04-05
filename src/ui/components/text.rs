use bevy::prelude::*;

use crate::{assets::Colors, ui::layout::Housing};

const ALIGNMENT: TextAlignment = TextAlignment {
    vertical: VerticalAlign::Center,
    horizontal: HorizontalAlign::Center,
};

#[derive(Clone)]
pub struct Text {
    pub bundle: TextBundle,
}

impl Text {
    pub fn new(text: String, style: TextStyle) -> Text {
        Text {
            bundle: TextBundle {
                text: bevy::text::Text::with_section(text, style, ALIGNMENT),
                ..Default::default()
            },
        }
    }

    pub fn set_node_style(&mut self, style: Style) {
        self.bundle.style = style;
    }

    pub fn spawn(self, parent: &mut ChildBuilder) {
        parent.spawn_bundle(self.bundle);
    }
}

pub struct EmbossedText {
    pub housing: Housing,
    pub foreground: TextBundle,
    pub background: TextBundle,
}

impl EmbossedText {
    pub fn new(text: String, style: TextStyle) -> EmbossedText {
        let mut background_style = style.clone();
        background_style.color = Colors::DARK;

        let housing = Housing::new(
            Val::Px((style.font_size * 2.8) + 8.0),
            Val::Px((style.font_size * 2.0) + 4.0),
        );

        let foreground = TextBundle {
            style: Style {
                position: Rect {
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            text: bevy::text::Text::with_section(text.clone(), style, ALIGNMENT),
            ..Default::default()
        };

        let background = TextBundle {
            style: Style {
                position: Rect {
                    bottom: Val::Px(0.0),
                    right: Val::Px(0.0),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            text: bevy::text::Text::with_section(text.clone(), background_style, ALIGNMENT),
            ..Default::default()
        };

        EmbossedText {
            housing,
            foreground,
            background,
        }
    }

    pub fn spawn(self, parent: &mut ChildBuilder) {
        self.housing.spawn(parent, |parent| {
            parent.spawn_bundle(self.background);
            parent.spawn_bundle(self.foreground);
        });
    }
}
