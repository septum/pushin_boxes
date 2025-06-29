use bevy::{app::Plugin as BevyPlugin, prelude::*};

use crate::{assets::prelude::*, input::ActionInputEvent, state::GameState};

use super::systems::{handle_input, play_sfx};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Limit), super::ui::spawn)
            .add_systems(
                Update,
                (
                    handle_input.run_if(on_event::<ActionInputEvent>),
                    play_sfx.run_if(on_event::<ActionInputEvent>),
                )
                    .run_if(in_state(GameState::Limit)),
            )
            .add_systems(OnExit(GameState::Limit), cleanup::<game_ui::OverlayMarker>);
    }
}
