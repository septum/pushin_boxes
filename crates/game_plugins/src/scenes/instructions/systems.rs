use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{
    assets::prelude::*,
    input::{ActionInput, ActionInputEvent},
    state::GameStateTransitionEvent,
};

pub fn handle_input(
    mut game_state_event_writer: EventWriter<GameStateTransitionEvent>,
    mut action_event_reader: EventReader<ActionInputEvent>,
) {
    for action_event in action_event_reader.read() {
        if matches!(action_event.value, ActionInput::Exit) {
            game_state_event_writer.write(GameStateTransitionEvent::title());
        }
    }
}

pub fn play_sfx(
    mut action_event_reader: EventReader<ActionInputEvent>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    for action_event in action_event_reader.read() {
        if matches!(action_event.value, ActionInput::Exit) {
            sfx.play(sounds.sfx_push_box.clone());
        }
    }
}
