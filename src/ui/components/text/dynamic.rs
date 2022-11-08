use bevy::prelude::*;

use crate::resources::prelude::*;

use super::GameText;

#[derive(Component, Default)]
pub struct DynamicTextMarker {
    pub name: String,
}

pub struct DynamicText {
    marker: DynamicTextMarker,
    bundle: TextBundle,
}

impl Default for DynamicText {
    fn default() -> DynamicText {
        let section = TextSection {
            value: String::new(),
            style: TextStyle {
                font: default(),
                font_size: DynamicText::SIZE_MEDIUM,
                color: Colors::LIGHT,
            },
        };
        DynamicText {
            marker: default(),
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
        parent.spawn_bundle(self.bundle).insert(self.marker);
    }
}

impl DynamicText {
    pub fn marker(&mut self, name: &str) -> &mut DynamicText {
        self.marker.name = name.to_string();
        self
    }
}
