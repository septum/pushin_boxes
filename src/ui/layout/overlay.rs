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
    node: Node,
    background_color: BackgroundColor,
}

impl Default for Overlay {
    fn default() -> Overlay {
        Overlay {
            node: Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Colors::TRANSPARENT),
        }
    }
}

impl Overlay {
    pub fn extended() -> Overlay {
        let mut extended = Overlay::default();
        extended.node.justify_content = JustifyContent::SpaceBetween;
        extended.node.padding = EXTENDED_CONTAINER_PADDING;
        extended
    }

    pub fn spawn(self, commands: &mut Commands, children: impl FnOnce(&mut ChildBuilder)) {
        commands
            .spawn((self.node, self.background_color))
            .with_children(children)
            .insert(OverlayMarker);
    }
}
