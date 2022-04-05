use bevy::prelude::*;

use crate::assets::Colors;

pub struct Housing {
    pub bundle: NodeBundle,
}

impl Housing {
    pub fn new(width: Val, height: Val) -> Housing {
        Housing {
            bundle: NodeBundle {
                style: Style {
                    size: Size::new(width, height),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: Colors::TRANSPARENT.into(),
                ..Default::default()
            },
        }
    }

    pub fn set_justify_content(&mut self, justify_content: JustifyContent) {
        self.bundle.style.justify_content = justify_content;
    }

    pub fn spawn(self, parent: &mut ChildBuilder, children: impl FnOnce(&mut ChildBuilder)) {
        parent.spawn_bundle(self.bundle).with_children(children);
    }
}
