use bevy::prelude::*;

use crate::resources::prelude::*;

use super::GameText;

#[derive(Component, Default)]
pub struct DynamicTextData {
    pub id: usize,
}

pub struct DynamicText {
    data: DynamicTextData,
    text: Text,
    span: TextSpan,
    font: TextFont,
    color: TextColor,
    layout: TextLayout,
}

impl Default for DynamicText {
    fn default() -> DynamicText {
        let font = TextFont {
            font: Handle::default(),
            font_size: DynamicText::SIZE_MEDIUM,
            ..default()
        };
        let color = TextColor(Colors::LIGHT);
        let text = Text::new("");
        let span = TextSpan::new("");
        let layout = TextLayout::new_with_justify(JustifyText::Center);

        DynamicText {
            data: DynamicTextData::default(),
            text,
            span,
            font,
            color,
            layout,
        }
    }
}

impl GameText for DynamicText {
    fn spawn(self, parent: &mut ChildBuilder) {
        parent
            .spawn((self.text, self.font.clone(), self.color, self.layout, self.data))
            .with_child((self.span, self.font, self.color, self.layout));
    }

    fn get_text_color(&mut self) -> &mut TextColor {
        &mut self.color
    }

    fn get_text_font(&mut self) -> &mut TextFont {
        &mut self.font
    }

    fn get_text(&mut self) -> &mut Text {
        &mut self.text
    }
}

impl DynamicText {
    pub fn id(&mut self, id: usize) -> &mut DynamicText {
        self.data.id = id;
        self
    }

    pub fn dynamic_text_value<S: Into<String> + Clone>(&mut self, text: S) -> &mut DynamicText {
        self.span.0 = text.into();
        self
    }
}
