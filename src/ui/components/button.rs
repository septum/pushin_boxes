use bevy::prelude::*;

use crate::ui::components;

#[derive(Clone)]
pub struct Button {
    pub bundle: ButtonBundle,
    pub child: components::Text,
}

impl Button {
    pub fn new(child: components::Text, style: Style, color: UiColor) -> Button {
        Button {
            bundle: ButtonBundle {
                style,
                color,
                ..Default::default()
            },
            child,
        }
    }

    pub fn spawn(self, parent: &mut ChildBuilder) {
        parent.spawn_bundle(self.bundle).with_children(|parent| {
            self.child.spawn(parent);
        });
    }
}
