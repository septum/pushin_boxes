use bevy::{app::Plugin as BevyPlugin, prelude::*};

use crate::{
    assets::prelude::*,
    character::{Character, CharacterAnimation},
    input::InputEvent,
    state::GameState,
};

use super::systems::{handle_input, save};

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
                CharacterAnimation::update_character_happy_animation,
            )
                .run_if(in_state(GameState::Win)),
        )
        .add_systems(
            OnExit(GameState::Win),
            (cleanup::<game_ui::OverlayMarker>, cleanup::<Character>).chain(),
        );
    }
}
