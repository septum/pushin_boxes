use bevy::{app::Plugin as BevyPlugin, prelude::*};
use game_ui::OverlayMarker;

use crate::{
    assets::prelude::*,
    input::{ActionInputEvent, DirectionInputEvent},
    scenes::title::systems::handle_direction_input,
    state::GameState,
};

use super::{
    systems::{
        handle_action_input, play_action_sfx, play_direction_sfx, update_character_animation,
    },
    ui,
};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Title),
            (
                ui::spawn,
                CharacterAnimation::insert_blinking_character_animation,
            ),
        )
        .add_systems(
            Update,
            (
                update_character_animation,
                handle_action_input.run_if(on_event::<ActionInputEvent>),
                handle_direction_input.run_if(on_event::<DirectionInputEvent>),
                play_action_sfx.run_if(on_event::<ActionInputEvent>),
                play_direction_sfx.run_if(on_event::<DirectionInputEvent>),
            )
                .run_if(in_state(GameState::Title)),
        )
        .add_systems(
            OnExit(GameState::Title),
            (cleanup::<OverlayMarker>, cleanup::<CharacterMarker>),
        );
    }
}
