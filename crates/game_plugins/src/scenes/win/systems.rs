use bevy::prelude::*;

use game_core::{
    input::{Action, Input},
    level::LevelKind,
};

use crate::{
    assets::prelude::*,
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

pub fn update_character_animation(
    time: Res<Time>,
    mut character_animation: ResMut<CharacterAnimation>,
    mut query: Query<&mut Sprite, With<CharacterMarker>>,
) {
    character_animation.tick(time.delta());
    if character_animation.primary_timer_just_finished() {
        let mut sprite = query.single_mut().unwrap();
        character_animation.next_index();
        sprite.texture_atlas.as_mut().unwrap().index = character_animation.sprite_index();
    }
}
