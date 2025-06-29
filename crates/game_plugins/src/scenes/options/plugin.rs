use bevy::{app::Plugin as BevyPlugin, prelude::*};
use game_ui::OverlayMarker;

use crate::{
    assets::prelude::*,
    input::{ActionInputEvent, DirectionInputEvent},
    state::GameState,
};

use super::systems::{
    handle_action_input, handle_direction_input, play_action_sfx, play_direction_sfx,
    update_dynamic_text,
};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Options), super::ui::spawn)
            .add_systems(
                Update,
                (
                    handle_action_input.run_if(on_event::<ActionInputEvent>),
                    handle_direction_input.run_if(on_event::<DirectionInputEvent>),
                    play_action_sfx.run_if(on_event::<ActionInputEvent>),
                    play_direction_sfx.run_if(on_event::<DirectionInputEvent>),
                    update_dynamic_text,
                )
                    .run_if(in_state(GameState::Options)),
            )
            .add_systems(OnExit(GameState::Options), cleanup::<OverlayMarker>);
    }
}
