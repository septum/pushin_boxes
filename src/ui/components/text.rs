use bevy::prelude::*;

use crate::resources::Colors;

const ALIGNMENT: TextAlignment = TextAlignment {
    vertical: VerticalAlign::Center,
    horizontal: HorizontalAlign::Center,
};

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
    pub foreground: TextBundle,
    pub background: TextBundle,
}

impl EmbossedText {
    pub fn new(text: String, relief: f32, style: TextStyle) -> EmbossedText {
        let foreground = TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            text: Text::with_section(text.clone(), style.clone(), ALIGNMENT),
            ..Default::default()
        };

        let background = TextBundle {
            style: Style {
                position: Rect {
                    top: Val::Px(relief),
                    left: Val::Px(relief),
                    ..Default::default()
                },
                position_type: PositionType::Relative,
                ..Default::default()
            },
            text: Text::with_section(
                text,
                TextStyle {
                    color: Colors::DARK,
                    ..style
                },
                ALIGNMENT,
            ),
            ..Default::default()
        };

        EmbossedText {
            foreground,
            background,
        }
    }

    pub fn spawn(self, parent: &mut ChildBuilder) {
        parent.spawn_bundle(self.background);
        parent.spawn_bundle(self.foreground);
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
