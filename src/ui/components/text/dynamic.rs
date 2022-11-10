use bevy::prelude::*;

use crate::resources::prelude::*;

use super::GameText;

#[derive(Component, Default)]
pub struct DynamicTextData {
    pub id: usize,
}

pub struct DynamicText {
    data: DynamicTextData,
    bundle: TextBundle,
}

impl Default for DynamicText {
    fn default() -> DynamicText {
        let section = TextSection {
            value: String::new(),
            style: TextStyle {
                font: Handle::default(),
                font_size: DynamicText::SIZE_MEDIUM,
                color: Colors::LIGHT,
            },
        };
        DynamicText {
            data: DynamicTextData::default(),
            bundle: TextBundle::from_sections(vec![section; 2])
                .with_text_alignment(TextAlignment::CENTER),
        }
    }
}

impl GameText for DynamicText {
    fn text_bundle(&mut self) -> &mut TextBundle {
        &mut self.bundle
    }

    fn spawn(self, parent: &mut ChildBuilder) {
        parent.spawn_bundle(self.bundle).insert(self.data);
    }
}

impl DynamicText {
    pub fn id(&mut self, id: usize) -> &mut DynamicText {
        self.data.id = id;
        self
    }
}
