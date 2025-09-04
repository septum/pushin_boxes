use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_ui_bits::RootMarker;

use crate::{assets::prelude::*, input::InputEvent, state::GameState};

use super::systems::{handle_input, update_dynamic_text};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Options), super::ui::spawn)
            .add_systems(
                Update,
                (
                    handle_input.run_if(on_event::<InputEvent>),
                    update_dynamic_text,
                )
                    .run_if(in_state(GameState::Options)),
            )
            .add_systems(OnExit(GameState::Options), cleanup::<RootMarker>);
    }
}
