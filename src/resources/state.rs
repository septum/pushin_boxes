use bevy::{app::Plugin as BevyPlugin, prelude::*};

use super::prelude::*;

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
    SelectionStock,
    SelectionCustom,
    // Selection {
    //     kind: SelectionKind,
    // },
    Level,
    Win,
}

impl GameState {
    pub fn get_selection_kind(&self) -> &SelectionKind {
        match self {
            Self::SelectionStock => &SelectionKind::Stock,
            Self::SelectionCustom => &SelectionKind::Custom,
            _ => unreachable!("The GameState is not Selection"),
        }
    }
}

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(insert_next_state.run_if(on_event::<SceneTransitionEvent>()));
    }
}

fn insert_next_state(
    mut commands: Commands,
    mut scene_transition_event_reader: EventReader<SceneTransitionEvent>,
) {
    if let Some(scene_transition) = scene_transition_event_reader.iter().next() {
        commands.insert_resource(NextState(scene_transition.state.clone().into()));
    };
}
