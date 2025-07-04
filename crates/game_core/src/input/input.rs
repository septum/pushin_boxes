use crate::input::{Action, Direction};

#[derive(Clone, Copy)]
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

    pub fn select() -> Input {
        Input::Action(Action::Select)
    }

    pub fn toggle() -> Input {
        Input::Action(Action::Toggle)
    }

    pub fn delete() -> Input {
        Input::Action(Action::Delete)
    }

    pub fn exit() -> Input {
        Input::Action(Action::Exit)
    }
}
