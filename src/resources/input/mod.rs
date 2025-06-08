mod action;
mod direction;

pub use action::{ActionInput, ActionInputEvent};
pub use direction::{DirectionInput, DirectionInputEvent};

use bevy::{app::Plugin as BevyPlugin, input::keyboard::KeyboardInput, prelude::*};

use super::prelude::*;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                gather_input,
                clear_input.run_if(on_event::<SceneTransitionEvent>()),
            ),
        );
    }
}

fn gather_input(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut action_event_writer: EventWriter<ActionInputEvent>,
    mut direction_event_writer: EventWriter<DirectionInputEvent>,
) {
    for event in keyboard_input_events.read() {
        if event.state.is_pressed() {
            if let Some(key_code) = event.key_code {
                match key_code {
                    KeyCode::Up => direction_event_writer.send(DirectionInputEvent::up()),
                    KeyCode::Down => direction_event_writer.send(DirectionInputEvent::down()),
                    KeyCode::Left => direction_event_writer.send(DirectionInputEvent::left()),
                    KeyCode::Right => direction_event_writer.send(DirectionInputEvent::right()),
                    KeyCode::Z => action_event_writer.send(ActionInputEvent::undo()),
                    KeyCode::F5 => action_event_writer.send(ActionInputEvent::reload()),
                    KeyCode::Escape => action_event_writer.send(ActionInputEvent::exit()),
                    KeyCode::Space => action_event_writer.send(ActionInputEvent::select()),
                    KeyCode::Return => action_event_writer.send(ActionInputEvent::toggle()),
                    KeyCode::Delete => action_event_writer.send(ActionInputEvent::delete()),
                    _ => (),
                };
            }
        }
    }
}

fn clear_input(
    mut action_events: ResMut<Events<ActionInputEvent>>,
    mut direction_events: ResMut<Events<DirectionInputEvent>>,
) {
    action_events.clear();
    direction_events.clear();
}
