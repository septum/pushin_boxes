use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use game_core::input::{Action, Input};

use crate::{assets::prelude::*, input::InputEvent, state::GameStateTransitionEvent};

pub fn handle_input(
    mut game_state_event_writer: EventWriter<GameStateTransitionEvent>,
    mut input_event_reader: EventReader<InputEvent>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    for input_event in input_event_reader.read() {
        if matches!(**input_event, Input::Action(Action::Exit)) {
            sfx.play(sounds.sfx_push_box.clone());
            game_state_event_writer.write(GameStateTransitionEvent::title());
        }
    }
}
