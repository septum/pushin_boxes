use super::state::{GameState, SelectionKind};

pub struct SceneTransitionEvent {
    pub state: GameState,
}

impl SceneTransitionEvent {
    pub fn loading() -> Self {
        Self {
            state: GameState::Loading,
        }
    }

    pub fn title() -> Self {
        Self {
            state: GameState::Title,
        }
    }

    pub fn instructions() -> Self {
        Self {
            state: GameState::Instructions,
        }
    }

    pub fn editor() -> Self {
        Self {
            state: GameState::Editor,
        }
    }

    pub fn options() -> Self {
        Self {
            state: GameState::Options,
        }
    }

    pub fn passed() -> Self {
        Self {
            state: GameState::Passed,
        }
    }

    pub fn selection(kind: SelectionKind) -> Self {
        Self {
            state: GameState::Selection { kind },
        }
    }

    pub fn level() -> Self {
        Self {
            state: GameState::Level,
        }
    }

    pub fn win() -> Self {
        Self {
            state: GameState::Win,
        }
    }
}
