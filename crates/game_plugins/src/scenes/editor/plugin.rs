use bevy::{app::Plugin as BevyPlugin, prelude::*};
use game_ui::OverlayMarker;

use crate::{
    assets::prelude::*,
    input::{ActionInputEvent, DirectionInputEvent},
    level::{Brush, BrushSprite, LevelValidity, MapPositionComponent},
    state::GameState,
};

use super::systems::{
    apply_brush_to_level, blink_tile, check_total_custom_levels, check_validity,
    handle_action_input, handle_direction_input, play_action_sfx, play_direction_sfx, setup_level,
    update_brush_sprite, update_character_position, update_map,
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
                    handle_action_input,
                    handle_direction_input,
                    blink_tile,
                    apply_brush_to_level,
                    update_character_position,
                    update_map,
                    update_brush_sprite,
                    check_validity,
                    play_action_sfx.run_if(on_event::<ActionInputEvent>),
                    play_direction_sfx.run_if(on_event::<DirectionInputEvent>),
                )
                    .run_if(in_state(GameState::Editor)),
            )
            .add_systems(
                OnExit(GameState::Editor),
                (
                    cleanup::<OverlayMarker>,
                    cleanup::<CharacterMarker>,
                    cleanup::<MapPositionComponent>,
                    cleanup::<BrushSprite>,
                ),
            );
    }
}
