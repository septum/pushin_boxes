mod ui;

use bevy::{app::Plugin as BevyPlugin, prelude::*};
use iyes_loopless::prelude::*;

use crate::{resources::prelude::*, ui::OverlayMarker};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Instructions, self::ui::spawn)
            .add_system(handle_input.run_in_state(GameState::Instructions))
            .add_exit_system(GameState::Instructions, cleanup::<OverlayMarker>);
    }
}

fn handle_input(
    mut game_state_event_writer: EventWriter<SceneTransitionEvent>,
    mut action_event_reader: EventReader<ActionInputEvent>,
) {
    for action_event in action_event_reader.iter() {
        match action_event.value {
            ActionInput::Pick => {
                game_state_event_writer.send(SceneTransitionEvent::selection(false));
            }
            ActionInput::Exit => game_state_event_writer.send(SceneTransitionEvent::title()),
            _ => (),
        }
    }
}
