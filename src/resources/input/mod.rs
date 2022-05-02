mod action;
mod buffer;
mod direction;

pub use action::Action;
pub use buffer::InputBuffer;
pub use direction::Direction;

pub enum Input {
    Direction(Direction),
    Action(Action),
}

impl Input {
    pub fn up() -> Input {
        Input::Direction(Direction::Up)
    }

    pub fn down() -> Input {
        Input::Direction(Direction::Down)
    }

    pub fn left() -> Input {
        Input::Direction(Direction::Left)
    }

    pub fn right() -> Input {
        Input::Direction(Direction::Right)
    }

    pub fn undo() -> Input {
        Input::Action(Action::Undo)
    }

    pub fn reload() -> Input {
        Input::Action(Action::Reload)
    }

    pub fn selection() -> Input {
        Input::Action(Action::Selection)
    }

    pub fn exit() -> Input {
        Input::Action(Action::Exit)
    }
}
