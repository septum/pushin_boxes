use bevy::prelude::*;

use crate::resources::prelude::Colors;

use super::ALIGNMENT;

pub struct SimpleText {
    pub bundle: TextBundle,
}

impl Default for SimpleText {
    fn default() -> SimpleText {
        let value = "";
        let style = TextStyle {
            font_size: 42.0,
            color: Colors::LIGHT,
            font: Handle::default(),
        };

        SimpleText {
            bundle: TextBundle {
                text: Text::with_section(value, style, ALIGNMENT),
                ..Default::default()
            },
        }
    }
}

impl SimpleText {
    pub fn small<S: Into<String>>(value: S, font: &Handle<Font>) -> SimpleText {
        let mut simple = SimpleText::default();

        simple.bundle.text.sections[0].value = value.into();
        simple.bundle.text.sections[0].style.font = font.clone();
        simple.bundle.text.sections[0].style.font_size = 21.0;

        simple
    }

    pub fn medium<S: Into<String>>(value: S, font: &Handle<Font>) -> SimpleText {
        let mut simple = SimpleText::default();

        simple.bundle.text.sections[0].value = value.into();
        simple.bundle.text.sections[0].style.font = font.clone();

        simple
    }

    pub fn big<S: Into<String>>(value: S, font: &Handle<Font>) -> SimpleText {
        let mut simple = SimpleText::default();

        simple.bundle.text.sections[0].value = value.into();
        simple.bundle.text.sections[0].style.font = font.clone();
        simple.bundle.text.sections[0].style.font_size = 84.0;

        simple
    }

    pub fn value<S: Into<String>>(&mut self, value: S) -> &mut SimpleText {
        self.bundle.text.sections[0].value = value.into();
        self
    }

    pub fn size(&mut self, font_size: f32) -> &mut SimpleText {
        self.bundle.text.sections[0].style.font_size = font_size;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut SimpleText {
        self.bundle.text.sections[0].style.color = color;
        self
    }

    pub fn font(&mut self, font: &Handle<Font>) -> &mut SimpleText {
        self.bundle.text.sections[0].style.font = font.clone();
        self
    }

    pub fn top_position(&mut self, position: f32) -> &mut SimpleText {
        self.bundle.style.position.top = Val::Px(position);
        self
    }

    pub fn spawn(self, parent: &mut ChildBuilder) {
        parent.spawn_bundle(self.bundle);
    }
}
