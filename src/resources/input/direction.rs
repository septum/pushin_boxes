pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct DirectionEvent {
    pub value: Direction,
}

impl DirectionEvent {
    pub fn up() -> DirectionEvent {
        DirectionEvent {
            value: Direction::Up,
        }
    }

    pub fn down() -> DirectionEvent {
        DirectionEvent {
            value: Direction::Down,
        }
    }

    pub fn left() -> DirectionEvent {
        DirectionEvent {
            value: Direction::Left,
        }
    }

    pub fn right() -> DirectionEvent {
        DirectionEvent {
            value: Direction::Right,
        }
    }
}
