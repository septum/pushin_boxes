mod action;
mod buffer;
mod direction;

pub use action::Action;
pub use buffer::GameInputBuffer;
pub use direction::Direction;

const COUNTER_START_VALUE: usize = 16;

#[derive(Default)]
pub struct IgnoreInputCounter {
    value: usize,
}

impl IgnoreInputCounter {
    pub fn start(&mut self) {
        self.value = COUNTER_START_VALUE;
    }

    pub fn tick(&mut self) {
        self.value = self.value.saturating_sub(1);
    }

    pub fn done(&mut self) -> bool {
        self.value < 1
    }
}

pub enum GameInput {
    Direction(Direction),
    Action(Action),
}

impl GameInput {
    #[must_use]
    pub fn up() -> GameInput {
        GameInput::Direction(Direction::Up)
    }

    #[must_use]
    pub fn down() -> GameInput {
        GameInput::Direction(Direction::Down)
    }

    #[must_use]
    pub fn left() -> GameInput {
        GameInput::Direction(Direction::Left)
    }

    #[must_use]
    pub fn right() -> GameInput {
        GameInput::Direction(Direction::Right)
    }

    #[must_use]
    pub fn undo() -> GameInput {
        GameInput::Action(Action::Undo)
    }

    #[must_use]
    pub fn reload() -> GameInput {
        GameInput::Action(Action::Reload)
    }

    #[must_use]
    pub fn selection() -> GameInput {
        GameInput::Action(Action::Selection)
    }

    #[must_use]
    pub fn pick() -> GameInput {
        GameInput::Action(Action::Pick)
    }

    #[must_use]
    pub fn volume() -> GameInput {
        GameInput::Action(Action::Volume)
    }

    #[must_use]
    pub fn exit() -> GameInput {
        GameInput::Action(Action::Exit)
    }
}
