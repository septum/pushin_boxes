use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_ui_bits::RootMarker;

use crate::{
    assets::prelude::*,
    input::InputEvent,
    state::{GameState, SelectionKind},
};

use super::systems::{handle_input, play_sfx};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        for state in [
            GameState::Selection(SelectionKind::Stock),
            GameState::Selection(SelectionKind::Custom),
        ] {
            app.add_systems(OnEnter(state), super::ui::spawn)
                .add_systems(
                    Update,
                    (
                        handle_input.run_if(on_event::<InputEvent>),
                        play_sfx.run_if(on_event::<InputEvent>),
                    )
                        .run_if(in_state(state)),
                )
                .add_systems(OnExit(state), cleanup::<RootMarker>);
        }
    }
}
