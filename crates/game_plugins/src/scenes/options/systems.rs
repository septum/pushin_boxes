use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use game_core::input::{Action, Direction, Input};
use game_ui::DynamicTextData;

use crate::{
    assets::prelude::*, input::InputEvent, save_file::SaveFile, state::GameStateTransitionEvent,
};

use super::ui::VOLUME_ID;

pub fn handle_input(
    mut game_state_event_writer: EventWriter<GameStateTransitionEvent>,
    mut input_event_reader: EventReader<InputEvent>,
    mut sounds: ResMut<Sounds>,
    mut save_file: ResMut<SaveFile>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    for input_event in input_event_reader.read() {
        match **input_event {
            Input::Direction(Direction::Left) => {
                sfx.play(sounds.sfx_move_character.clone());
                sounds.decrease_volume();
                save_file.set_volume(sounds.get_volume());
            }
            Input::Direction(Direction::Right) => {
                sfx.play(sounds.sfx_move_character.clone());
                sounds.increase_volume();
                save_file.set_volume(sounds.get_volume());
            }
            Input::Action(Action::Exit) => {
                sfx.play(sounds.sfx_push_box.clone());
                save_file.save();
                game_state_event_writer.write(GameStateTransitionEvent::title());
            }
            _ => (),
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
