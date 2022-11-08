pub enum DirectionInput {
    Up,
    Down,
    Left,
    Right,
}

pub struct DirectionInputEvent {
    pub value: DirectionInput,
}

impl DirectionInputEvent {
    pub fn up() -> DirectionInputEvent {
        DirectionInputEvent {
            value: DirectionInput::Up,
        }
    }

    pub fn down() -> DirectionInputEvent {
        DirectionInputEvent {
            value: DirectionInput::Down,
        }
    }

    pub fn left() -> DirectionInputEvent {
        DirectionInputEvent {
            value: DirectionInput::Left,
        }
    }

    pub fn right() -> DirectionInputEvent {
        DirectionInputEvent {
            value: DirectionInput::Right,
        }
    }
}
