use bevy::prelude::*;

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Title,
    Instructions,
    Editor,
    Limit,
    Passed,
    Options,
    Selection(SelectionKind),
    Level,
    Win,
}

impl GameState {
    pub fn get_selection_kind(&self) -> &SelectionKind {
        match self {
            Self::Selection(kind) => kind,
            _ => unreachable!("The GameState is not Selection"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SelectionKind {
    Stock,
    Custom,
}

impl SelectionKind {
    pub fn is_stock(&self) -> bool {
        matches!(self, Self::Stock)
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Stock => "Stock",
            Self::Custom => "Custom",
        }
    }

    #[must_use]
    pub fn toggle(&self) -> Self {
        match self {
            Self::Stock => Self::Custom,
            Self::Custom => Self::Stock,
        }
    }
}
