use bevy::prelude::*;

use crate::resources::prelude::Colors;

use super::ALIGNMENT;

pub struct EmbossedText {
    pub foreground: TextBundle,
    pub background: TextBundle,
}

impl Default for EmbossedText {
    fn default() -> EmbossedText {
        let relief = 3.0;
        let style = TextStyle {
            font_size: 42.0,
            color: Colors::PRIMARY,
            font: Default::default(),
        };
        let foreground = TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            text: Text::with_section("", style.clone(), ALIGNMENT),
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
                "",
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
}

impl EmbossedText {
    pub fn small<S: Into<String>>(value: S, font: &Handle<Font>) -> EmbossedText {
        let string = value.into();
        let mut embossed = EmbossedText::default();

        embossed.foreground.text.sections[0].value = string.clone();
        embossed.foreground.text.sections[0].style.font = font.clone();
        embossed.foreground.text.sections[0].style.font_size = 21.0;

        embossed.background.text.sections[0].value = string;
        embossed.background.text.sections[0].style.font = font.clone();
        embossed.background.text.sections[0].style.font_size = 21.0;
        embossed.background.style.position.top = Val::Px(2.0);
        embossed.background.style.position.left = Val::Px(2.0);

        embossed
    }

    pub fn medium<S: Into<String>>(value: S, font: &Handle<Font>) -> EmbossedText {
        let string = value.into();
        let mut embossed = EmbossedText::default();

        embossed.foreground.text.sections[0].value = string.clone();
        embossed.foreground.text.sections[0].style.font = font.clone();

        embossed.background.text.sections[0].value = string;
        embossed.background.text.sections[0].style.font = font.clone();

        embossed
    }

    pub fn big<S: Into<String>>(value: S, font: &Handle<Font>) -> EmbossedText {
        let string = value.into();
        let mut embossed = EmbossedText::default();

        embossed.foreground.text.sections[0].value = string.clone();
        embossed.foreground.text.sections[0].style.font = font.clone();
        embossed.foreground.text.sections[0].style.font_size = 84.0;

        embossed.background.text.sections[0].value = string;
        embossed.background.text.sections[0].style.font = font.clone();
        embossed.background.text.sections[0].style.font_size = 84.0;
        embossed.background.style.position.top = Val::Px(4.0);
        embossed.background.style.position.left = Val::Px(4.0);

        embossed
    }

    pub fn value<S: Into<String>>(&mut self, value: S) -> &mut EmbossedText {
        let string = value.into();
        self.foreground.text.sections[0].value = string.clone();
        self.background.text.sections[0].value = string;
        self
    }

    pub fn size(&mut self, font_size: f32) -> &mut EmbossedText {
        self.foreground.text.sections[0].style.font_size = font_size;
        self.background.text.sections[0].style.font_size = font_size;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut EmbossedText {
        self.foreground.text.sections[0].style.color = color;
        self.background.text.sections[0].style.color = color;
        self
    }

    pub fn relief(&mut self, relief: f32) -> &mut EmbossedText {
        self.background.style.position.top = Val::Px(relief);
        self.background.style.position.left = Val::Px(relief);
        self
    }

    pub fn font(&mut self, font: &Handle<Font>) -> &mut EmbossedText {
        self.foreground.text.sections[0].style.font = font.clone();
        self.background.text.sections[0].style.font = font.clone();
        self
    }

    pub fn spawn(self, parent: &mut ChildBuilder) {
        parent.spawn_bundle(self.background);
        parent.spawn_bundle(self.foreground);
    }
}
