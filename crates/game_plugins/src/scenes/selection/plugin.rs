use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_ui_bits::RootMarker;

use crate::{
    assets::prelude::*,
    input::InputEvent,
    save_file::SaveFile,
    state::{GameState, SelectionKind},
};

use super::systems::{handle_input, play_sfx};

#[derive(Resource)]
pub(super) struct SelectedButton(pub usize);

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        for state in [
            GameState::Selection(SelectionKind::Stock),
            GameState::Selection(SelectionKind::Custom),
        ] {
            app.insert_resource(SelectedButton(0))
                .add_systems(
                    OnEnter(state),
                    (initial_selected_button, super::ui::spawn).chain(),
                )
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

pub fn initial_selected_button(
    game_state: Res<State<GameState>>,
    save_file: Res<SaveFile>,
    mut selected_button: ResMut<SelectedButton>,
) {
    #[cfg(not(target_family = "wasm"))]
    {
        match game_state.get_selection_kind() {
            SelectionKind::Stock => selected_button.0 = save_file.unlocked_levels() - 1,
            SelectionKind::Custom => selected_button.0 = save_file.number_custom_levels() - 1,
        }
    }

    #[cfg(target_family = "wasm")]
    {
        selected_button.0 = save_file.unlocked_levels() - 1;
    }
}
