use bevy::{app::Plugin as BevyPlugin, prelude::*};
use game_ui::OverlayMarker;

use crate::{
    assets::prelude::*,
    character::Character,
    input::InputEvent,
    level::{Brush, BrushSprite, EntityComponent, LevelValidity},
    state::GameState,
};

use super::systems::{
    apply_brush_to_level, blink_tile, check_total_custom_levels, check_validity, handle_input,
    play_sfx, setup_level, update_brush_sprite, update_character_position, update_map,
};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelValidity::default())
            .add_systems(
                OnEnter(GameState::Editor),
                (
                    check_total_custom_levels,
                    super::ui::spawn,
                    Brush::insert,
                    setup_level,
                ),
            )
            .add_systems(
                Update,
                (
                    handle_input.run_if(on_event::<InputEvent>),
                    blink_tile,
                    apply_brush_to_level,
                    update_character_position,
                    update_map,
                    update_brush_sprite,
                    check_validity,
                    play_sfx.run_if(on_event::<InputEvent>),
                )
                    .run_if(in_state(GameState::Editor)),
            )
            .add_systems(
                OnExit(GameState::Editor),
                (
                    cleanup::<OverlayMarker>,
                    cleanup::<Character>,
                    cleanup::<EntityComponent>,
                    cleanup::<BrushSprite>,
                ),
            );
    }
}
