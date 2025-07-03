use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use game_core::input::ActionInput;

use crate::{
    assets::prelude::*,
    input::ActionInputEvent,
    state::{GameStateTransitionEvent, SelectionKind},
};

pub fn handle_input(
    mut game_state_event_writer: EventWriter<GameStateTransitionEvent>,
    mut action_event_reader: EventReader<ActionInputEvent>,
) {
    for action_event in action_event_reader.read() {
        if matches!(action_event.value, ActionInput::Select) {
            game_state_event_writer
                .write(GameStateTransitionEvent::selection(SelectionKind::Custom));
        }
    }
}

pub fn play_sfx(
    mut action_event_reader: EventReader<ActionInputEvent>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    for action_event in action_event_reader.read() {
        if matches!(action_event.value, ActionInput::Select) {
            sfx.play(sounds.sfx_push_box.clone());
        }
    }
}
