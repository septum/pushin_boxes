use bevy::prelude::*;
use uuid::Uuid;

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

    pub fn play() -> ButtonMarker {
        ButtonMarker::new(ButtonKind::Play)
    }

    pub fn editor() -> ButtonMarker {
        ButtonMarker::new(ButtonKind::Editor)
    }

    pub fn options() -> ButtonMarker {
        ButtonMarker::new(ButtonKind::Options)
    }

    pub fn quit() -> ButtonMarker {
        ButtonMarker::new(ButtonKind::Quit)
    }

    pub fn levels() -> ButtonMarker {
        ButtonMarker::new(ButtonKind::Levels)
    }

    pub fn custom_level(uuid: Uuid) -> ButtonMarker {
        ButtonMarker::new(ButtonKind::Level(LevelKind::Custom(uuid)))
    }

    pub fn stock_level(index: usize) -> ButtonMarker {
        ButtonMarker::new(ButtonKind::Level(LevelKind::Stock(index)))
    }
}
