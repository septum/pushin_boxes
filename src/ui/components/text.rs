use bevy::prelude::*;

use crate::{assets::Colors, ui::layout::Housing};

const ALIGNMENT: TextAlignment = TextAlignment {
    vertical: VerticalAlign::Center,
    horizontal: HorizontalAlign::Center,
};

const HOUSING_WIDTH_SCALING_FACTOR: f32 = 2.8;
const HOUSING_HEIGHT_SCALING_FACTOR: f32 = 2.0;

pub struct SimpleText {
    pub bundle: TextBundle,
}

impl SimpleText {
    pub fn new(text: String, style: TextStyle) -> SimpleText {
        SimpleText {
            bundle: TextBundle {
                text: Text::with_section(text, style, ALIGNMENT),
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
    pub fn new(text: String, relief_factor: f32, style: TextStyle) -> EmbossedText {
        let mut background_style = style.clone();
        background_style.color = Colors::DARK;

        let housing = Housing::new(
            Val::Px((style.font_size * HOUSING_WIDTH_SCALING_FACTOR) + relief_factor),
            Val::Px((style.font_size * HOUSING_HEIGHT_SCALING_FACTOR) + relief_factor),
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
            text: Text::with_section(text.clone(), style, ALIGNMENT),
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
            text: Text::with_section(text, background_style, ALIGNMENT),
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

pub struct DynamicText {
    pub bundle: TextBundle,
}

impl DynamicText {
    pub fn new(static_text: String, dynamic_text: String, style: TextStyle) -> DynamicText {
        DynamicText {
            bundle: TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: static_text,
                            style: style.clone(),
                        },
                        TextSection {
                            value: dynamic_text,
                            style,
                        },
                    ],
                    alignment: ALIGNMENT,
                },
                ..Default::default()
            },
        }
    }

    pub fn update(text: &mut Text, value: String) {
        text.sections[1].value = value;
    }

    pub fn set_node_style(&mut self, style: Style) {
        self.bundle.style = style;
    }

    pub fn spawn(self, parent: &mut ChildBuilder, marker: impl Component) {
        parent.spawn_bundle(self.bundle).insert(marker);
    }
}
