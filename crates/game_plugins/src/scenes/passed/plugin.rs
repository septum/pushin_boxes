use bevy::{app::Plugin as BevyPlugin, prelude::*};
use game_ui::OverlayMarker;
use regex::Regex;

use crate::{assets::prelude::*, input::InputEvent, state::GameState};

use super::systems::{LevelNameRegex, TextCursor, handle_input, handle_text_input};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelNameRegex {
            value: Regex::new(r"^[a-zA-Z ]$").unwrap(),
        })
        .insert_resource(TextCursor {
            blink_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            blink_toggle: true,
        })
        .add_systems(OnEnter(GameState::Passed), super::ui::spawn)
        .add_systems(
            Update,
            (
                handle_input.run_if(on_event::<InputEvent>),
                handle_text_input,
            )
                .run_if(in_state(GameState::Passed)),
        )
        .add_systems(OnExit(GameState::Passed), cleanup::<OverlayMarker>);
    }
}
