use bevy::{app::AppExit, prelude::*};
use bevy_kira_audio::{AudioChannel, AudioControl};
use bevy_ui_bits::UiButtonData;
use game_core::input::{Action, Direction, Input};

use crate::{
    assets::prelude::*,
    input::InputEvent,
    scenes::title::plugin::SelectedButton,
    state::{GameStateTransitionEvent, SelectionKind},
};

use super::ui::{EDITOR_ID, INSTRUCTIONS_ID, OPTIONS_ID, PLAY_ID, QUIT_ID};

pub fn handle_input(
    mut query: Query<(&UiButtonData, &mut BackgroundColor)>,
    mut input_event_reader: EventReader<InputEvent>,
    mut game_state_event_writer: EventWriter<GameStateTransitionEvent>,
    mut exit: EventWriter<AppExit>,
    mut selected_button: ResMut<SelectedButton>,
) {
    for input_event in input_event_reader.read() {
        match **input_event {
            Input::Direction(direction) => {
                if matches!(direction, Direction::Up | Direction::Down) {
                    let up = matches!(direction, Direction::Up);
                    #[cfg(not(target_family = "wasm"))]
                    {
                        selected_button.0 = match selected_button.0 {
                            PLAY_ID => {
                                if up {
                                    QUIT_ID
                                } else {
                                    INSTRUCTIONS_ID
                                }
                            }
                            INSTRUCTIONS_ID => {
                                if up {
                                    PLAY_ID
                                } else {
                                    EDITOR_ID
                                }
                            }
                            EDITOR_ID => {
                                if up {
                                    INSTRUCTIONS_ID
                                } else {
                                    OPTIONS_ID
                                }
                            }
                            OPTIONS_ID => {
                                if up {
                                    EDITOR_ID
                                } else {
                                    QUIT_ID
                                }
                            }
                            QUIT_ID => {
                                if up {
                                    OPTIONS_ID
                                } else {
                                    PLAY_ID
                                }
                            }
                            _ => unreachable!("The button id was not declared"),
                        };
                    }

                    #[cfg(target_family = "wasm")]
                    {
                        selected_button.0 = match selected_button.0 {
                            PLAY_ID => {
                                if up {
                                    OPTIONS_ID
                                } else {
                                    INSTRUCTIONS_ID
                                }
                            }
                            INSTRUCTIONS_ID => {
                                if up {
                                    PLAY_ID
                                } else {
                                    OPTIONS_ID
                                }
                            }
                            OPTIONS_ID => {
                                if up {
                                    INSTRUCTIONS_ID
                                } else {
                                    PLAY_ID
                                }
                            }
                            _ => unreachable!("The button id was not declared or is not available"),
                        };
                    }

                    for (button, mut color) in &mut query {
                        if button.id == selected_button.0 {
                            *color = crate::theme::PRIMARY_DARK.into();
                        } else {
                            *color = crate::theme::TRANSPARENT.into();
                        }
                    }
                }
            }
            Input::Action(Action::Select) => match selected_button.0 {
                PLAY_ID => {
                    game_state_event_writer
                        .write(GameStateTransitionEvent::selection(SelectionKind::Stock));
                }
                INSTRUCTIONS_ID => {
                    game_state_event_writer.write(GameStateTransitionEvent::instructions());
                }
                EDITOR_ID => {
                    game_state_event_writer.write(GameStateTransitionEvent::editor());
                }
                OPTIONS_ID => {
                    game_state_event_writer.write(GameStateTransitionEvent::options());
                }
                QUIT_ID => {
                    #[cfg(not(target_family = "wasm"))]
                    {
                        exit.write(AppExit::Success);
                    }
                }
                _ => unreachable!("The button id was not declared"),
            },
            Input::Action(Action::Exit) => {
                #[cfg(not(target_family = "wasm"))]
                {
                    exit.write(AppExit::Success);
                }
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
            Input::Action(Action::Exit) => {
                sfx.play(sounds.sfx_push_box.clone());
            }
            Input::Action(Action::Select) => {
                sfx.play(sounds.sfx_set_zone.clone());
            }
            Input::Action(_) => (),
        }
    }
}
