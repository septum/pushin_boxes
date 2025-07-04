use bevy::{app::Plugin as BevyPlugin, prelude::*};
use game_ui::OverlayMarker;

use crate::{assets::prelude::*, input::InputEvent, state::GameState};

use super::{
    systems::{handle_input, play_sfx, update_character_animation},
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
                handle_input.run_if(on_event::<InputEvent>),
                play_sfx.run_if(on_event::<InputEvent>),
            )
                .run_if(in_state(GameState::Title)),
        )
        .add_systems(
            OnExit(GameState::Title),
            (cleanup::<OverlayMarker>, cleanup::<CharacterMarker>),
        );
    }
}
