use bevy::prelude::*;

use crate::resources::prelude::*;

pub struct Overlay {
    pub bundle: NodeBundle,
}

impl Default for Overlay {
    fn default() -> Overlay {
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
}

impl Overlay {
    pub fn new() -> Overlay {
        Overlay::default()
    }

    pub fn flex_direction(&mut self, flex_direction: FlexDirection) -> &mut Overlay {
        self.bundle.style.flex_direction = flex_direction;
        self
    }

    pub fn justify_content(&mut self, justify_content: JustifyContent) -> &mut Overlay {
        self.bundle.style.justify_content = justify_content;
        self
    }

    pub fn align_items(&mut self, align_items: AlignItems) -> &mut Overlay {
        self.bundle.style.align_items = align_items;
        self
    }

    pub fn padding(&mut self, padding: Rect<Val>) -> &mut Overlay {
        self.bundle.style.padding = padding;
        self
    }

    pub fn spawn(
        self,
        commands: &mut Commands,
        children: impl FnOnce(&mut ChildBuilder),
        marker: impl Component,
    ) {
        commands
            .spawn_bundle(self.bundle)
            .with_children(children)
            .insert(marker);
    }
}
