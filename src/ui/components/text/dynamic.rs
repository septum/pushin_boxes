use bevy::prelude::*;

use crate::resources::prelude::Colors;

use super::ALIGNMENT;

pub struct DynamicText {
    pub bundle: TextBundle,
}

impl Default for DynamicText {
    fn default() -> DynamicText {
        let style = TextStyle {
            font_size: 36.0,
            color: Colors::LIGHT,
            font: Handle::default(),
        };

        DynamicText {
            bundle: TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: String::new(),
                            style: style.clone(),
                        },
                        TextSection {
                            value: String::new(),
                            style,
                        },
                    ],
                    alignment: ALIGNMENT,
                },
                ..Default::default()
            },
        }
    }
}

impl DynamicText {
    pub fn small<S: Into<String>>(static_value: S, font: &Handle<Font>) -> DynamicText {
        let mut dynamic = DynamicText::default();

        dynamic.bundle.text.sections[0].value = static_value.into();
        dynamic.bundle.text.sections[0].style.font = font.clone();
        dynamic.bundle.text.sections[0].style.font_size = 18.0;

        dynamic.bundle.text.sections[1].style.font = font.clone();
        dynamic.bundle.text.sections[1].style.font_size = 18.0;

        dynamic
    }

    pub fn medium<S: Into<String>>(static_value: S, font: &Handle<Font>) -> DynamicText {
        let mut dynamic = DynamicText::default();

        dynamic.bundle.text.sections[0].value = static_value.into();
        dynamic.bundle.text.sections[0].style.font = font.clone();
        dynamic.bundle.text.sections[1].style.font = font.clone();

        dynamic
    }

    pub fn big<S: Into<String>>(static_value: S, font: &Handle<Font>) -> DynamicText {
        let mut dynamic = DynamicText::default();

        dynamic.bundle.text.sections[0].value = static_value.into();
        dynamic.bundle.text.sections[0].style.font = font.clone();
        dynamic.bundle.text.sections[0].style.font_size = 108.0;

        dynamic.bundle.text.sections[1].style.font = font.clone();
        dynamic.bundle.text.sections[1].style.font_size = 108.0;

        dynamic
    }

    pub fn static_value<S: Into<String>>(&mut self, value: S) -> &mut DynamicText {
        self.bundle.text.sections[0].value = value.into();
        self
    }

    pub fn dynamic_value<S: Into<String>>(&mut self, value: S) -> &mut DynamicText {
        self.bundle.text.sections[1].value = value.into();
        self
    }

    pub fn size(&mut self, font_size: f32) -> &mut DynamicText {
        self.bundle.text.sections[0].style.font_size = font_size;
        self.bundle.text.sections[1].style.font_size = font_size;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut DynamicText {
        self.bundle.text.sections[0].style.color = color;
        self.bundle.text.sections[1].style.color = color;
        self
    }

    pub fn font(&mut self, font: &Handle<Font>) -> &mut DynamicText {
        self.bundle.text.sections[0].style.font = font.clone();
        self.bundle.text.sections[1].style.font = font.clone();
        self
    }

    pub fn spawn(self, parent: &mut ChildBuilder, marker: impl Component) {
        parent.spawn_bundle(self.bundle).insert(marker);
    }
}
