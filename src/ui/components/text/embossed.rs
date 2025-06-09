use bevy::prelude::*;

use crate::resources::prelude::*;

use super::GameText;

#[derive(Bundle, Clone)]
pub struct EmbossedText {
    text: Text,
    font: TextFont,
    color: TextColor,
    layout: TextLayout,
    node: Node,
}

impl Default for EmbossedText {
    fn default() -> EmbossedText {
        let font = TextFont {
            font: Handle::default(),
            font_size: EmbossedText::SIZE_MEDIUM,
            ..default()
        };
        let color = TextColor(Colors::LIGHT);
        let text = Text::new("");
        let layout = TextLayout::new_with_justify(JustifyText::Center);

        EmbossedText {
            text,
            font,
            color,
            layout,
            node: Node::default(),
        }
    }
}

impl GameText for EmbossedText {
    fn spawn(self, parent: &mut ChildSpawnerCommands) {
        let mut foreground = self.clone();
        let mut background = self;
        let relief = foreground.font.font_size / EmbossedText::SIZE_SMALL;

        foreground.node.position_type = PositionType::Absolute;
        background.node.top = Val::Px(relief);
        background.node.left = Val::Px(relief);
        background.color = TextColor(Colors::DARK);

        parent.spawn(background);
        parent.spawn(foreground);
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
