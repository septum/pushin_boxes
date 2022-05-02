use bevy::prelude::*;

#[derive(Component)]
pub struct CounterMarker {
    pub kind: CounterKind,
}

pub enum CounterKind {
    Moves,
    Undos,
}

impl CounterMarker {
    pub fn moves() -> CounterMarker {
        CounterMarker {
            kind: CounterKind::Moves,
        }
    }

    pub fn undos() -> CounterMarker {
        CounterMarker {
            kind: CounterKind::Undos,
        }
    }
}
