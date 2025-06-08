use bevy::prelude::*;

use crate::resources::prelude::*;

use super::GameText;

pub struct EmbossedText {
    bundle: TextBundle,
}

impl Default for EmbossedText {
    fn default() -> EmbossedText {
        let style = TextStyle {
            font: Handle::default(),
            font_size: EmbossedText::SIZE_MEDIUM,
            color: Colors::LIGHT,
        };
        EmbossedText {
            bundle: TextBundle::from_section(String::new(), style)
                .with_text_justify(JustifyText::Center),
        }
    }
}

impl GameText for EmbossedText {
    fn text_bundle(&mut self) -> &mut TextBundle {
        &mut self.bundle
    }

    fn spawn(self, parent: &mut ChildBuilder) {
        let text_clone = self.bundle.text.clone();
        let mut foreground = self.bundle;
        let mut background = TextBundle {
            text: text_clone,
            ..default()
        };
        let relief = foreground.text.sections[0].style.font_size / EmbossedText::SIZE_SMALL;

        foreground.style.position_type = PositionType::Absolute;
        background.style.top = Val::Px(relief);
        background.style.left = Val::Px(relief);
        background.text.sections[0].style.color = Colors::DARK;

        parent.spawn(background);
        parent.spawn(foreground);
    }
}
