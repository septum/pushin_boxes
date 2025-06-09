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
                clear_input.run_if(on_event::<SceneTransitionEvent>),
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
            match event.key_code {
                KeyCode::ArrowUp => {
                    direction_event_writer.write(DirectionInputEvent::up());
                }
                KeyCode::ArrowDown => {
                    direction_event_writer.write(DirectionInputEvent::down());
                }
                KeyCode::ArrowLeft => {
                    direction_event_writer.write(DirectionInputEvent::left());
                }
                KeyCode::ArrowRight => {
                    direction_event_writer.write(DirectionInputEvent::right());
                }
                KeyCode::KeyZ => {
                    action_event_writer.write(ActionInputEvent::undo());
                }
                KeyCode::F5 => {
                    action_event_writer.write(ActionInputEvent::reload());
                }
                KeyCode::Escape => {
                    action_event_writer.write(ActionInputEvent::exit());
                }
                KeyCode::Space => {
                    action_event_writer.write(ActionInputEvent::select());
                }
                KeyCode::Enter => {
                    action_event_writer.write(ActionInputEvent::toggle());
                }
                KeyCode::Delete => {
                    action_event_writer.write(ActionInputEvent::delete());
                }
                _ => (),
            };
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
