use bevy::{app::Plugin as BevyPlugin, prelude::*};
use game_ui::OverlayMarker;

use crate::{
    assets::prelude::*,
    character::{Character, CharacterAnimation},
    input::InputEvent,
    level::EntityComponent,
    state::GameState,
};

use super::systems::{
    check_lever_timer_just_finished, handle_input, spawn_level, update_character_position,
    update_counters, update_level_state, update_map,
};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Level),
            (
                super::ui::spawn,
                spawn_level,
                CharacterAnimation::insert_level_character_animation,
            ),
        )
        .add_systems(
            Update,
            (
                handle_input.run_if(on_event::<InputEvent>),
                CharacterAnimation::update_level_character_animation,
                update_character_position,
                update_counters,
                update_map,
                update_level_state,
                check_lever_timer_just_finished,
            )
                .chain()
                .run_if(in_state(GameState::Level)),
        )
        .add_systems(
            OnExit(GameState::Level),
            (
                cleanup::<OverlayMarker>,
                cleanup::<Character>,
                cleanup::<EntityComponent>,
            ),
        );
    }
}
