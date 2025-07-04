use bevy::{app::Plugin as BevyPlugin, input::keyboard::KeyboardInput, prelude::*};
use game_core::input::Input;

use crate::{input::InputEvent, state::GameStateTransitionEvent};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                gather_input,
                clear_input.run_if(on_event::<GameStateTransitionEvent>),
            ),
        );
    }
}

fn gather_input(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut input_event_writer: EventWriter<InputEvent>,
) {
    for event in keyboard_input_events.read() {
        if event.state.is_pressed() {
            let input = match event.key_code {
                KeyCode::ArrowUp => Input::up(),
                KeyCode::ArrowDown => Input::down(),
                KeyCode::ArrowLeft => Input::left(),
                KeyCode::ArrowRight => Input::right(),
                KeyCode::KeyZ => Input::undo(),
                KeyCode::F5 => Input::reload(),
                KeyCode::Escape => Input::exit(),
                KeyCode::Space => Input::select(),
                KeyCode::Enter => Input::toggle(),
                KeyCode::Delete => Input::delete(),
                _ => return,
            };
            input_event_writer.write(input.into());
        }
    }
}

fn clear_input(mut input_events: ResMut<Events<InputEvent>>) {
    input_events.clear();
}
