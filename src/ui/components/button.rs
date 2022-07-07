use bevy::prelude::*;

use crate::resources::prelude::Colors;

use super::text::SimpleText;

pub enum LevelKind {
    Stock(usize),
}

pub enum ButtonKind {
    Play,
    Quit,
    Level(LevelKind),
}

#[derive(Component)]
pub struct ButtonMarker {
    pub kind: ButtonKind,
}

impl ButtonMarker {
    #[must_use]
    pub fn new(kind: ButtonKind) -> ButtonMarker {
        ButtonMarker { kind }
    }

    #[must_use]
    pub fn play() -> ButtonMarker {
        ButtonMarker::new(ButtonKind::Play)
    }

    #[must_use]
    pub fn quit() -> ButtonMarker {
        ButtonMarker::new(ButtonKind::Quit)
    }

    #[must_use]
    pub fn stock_level(index: usize) -> ButtonMarker {
        ButtonMarker::new(ButtonKind::Level(LevelKind::Stock(index)))
    }
}

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
    pub fn new<S: Into<String>>(value: S, font: &Handle<Font>, size: Size<Val>) -> ActionButton {
        let mut child = SimpleText::medium(value, font);
        child.color(Colors::DARK);

        let mut button = ActionButton::default();
        button.bundle.style.size = size;
        button.child = child;

        button
    }

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

    pub fn font_size(&mut self, font_size: f32) -> &mut ActionButton {
        self.child.bundle.text.sections[0].style.font_size = font_size;
        self
    }

    pub fn spawn(self, parent: &mut ChildBuilder, marker: ButtonMarker) {
        parent
            .spawn_bundle(self.bundle)
            .with_children(|parent| self.child.spawn(parent))
            .insert(marker);
    }
}
