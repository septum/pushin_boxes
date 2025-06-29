use bevy::{app::Plugin as BevyPlugin, prelude::*};

use crate::{assets::prelude::*, input::ActionInputEvent, state::GameState};

use super::systems::{handle_input, play_sfx};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Instructions), super::ui::spawn)
            .add_systems(
                Update,
                (
                    handle_input.run_if(on_event::<ActionInputEvent>),
                    play_sfx.run_if(on_event::<ActionInputEvent>),
                )
                    .run_if(in_state(GameState::Instructions)),
            )
            .add_systems(
                OnExit(GameState::Instructions),
                cleanup::<game_ui::OverlayMarker>,
            );
    }
}
