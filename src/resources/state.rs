use bevy::{app::Plugin as BevyPlugin, prelude::*};
use iyes_loopless::prelude::*;

use super::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Loading,
    Title,
    Instructions,
    Editor,
    Selection,
    Level,
    Win,
}

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(insert_next_state.run_on_event::<SceneTransitionEvent>());
    }
}

fn insert_next_state(
    mut commands: Commands,
    mut scene_transition_event_reader: EventReader<SceneTransitionEvent>,
) {
    if let Some(scene_transition) = scene_transition_event_reader.iter().next() {
        commands.insert_resource(NextState(scene_transition.state.clone()));
    };
}
