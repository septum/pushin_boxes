use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_asset_loader::prelude::*;

use crate::{
    assets::prelude::*,
    level::LevelHandles,
    state::{GameState, GameStateTransitionEvent},
};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Title)
                .load_collection::<LevelHandles>()
                .load_collection::<Fonts>()
                .load_collection::<Images>()
                .load_collection::<Sounds>(),
        )
        .add_systems(
            Update,
            insert_next_state.run_if(on_event::<GameStateTransitionEvent>),
        );
    }
}

fn insert_next_state(
    mut commands: Commands,
    mut scene_transition_event_reader: EventReader<GameStateTransitionEvent>,
) {
    if let Some(scene_transition) = scene_transition_event_reader.read().next() {
        commands.insert_resource(NextState::Pending(scene_transition.state));
    }
}
