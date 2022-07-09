mod ui;

use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_rust_arcade::{ArcadeInput, ArcadeInputEvent};

use crate::{
    core::{self, state::GameState},
    resources::{
        input::{Action, Direction},
        prelude::*,
    },
    ui::{ButtonKind, ButtonMarker, LevelKind},
};

use ui::{spawn_ui, UiMarker};

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        add_systems_lifecycle(app, GameState::stock_selection());
    }
}

fn add_systems_lifecycle(app: &mut App, state: GameState) {
    app.add_system_set(
        SystemSet::on_enter(state.clone())
            .with_system(setup)
            .with_system(start_audio),
    )
    .add_system_set(
        SystemSet::on_update(state.clone())
            .with_system(gather_input)
            .with_system(handle_input),
    )
    .add_system_set(
        SystemSet::on_exit(state)
            .with_system(cleanup)
            .with_system(stop_audio),
    );
}

fn setup(
    mut commands: Commands,
    fonts: Res<Fonts>,
    save_file: Res<SaveFile>,
    mut ignore_input_counter: ResMut<IgnoreInputCounter>,
) {
    spawn_ui(&mut commands, &fonts, &save_file);
    ignore_input_counter.start();
}

fn start_audio(sounds: Res<Sounds>, audio: Res<Audio>) {
    let audio_source = sounds.music.selection.clone();
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
                    ArcadeInput::JoyLeft => GameInput::left(),
                    ArcadeInput::JoyRight => GameInput::right(),
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
    mut commands: Commands,
    level_states_assets: Res<Assets<LevelState>>,
    level_handles: Res<LevelHandles>,
    save_file: Res<SaveFile>,
    audio: Res<Audio>,
    mut sounds: ResMut<Sounds>,
    mut game_state: ResMut<State<GameState>>,
    mut query: Query<(&mut ButtonMarker, &mut UiColor)>,
    mut input: ResMut<GameInputBuffer>,
) {
    let mut selected_button = None;

    if let Some(input) = input.pop() {
        let mut button_clicked = false;
        let mut direction = None;

        match input {
            GameInput::Direction(Direction::Up) => direction = Some(Direction::Up),
            GameInput::Direction(Direction::Down) => direction = Some(Direction::Down),
            GameInput::Direction(Direction::Left) => direction = Some(Direction::Left),
            GameInput::Direction(Direction::Right) => direction = Some(Direction::Right),
            GameInput::Action(Action::Pick) => button_clicked = true,
            GameInput::Action(Action::Exit) => game_state.set(GameState::Title).unwrap(),
            GameInput::Action(Action::Volume) => {
                if sounds.volume < 0.1 {
                    sounds.volume = 1.0;
                } else {
                    sounds.volume -= 0.25;
                }
                audio.set_volume_in_channel(sounds.volume, &sounds.channels.sfx);
                audio.set_volume_in_channel(sounds.volume, &sounds.channels.music);
            }
            _ => (),
        }

        for (button, _) in query.iter_mut() {
            match (&button.kind, button.selected) {
                (ButtonKind::Level(LevelKind::Stock(index)), selected) => {
                    if selected {
                        if button_clicked {
                            core::level::stock::insert(
                                &mut commands,
                                *index,
                                &save_file,
                                &level_handles,
                                &level_states_assets,
                            );
                            game_state.set(GameState::Level).unwrap();
                        }
                        if let Some(direction) = &direction {
                            let index = index + 1;
                            let value = match direction {
                                Direction::Up => {
                                    if index.saturating_sub(4) >= 1 {
                                        index.saturating_sub(4)
                                    } else {
                                        index + 12
                                    }
                                }
                                Direction::Down => {
                                    if index + 4 <= 16 {
                                        index + 4
                                    } else {
                                        index.saturating_sub(12)
                                    }
                                }
                                Direction::Left => {
                                    if index.saturating_sub(1) >= 1 {
                                        index.saturating_sub(1)
                                    } else {
                                        index + 15
                                    }
                                }
                                Direction::Right => {
                                    if index + 1 <= 16 {
                                        index + 1
                                    } else {
                                        index.saturating_sub(15)
                                    }
                                }
                            };
                            let value = if value < save_file.stock_levels_len() {
                                value
                            } else {
                                save_file.stock_levels_len()
                            };

                            selected_button = Some(value - 1);
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
        if let Some(selected_index) = selected_button {
            if let ButtonKind::Level(LevelKind::Stock(index)) = button.kind {
                if selected_index == index {
                    button.selected = true;
                } else {
                    button.selected = false;
                }
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
