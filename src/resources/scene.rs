use super::state::GameState;

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
    pub fn selection() -> Self {
        Self {
            state: GameState::Selection,
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
