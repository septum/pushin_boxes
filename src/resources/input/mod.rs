mod action;
mod buffer;
mod direction;

pub use action::Action;
pub use buffer::GameInputBuffer;
pub use direction::Direction;

pub enum GameInput {
    Direction(Direction),
    Action(Action),
}

impl GameInput {
    pub fn up() -> GameInput {
        GameInput::Direction(Direction::Up)
    }

    pub fn down() -> GameInput {
        GameInput::Direction(Direction::Down)
    }

    pub fn left() -> GameInput {
        GameInput::Direction(Direction::Left)
    }

    pub fn right() -> GameInput {
        GameInput::Direction(Direction::Right)
    }

    pub fn undo() -> GameInput {
        GameInput::Action(Action::Undo)
    }

    pub fn reload() -> GameInput {
        GameInput::Action(Action::Reload)
    }

    pub fn selection() -> GameInput {
        GameInput::Action(Action::Selection)
    }

    pub fn exit() -> GameInput {
        GameInput::Action(Action::Exit)
    }
}
