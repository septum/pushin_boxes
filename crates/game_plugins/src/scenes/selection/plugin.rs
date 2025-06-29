use bevy::{app::Plugin as BevyPlugin, prelude::*};
use game_ui::OverlayMarker;

use crate::{
    assets::prelude::*,
    input::{ActionInputEvent, DirectionInputEvent},
    state::{GameState, SelectionKind},
};

use super::systems::{
    handle_action_input, handle_direction_input, play_action_sfx, play_direction_sfx,
};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        for state in [
            GameState::Selection(SelectionKind::Stock),
            GameState::Selection(SelectionKind::Custom),
        ] {
            app.add_systems(OnEnter(state), super::ui::spawn)
                .add_systems(
                    Update,
                    (
                        handle_action_input.run_if(on_event::<ActionInputEvent>),
                        handle_direction_input.run_if(on_event::<DirectionInputEvent>),
                        play_action_sfx.run_if(on_event::<ActionInputEvent>),
                        play_direction_sfx.run_if(on_event::<DirectionInputEvent>),
                    )
                        .run_if(in_state(state)),
                )
                .add_systems(OnExit(state), cleanup::<OverlayMarker>);
        }
    }
}
