use bevy::prelude::*;

use crate::resources::prelude::Colors;

use super::GameText;

pub struct EmbossedText {
    bundle: TextBundle,
}

impl Default for EmbossedText {
    fn default() -> EmbossedText {
        let style = TextStyle {
            font: default(),
            font_size: EmbossedText::SIZE_MEDIUM,
            color: Colors::LIGHT,
        };
        EmbossedText {
            bundle: TextBundle::from_section(String::new(), style)
                .with_text_alignment(TextAlignment::CENTER),
        }
    }
}

impl GameText for EmbossedText {
    fn text_bundle(&mut self) -> &mut TextBundle {
        &mut self.bundle
    }

    fn spawn(self, parent: &mut ChildBuilder) {
        let mut foreground = self.bundle;
        let mut background = foreground.clone();
        let relief = foreground.text.sections[0].style.font_size / EmbossedText::SIZE_SMALL;

        foreground.style.position_type = PositionType::Absolute;
        background.style.position.top = Val::Px(relief);
        background.style.position.left = Val::Px(relief);
        background.text.sections[0].style.color = Colors::DARK;

        parent.spawn_bundle(background);
        parent.spawn_bundle(foreground);
    }
}
