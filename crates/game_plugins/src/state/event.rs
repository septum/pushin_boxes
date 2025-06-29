use bevy::prelude::*;

use super::state::{GameState, SelectionKind};

#[derive(Event)]
pub struct GameStateTransitionEvent {
    pub state: GameState,
}

impl GameStateTransitionEvent {
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

    pub fn limit() -> Self {
        Self {
            state: GameState::Limit,
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
            state: GameState::Selection(kind),
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
