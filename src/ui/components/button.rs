use bevy::prelude::*;

use crate::resources::prelude::Colors;

use super::text::EmbossedText;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum LevelKind {
    Stock(usize),
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum ButtonKind {
    Play,
    Quit,
    Instructions,
    Level(LevelKind),
}

#[derive(Component)]
pub struct ButtonMarker {
    pub kind: ButtonKind,
    pub selected: bool,
}

impl ButtonMarker {
    #[must_use]
    pub fn new(kind: ButtonKind, selected: bool) -> ButtonMarker {
        ButtonMarker { kind, selected }
    }

    #[must_use]
    pub fn play(selected: bool) -> ButtonMarker {
        ButtonMarker::new(ButtonKind::Play, selected)
    }

    #[must_use]
    pub fn instructions(selected: bool) -> ButtonMarker {
        ButtonMarker::new(ButtonKind::Instructions, selected)
    }

    #[must_use]
    pub fn quit(selected: bool) -> ButtonMarker {
        ButtonMarker::new(ButtonKind::Quit, selected)
    }

    #[must_use]
    pub fn stock_level(index: usize, selected: bool) -> ButtonMarker {
        ButtonMarker::new(ButtonKind::Level(LevelKind::Stock(index)), selected)
    }
}

pub struct ActionButton {
    pub bundle: ButtonBundle,
    pub child: EmbossedText,
}

impl Default for ActionButton {
    fn default() -> ActionButton {
        let style = Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Percent(100.0), Val::Px(50.0)),
            ..Default::default()
        };
        let child = EmbossedText::default();

        ActionButton {
            bundle: ButtonBundle {
                style,
                color: Colors::TRANSPARENT.into(),
                ..Default::default()
            },
            child,
        }
    }
}

impl ActionButton {
    pub fn new<S: Into<String>>(value: S, font: &Handle<Font>, size: Size<Val>) -> ActionButton {
        let mut child = EmbossedText::medium(value, font);
        child.foreground_color(Colors::LIGHT);
        child.background_color(Colors::DARK);

        let mut button = ActionButton::default();
        button.bundle.style.size = size;
        button.child = child;

        button
    }

    pub fn full<S: Into<String>>(value: S, font: &Handle<Font>) -> ActionButton {
        let mut child = EmbossedText::medium(value, font);
        child.foreground_color(Colors::LIGHT);
        child.background_color(Colors::DARK);

        ActionButton {
            child,
            ..Default::default()
        }
    }

    pub fn square<S: Into<String>>(value: S, font: &Handle<Font>) -> ActionButton {
        let mut child = EmbossedText::medium(value, font);
        child.foreground_color(Colors::LIGHT);
        child.background_color(Colors::DARK);

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

    pub fn spawn(self, parent: &mut ChildBuilder, marker: ButtonMarker) {
        parent
            .spawn_bundle(self.bundle)
            .with_children(|parent| self.child.spawn(parent))
            .insert(marker);
    }
}
