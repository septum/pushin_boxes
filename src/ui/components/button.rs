use bevy::prelude::*;

use crate::{resources::prelude::Colors, ui::ButtonMarker};

use super::text::SimpleText;

pub struct ActionButton {
    pub bundle: ButtonBundle,
    pub child: SimpleText,
}

impl Default for ActionButton {
    fn default() -> ActionButton {
        let style = Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Percent(100.0), Val::Px(50.0)),
            ..Default::default()
        };
        let child = SimpleText::default();

        ActionButton {
            bundle: ButtonBundle {
                style,
                color: Colors::PRIMARY.into(),
                ..Default::default()
            },
            child,
        }
    }
}

impl ActionButton {
    pub fn full<S: Into<String>>(value: S, font: &Handle<Font>) -> ActionButton {
        let mut child = SimpleText::medium(value, font);
        child.color(Colors::DARK);

        ActionButton {
            child,
            ..Default::default()
        }
    }

    pub fn square<S: Into<String>>(value: S, font: &Handle<Font>) -> ActionButton {
        let mut child = SimpleText::medium(value, font);
        child.color(Colors::DARK);

        let mut button = ActionButton::default();
        button.width(Val::Px(50.0));
        button.child = child;

        button
    }

    pub fn width(&mut self, width: Val) -> &mut ActionButton {
        self.bundle.style.size.width = width;
        self
    }

    pub fn height(&mut self, height: Val) -> &mut ActionButton {
        self.bundle.style.size.height = height;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut ActionButton {
        self.bundle.color = color.into();
        self
    }

    pub fn child<S: Into<String>>(&mut self, value: S) -> &mut ActionButton {
        self.child.bundle.text.sections[0].value = value.into();
        self
    }

    pub fn spawn(self, parent: &mut ChildBuilder, marker: ButtonMarker) {
        parent
            .spawn_bundle(self.bundle)
            .with_children(|parent| self.child.spawn(parent))
            .insert(marker);
    }
}
