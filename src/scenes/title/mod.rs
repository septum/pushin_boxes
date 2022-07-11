mod ui;

use bevy::{app::AppExit, prelude::*};
use bevy_kira_audio::Audio;
use bevy_rust_arcade::{ArcadeInput, ArcadeInputEvent};

use crate::{
    core::state::GameState,
    resources::{
        input::{Action, Direction},
        prelude::*,
    },
    ui::{ButtonKind, ButtonMarker},
};

use ui::{spawn_ui, UiMarker};

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Title)
                .with_system(setup)
                .with_system(start_audio),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Title)
                .with_system(gather_input)
                .with_system(handle_input),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Title)
                .with_system(cleanup)
                .with_system(stop_audio),
        );
    }
}

fn setup(
    mut commands: Commands,
    fonts: Res<Fonts>,
    mut ignore_input_counter: ResMut<IgnoreInputCounter>,
) {
    spawn_ui(&mut commands, &fonts);
    ignore_input_counter.start();
}

fn start_audio(sounds: Res<Sounds>, audio: Res<Audio>) {
    let audio_source = sounds.music.title.clone();
    let channel_id = &sounds.channels.music;
    audio.play_looped_in_channel(audio_source, channel_id);
}

fn gather_input(
    mut arcade_input_events: EventReader<ArcadeInputEvent>,
    mut input_buffer: ResMut<GameInputBuffer>,
    mut ignore_input_counter: ResMut<IgnoreInputCounter>,
) {
    if ignore_input_counter.done() {
        for event in arcade_input_events.iter() {
            if event.value > 0.0 {
                let input = match event.arcade_input {
                    ArcadeInput::JoyUp => GameInput::up(),
                    ArcadeInput::JoyDown => GameInput::down(),
                    ArcadeInput::JoyButton => GameInput::pick(),
                    ArcadeInput::ButtonFront1 => GameInput::exit(),
                    ArcadeInput::ButtonFront2 => GameInput::volume(),
                    _ => return,
                };
                input_buffer.insert(input);
            }
        }
    } else {
        ignore_input_counter.tick();
    }
}

fn handle_input(
    audio: Res<Audio>,
    mut sounds: ResMut<Sounds>,
    mut exit_event: EventWriter<AppExit>,
    mut game_state: ResMut<State<GameState>>,
    mut query: Query<(&mut ButtonMarker, &mut UiColor)>,
    mut input: ResMut<GameInputBuffer>,
) {
    let mut selected_button = None;

    if let Some(input) = input.pop() {
        let mut button_clicked = false;
        let mut direction = None;

        match input {
            GameInput::Direction(Direction::Up) => {
                let audio_source = sounds.sfx.move_player.clone();
                let channel_id = &sounds.channels.sfx;
                audio.play_in_channel(audio_source, channel_id);
                direction = Some(Direction::Up);
            }
            GameInput::Direction(Direction::Down) => {
                let audio_source = sounds.sfx.move_player.clone();
                let channel_id = &sounds.channels.sfx;
                audio.play_in_channel(audio_source, channel_id);
                direction = Some(Direction::Down);
            }
            GameInput::Action(Action::Pick) => {
                let audio_source = sounds.sfx.set_zone.clone();
                let channel_id = &sounds.channels.sfx;
                audio.play_in_channel(audio_source, channel_id);
                button_clicked = true;
            }
            GameInput::Action(Action::Exit) => {
                let audio_source = sounds.sfx.push_box.clone();
                let channel_id = &sounds.channels.sfx;
                audio.play_in_channel(audio_source, channel_id);
                exit_event.send(AppExit);
            }
            GameInput::Action(Action::Volume) => {
                let audio_source = sounds.sfx.toggle_volume.clone();
                let channel_id = &sounds.channels.sfx;
                audio.play_in_channel(audio_source, channel_id);

                if sounds.volume < 0.1 {
                    sounds.volume = 1.0;
                } else {
                    sounds.volume -= 0.25;
                }

                audio.set_volume_in_channel(sounds.volume / 2.0, &sounds.channels.music);
                audio.set_volume_in_channel(sounds.volume, &sounds.channels.sfx);
            }
            _ => (),
        }

        for (button, _) in query.iter_mut() {
            match (&button.kind, button.selected) {
                (ButtonKind::Play, selected) => {
                    if selected {
                        if button_clicked {
                            game_state.set(GameState::stock_selection()).unwrap();
                        }
                        if let Some(direction) = &direction {
                            selected_button = Some(if matches!(direction, Direction::Up) {
                                ButtonKind::Quit
                            } else {
                                ButtonKind::Instructions
                            });
                        } else {
                            selected_button = None;
                        }
                    }
                }
                (ButtonKind::Instructions, selected) => {
                    if selected {
                        if button_clicked {
                            game_state.set(GameState::Instructions).unwrap();
                        }
                        if let Some(direction) = &direction {
                            selected_button = Some(if matches!(direction, Direction::Up) {
                                ButtonKind::Play
                            } else {
                                ButtonKind::Quit
                            });
                        } else {
                            selected_button = None;
                        }
                    }
                }
                (ButtonKind::Quit, selected) => {
                    if selected {
                        if button_clicked {
                            exit_event.send(AppExit);
                        }
                        if let Some(direction) = &direction {
                            selected_button = Some(if matches!(direction, Direction::Up) {
                                ButtonKind::Instructions
                            } else {
                                ButtonKind::Play
                            });
                        } else {
                            selected_button = None;
                        }
                    }
                }
                _ => {}
            };
        }
    }

    for (mut button, mut color) in query.iter_mut() {
        if let Some(selected_kind) = selected_button {
            if selected_kind == button.kind {
                button.selected = true;
            } else {
                button.selected = false;
            }
        }

        if button.selected {
            *color = Colors::PRIMARY_DARK.into();
        } else {
            *color = Colors::TRANSPARENT.into();
        }
    }
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<UiMarker>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn stop_audio(sounds: Res<Sounds>, audio: Res<Audio>) {
    let channel_id = &sounds.channels.music;
    audio.stop_channel(channel_id);
}
