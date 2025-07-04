use bevy::prelude::*;

use game_core::{
    input::{Action, Input},
    level::LevelKind,
};

use crate::{
    input::InputEvent,
    level::{LevelInsertionEvent, LevelResource},
    save_file::SaveFile,
    state::{GameStateTransitionEvent, SelectionKind},
};

pub fn save(mut save_file: ResMut<SaveFile>, level: Res<LevelResource>) {
    save_file.set_new_record(&level);
    save_file.unlock_new_level(&level);
    save_file.save();
}

pub fn handle_input(
    mut level_instertion_event_writer: EventWriter<LevelInsertionEvent>,
    mut game_state_event_writer: EventWriter<GameStateTransitionEvent>,
    mut input_event_reader: EventReader<InputEvent>,
    level: Res<LevelResource>,
) {
    for input_event in input_event_reader.read() {
        match **input_event {
            Input::Action(Action::Select) => match level.kind() {
                LevelKind::Stock(index) => {
                    if level.is_last() {
                        game_state_event_writer
                            .write(GameStateTransitionEvent::selection(SelectionKind::Stock));
                    } else {
                        level_instertion_event_writer
                            .write(LevelInsertionEvent::new(LevelKind::Stock(index + 1)));
                    }
                }
                LevelKind::Custom(_) => {
                    game_state_event_writer
                        .write(GameStateTransitionEvent::selection(SelectionKind::Custom));
                }
                LevelKind::Editable(_) => {
                    unreachable!("An editable level cannot be won");
                }
            },
            Input::Action(Action::Exit) => {
                game_state_event_writer.write(GameStateTransitionEvent::title());
            }
            _ => {}
        }
    }
}
