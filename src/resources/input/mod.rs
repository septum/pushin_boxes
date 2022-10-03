mod action;
mod direction;

pub use action::{Action, ActionEvent};
pub use direction::{Direction, DirectionEvent};

use bevy::{app::Plugin as BevyPlugin, input::keyboard::KeyboardInput, prelude::*};
use iyes_loopless::prelude::*;

use super::prelude::*;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(gather_input)
            .add_system(clear_input.run_on_event::<SceneTransitionEvent>());
    }
}

pub fn gather_input(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut action_event_writer: EventWriter<ActionEvent>,
    mut direction_event_writer: EventWriter<DirectionEvent>,
) {
    for event in keyboard_input_events.iter() {
        if event.state.is_pressed() {
            if let Some(key_code) = event.key_code {
                match key_code {
                    KeyCode::Up => direction_event_writer.send(DirectionEvent::up()),
                    KeyCode::Down => direction_event_writer.send(DirectionEvent::down()),
                    KeyCode::Left => direction_event_writer.send(DirectionEvent::left()),
                    KeyCode::Right => direction_event_writer.send(DirectionEvent::right()),
                    KeyCode::V => action_event_writer.send(ActionEvent::volume()),
                    KeyCode::U => action_event_writer.send(ActionEvent::undo()),
                    KeyCode::R => action_event_writer.send(ActionEvent::reload()),
                    KeyCode::S => action_event_writer.send(ActionEvent::selection()),
                    KeyCode::Escape => action_event_writer.send(ActionEvent::exit()),
                    KeyCode::Space => action_event_writer.send(ActionEvent::pick()),
                    _ => (),
                };
            }
        }
    }
}

pub fn clear_input(
    mut action_events: ResMut<Events<ActionEvent>>,
    mut direction_events: ResMut<Events<DirectionEvent>>,
) {
    action_events.clear();
    direction_events.clear();
}
