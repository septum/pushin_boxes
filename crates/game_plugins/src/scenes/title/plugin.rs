use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_ui_bits::RootMarker;

use crate::{
    assets::prelude::*,
    character::{Character, CharacterAnimation},
    input::InputEvent,
    state::GameState,
};

use super::{
    systems::{handle_input, play_sfx},
    ui,
};

#[derive(Resource)]
pub struct SelectedButton(pub Option<usize>);

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedButton(Some(ui::PLAY_ID)))
            .add_systems(
                OnEnter(GameState::Title),
                (
                    ui::spawn,
                    CharacterAnimation::insert_blinking_character_animation,
                ),
            )
            .add_systems(
                Update,
                (
                    CharacterAnimation::update_blinking_character_animation,
                    handle_input.run_if(on_event::<InputEvent>),
                    play_sfx.run_if(on_event::<InputEvent>),
                )
                    .run_if(in_state(GameState::Title)),
            )
            .add_systems(
                OnExit(GameState::Title),
                (cleanup::<RootMarker>, cleanup::<Character>),
            );
    }
}
