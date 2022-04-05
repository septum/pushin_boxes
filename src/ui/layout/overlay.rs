use bevy::prelude::*;

use crate::assets::Colors;

pub struct Overlay {
    pub bundle: NodeBundle,
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
