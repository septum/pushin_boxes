use bevy::prelude::*;
use uuid::Uuid;

use super::SimpleText;

pub enum LevelKind {
    Stock(usize),
    Custom(Uuid),
}

pub enum ButtonKind {
    Play,
    Editor,
    Options,
    Quit,
    Levels,
    Level(LevelKind),
}

#[derive(Component)]
pub struct ButtonMarker {
    pub kind: ButtonKind,
}

impl ButtonMarker {
    pub fn new(kind: ButtonKind) -> ButtonMarker {
        ButtonMarker { kind }
    }
}

pub struct Button {
    pub bundle: ButtonBundle,
    pub child: SimpleText,
}

impl Button {
    pub fn new(child: SimpleText, style: Style, color: UiColor) -> Button {
        Button {
            bundle: ButtonBundle {
                style,
                color,
                ..Default::default()
            },
            child,
        }
    }

    pub fn spawn(self, parent: &mut ChildBuilder, marker: impl Component) {
        parent
            .spawn_bundle(self.bundle)
            .with_children(|parent| {
                self.child.spawn(parent);
            })
            .insert(marker);
    }
}
