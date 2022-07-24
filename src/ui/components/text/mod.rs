mod dynamic;
mod embossed;
mod simple;

use bevy::prelude::*;

pub use dynamic::DynamicText;
pub use embossed::EmbossedText;
pub use simple::SimpleText;

const ALIGNMENT: TextAlignment = TextAlignment {
    vertical: VerticalAlign::Center,
    horizontal: HorizontalAlign::Center,
};

#[derive(Component)]
pub struct TextMarker {
    pub kind: TextKind,
}

pub enum TextKind {
    Moves,
    Undos,
    Stopwatch,
}

impl TextMarker {
    #[must_use]
    pub fn moves() -> TextMarker {
        TextMarker {
            kind: TextKind::Moves,
        }
    }

    #[must_use]
    pub fn undos() -> TextMarker {
        TextMarker {
            kind: TextKind::Undos,
        }
    }

    #[must_use]
    pub fn stopwatch() -> TextMarker {
        TextMarker {
            kind: TextKind::Stopwatch,
        }
    }
}
