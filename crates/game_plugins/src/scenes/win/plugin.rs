use bevy::{app::Plugin as BevyPlugin, prelude::*};

use crate::{assets::prelude::*, input::InputEvent, state::GameState};

use super::systems::{handle_input, save, update_character_animation};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Win),
            (
                save,
                super::ui::spawn,
                CharacterAnimation::insert_happy_character_animation,
            ),
        )
        .add_systems(
            Update,
            (
                handle_input.run_if(on_event::<InputEvent>),
                update_character_animation,
            )
                .run_if(in_state(GameState::Win)),
        )
        .add_systems(
            OnExit(GameState::Win),
            (
                cleanup::<game_ui::OverlayMarker>,
                cleanup::<CharacterMarker>,
            )
                .chain(),
        );
    }
}
