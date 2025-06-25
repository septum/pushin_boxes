use bevy::prelude::*;

use crate::{Colors, GameText};

#[derive(Bundle)]
pub struct SimpleText {
    text: Text,
    font: TextFont,
    color: TextColor,
    layout: TextLayout,
}

impl Default for SimpleText {
    fn default() -> SimpleText {
        let font = TextFont {
            font: Handle::default(),
            font_size: SimpleText::SIZE_MEDIUM,
            ..default()
        };
        let color = TextColor(Colors::LIGHT);
        let text = Text::new("");
        let layout = TextLayout::new_with_justify(JustifyText::Center);

        SimpleText {
            text,
            font,
            color,
            layout,
        }
    }
}

impl GameText for SimpleText {
    fn spawn(self, parent: &mut ChildSpawnerCommands) {
        parent.spawn(self);
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
