use bevy::prelude::*;

use crate::resources::prelude::*;

const EXTENDED_CONTAINER_PADDING: UiRect = UiRect {
    top: Val::Px(19.0),
    bottom: Val::Px(28.0),
    left: Val::Px(30.0),
    right: Val::Px(28.0),
};

#[derive(Component)]
pub struct OverlayMarker;

pub struct Overlay {
    bundle: NodeBundle,
}

impl Default for Overlay {
    fn default() -> Overlay {
        Overlay {
            bundle: NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Colors::TRANSPARENT.into(),
                ..default()
            },
        }
    }
}

impl Overlay {
    pub fn extended() -> Overlay {
        let mut extended = Overlay::default();
        extended.bundle.style.justify_content = JustifyContent::SpaceBetween;
        extended.bundle.style.padding = EXTENDED_CONTAINER_PADDING;
        extended
    }

    pub fn spawn(self, commands: &mut Commands, children: impl FnOnce(&mut ChildBuilder)) {
        commands
            .spawn(self.bundle)
            .with_children(children)
            .insert(OverlayMarker);
    }
}
