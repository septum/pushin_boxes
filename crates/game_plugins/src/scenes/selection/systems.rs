use std::{env, fs::remove_file, path::PathBuf};

use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use game_core::{
    input::{Action, Direction, Input},
    level::LevelKind,
};
use game_ui::{Colors, GameButtonData};

use crate::{
    assets::prelude::*,
    input::InputEvent,
    level::LevelInsertionEvent,
    save_file::SaveFile,
    state::{GameState, GameStateTransitionEvent, SelectionKind},
};

pub fn handle_input(
    mut level_insertion_event_writer: EventWriter<LevelInsertionEvent>,
    mut scene_transition_event_writer: EventWriter<GameStateTransitionEvent>,
    mut query: Query<(&mut GameButtonData, &mut BackgroundColor)>,
    mut input_event_reader: EventReader<InputEvent>,
    mut save_file: ResMut<SaveFile>,
    game_state: Res<State<GameState>>,
) {
    let is_stock = game_state.get_selection_kind().is_stock();

    for input_event in input_event_reader.read() {
        match **input_event {
            Input::Direction(direction) => {
                let mut selected_index: Option<usize> = None;
                for (button, _) in query.iter() {
                    if button.selected {
                        let index = match direction {
                            Direction::Up => button.id.saturating_sub(4),
                            Direction::Down => button.id + 4,
                            Direction::Left => button.id.saturating_sub(1),
                            Direction::Right => button.id + 1,
                        };

                        let max_value = if is_stock {
                            save_file.unlocked_levels()
                        } else {
                            save_file.number_custom_levels()
                        };

                        selected_index = Some(if index < max_value {
                            index
                        } else {
                            max_value - 1
                        });

                        break;
                    }
                }

                if let Some(selected_index) = selected_index {
                    for (mut button, mut color) in &mut query {
                        if selected_index == button.id {
                            button.selected = true;
                            *color = Colors::PRIMARY_DARK.into();
                        } else {
                            button.selected = false;
                            *color = Colors::TRANSPARENT.into();
                        }
                    }
                }
            }
            Input::Action(Action::Select) => {
                for (button, _) in &mut query {
                    if button.selected {
                        let kind = if is_stock {
                            LevelKind::Stock(button.id)
                        } else {
                            LevelKind::Custom(
                                button
                                    .payload
                                    .clone()
                                    .expect("The button payload was empty"),
                            )
                        };

                        level_insertion_event_writer.write(LevelInsertionEvent::new(kind));
                    }
                }
            }
            Input::Action(Action::Toggle) => {
                #[cfg(not(target_family = "wasm"))]
                {
                    scene_transition_event_writer.write(GameStateTransitionEvent::selection(
                        if is_stock {
                            SelectionKind::Custom
                        } else {
                            SelectionKind::Stock
                        },
                    ));
                }
            }

            Input::Action(Action::Delete) => {
                for (button, _) in &mut query {
                    if button.selected && !is_stock {
                        let payload = button
                            .payload
                            .clone()
                            .expect("The button payload was empty");
                        let parsed_payload: Vec<&str> = payload.split('$').collect();
                        save_file.delete_custom_level_record(&payload);
                        save_file.save();
                        #[cfg(not(target_family = "wasm"))]
                        {
                            let levels_path = format!("levels/custom/{}.lvl", parsed_payload[1]);
                            let assets_path = format!("assets/{}", &levels_path);
                            let path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
                                PathBuf::from(manifest_dir).join(assets_path)
                            } else {
                                PathBuf::from(assets_path)
                            };
                            remove_file(path).expect("File cannot be removed");
                        }
                        scene_transition_event_writer
                            .write(GameStateTransitionEvent::selection(SelectionKind::Custom));
                    }
                }
            }
            Input::Action(Action::Exit) => {
                scene_transition_event_writer.write(GameStateTransitionEvent::title());
            }
            Input::Action(_) => (),
        }
    }
}

pub fn play_sfx(
    mut input_event_reader: EventReader<InputEvent>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    for input_event in input_event_reader.read() {
        match **input_event {
            Input::Direction(_) => {
                sfx.play(sounds.sfx_move_character.clone());
            }
            Input::Action(Action::Exit | Action::Delete) => {
                sfx.play(sounds.sfx_push_box.clone());
            }
            Input::Action(Action::Toggle) => {
                sfx.play(sounds.sfx_toggle_volume.clone());
            }
            Input::Action(Action::Select) => {
                sfx.play(sounds.sfx_set_zone.clone());
            }
            Input::Action(_) => (),
        }
    }
}
