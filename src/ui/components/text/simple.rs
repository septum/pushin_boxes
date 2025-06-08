use bevy::prelude::*;

use crate::resources::prelude::*;

use super::GameText;

pub struct SimpleText {
    bundle: TextBundle,
}

impl Default for SimpleText {
    fn default() -> SimpleText {
        let style = TextStyle {
            font: Handle::default(),
            font_size: SimpleText::SIZE_MEDIUM,
            color: Colors::LIGHT,
        };
        SimpleText {
            bundle: TextBundle::from_section(String::new(), style)
                .with_text_justify(JustifyText::Center),
        }
    }
}

impl GameText for SimpleText {
    fn text_bundle(&mut self) -> &mut TextBundle {
        &mut self.bundle
    }

    fn spawn(self, parent: &mut ChildBuilder) {
        parent.spawn(self.bundle);
    }
}
