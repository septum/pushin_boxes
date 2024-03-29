use bevy::prelude::*;

use crate::resources::prelude::*;

use super::text::{EmbossedText, GameText};

#[derive(Component, Default)]
pub struct GameButtonData {
    pub id: usize,
    pub selected: bool,
    pub payload: Option<String>,
}

pub struct GameButton {
    bundle: ButtonBundle,
    child: EmbossedText,
    data: GameButtonData,
}

impl Default for GameButton {
    fn default() -> GameButton {
        let style = Style {
            size: Size::new(Val::Px(400.0), Val::Px(60.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        GameButton {
            bundle: ButtonBundle {
                background_color: Colors::TRANSPARENT.into(),
                style,
                ..default()
            },
            child: EmbossedText::default(),
            data: GameButtonData::default(),
        }
    }
}

impl GameButton {
    pub fn new<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> GameButton {
        GameButton {
            child: EmbossedText::medium(value, font),
            ..default()
        }
    }

    pub fn square<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> GameButton {
        let mut button = Self::default();
        button.bundle.style.size.width = Val::Px(60.0);
        button.bundle.style.size.height = Val::Px(60.0);
        button.child = EmbossedText::medium(value, font);
        button
    }

    pub fn id(&mut self, id: usize) -> &mut GameButton {
        self.data.id = id;
        self
    }

    pub fn selected(&mut self) -> &mut GameButton {
        self.data.selected = true;
        self.bundle.background_color = Colors::PRIMARY_DARK.into();
        self
    }

    pub fn payload(&mut self, payload: String) -> &mut GameButton {
        self.data.payload = Some(payload);
        self
    }

    pub fn spawn(self, parent: &mut ChildBuilder) {
        parent
            .spawn(self.bundle)
            .with_children(|parent| self.child.spawn(parent))
            .insert(self.data);
    }
}
