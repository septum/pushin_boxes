use bevy::prelude::*;

use crate::assets::Colors;

pub struct Overlay {
    pub bundle: NodeBundle,
}

impl Default for Overlay {
    fn default() -> Overlay {
        Overlay::new()
    }
}

impl Overlay {
    pub fn new() -> Overlay {
        Overlay {
            bundle: NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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

    pub fn set_flex_direction(&mut self, flex_direction: FlexDirection) {
        self.bundle.style.flex_direction = flex_direction;
    }

    pub fn set_justify_content(&mut self, justify_content: JustifyContent) {
        self.bundle.style.justify_content = justify_content;
    }

    pub fn set_align_items(&mut self, align_items: AlignItems) {
        self.bundle.style.align_items = align_items;
    }

    pub fn set_padding(&mut self, padding: Rect<Val>) {
        self.bundle.style.padding = padding;
    }

    pub fn spawn(
        self,
        commands: &mut Commands,
        marker: impl Component,
        children: impl FnOnce(&mut ChildBuilder),
    ) {
        commands
            .spawn_bundle(self.bundle)
            .with_children(children)
            .insert(marker);
    }
}
