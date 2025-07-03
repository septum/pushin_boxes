use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use game_core::input::{ActionInput, DirectionInput};
use game_ui::DynamicTextData;

use crate::{
    assets::prelude::*,
    input::{ActionInputEvent, DirectionInputEvent},
    save_file::SaveFile,
    state::GameStateTransitionEvent,
};

use super::ui::VOLUME_ID;

pub fn handle_action_input(
    mut game_state_event_writer: EventWriter<GameStateTransitionEvent>,
    mut action_event_reader: EventReader<ActionInputEvent>,
    save_file: Res<SaveFile>,
) {
    for action_event in action_event_reader.read() {
        if matches!(action_event.value, ActionInput::Exit) {
            save_file.save();
            game_state_event_writer.write(GameStateTransitionEvent::title());
        }
    }
}

pub fn handle_direction_input(
    mut direction_event_reader: EventReader<DirectionInputEvent>,
    mut sounds: ResMut<Sounds>,
    mut save_file: ResMut<SaveFile>,
) {
    for direction_event in direction_event_reader.read() {
        match direction_event.value {
            DirectionInput::Left => {
                sounds.decrease_volume();
                save_file.set_volume(sounds.get_volume());
            }
            DirectionInput::Right => {
                sounds.increase_volume();
                save_file.set_volume(sounds.get_volume());
            }
            _ => (),
        }
    }
}

pub fn play_direction_sfx(
    mut direction_event_reader: EventReader<DirectionInputEvent>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    for direction_event in direction_event_reader.read() {
        match direction_event.value {
            DirectionInput::Left | DirectionInput::Right => {
                sfx.play(sounds.sfx_move_character.clone());
            }
            _ => (),
        }
    }
}

pub fn play_action_sfx(
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

pub fn update_dynamic_text(
    sounds: Res<Sounds>,
    mut writer: TextUiWriter,
    texts: Query<(Entity, &DynamicTextData)>,
) {
    for (entity, data) in texts {
        *writer.text(entity, 1) = match data.id {
            VOLUME_ID => format!("<{:>4.0}%>", sounds.get_volume() * 100.0),
            _ => unreachable!("The text id does not exists"),
        };
    }
}
