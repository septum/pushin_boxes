mod ui;

use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_rust_arcade::{ArcadeInput, ArcadeInputEvent};

use crate::{
    core::{self, state::GameState},
    resources::{input::Action, prelude::*},
};

use ui::{spawn_ui, UiMarker};

pub struct WinPlugin;

impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Win)
                .with_system(save_record)
                .with_system(setup)
                .with_system(start_audio),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Win)
                .with_system(gather_input)
                .with_system(handle_input),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Win)
                .with_system(cleanup)
                .with_system(stop_audio),
        );
    }
}

fn save_record(mut save_file: ResMut<SaveFile>, level: Res<Level>) {
    core::save_file::set_if_new_record(&mut save_file, &level.tag, level.moves);
    core::save_file::save(&save_file);
}

fn setup(mut commands: Commands, save_file: Res<SaveFile>, fonts: Res<Fonts>, level: Res<Level>) {
    spawn_ui(&mut commands, &fonts, &level, &save_file);
}

fn start_audio(sounds: Res<Sounds>, audio: Res<Audio>) {
    let audio_source = sounds.music.win.clone();
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
                    ArcadeInput::ButtonFront1 => GameInput::exit(),
                    ArcadeInput::ButtonFront2 => GameInput::volume(),
                    ArcadeInput::JoyButton => GameInput::pick(),
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
    mut save_file: ResMut<SaveFile>,
    level_handles: Res<LevelHandles>,
    level_states_assets: Res<Assets<LevelState>>,
    level: Res<Level>,
    audio: Res<Audio>,
    mut sounds: ResMut<Sounds>,
    mut game_state: ResMut<State<GameState>>,
    mut input: ResMut<GameInputBuffer>,
) {
    if let Some(input) = input.pop() {
        match input {
            GameInput::Action(Action::Pick) => {
                let audio_source = sounds.sfx.set_zone.clone();
                let channel_id = &sounds.channels.sfx;
                audio.play_in_channel(audio_source, channel_id);
                match &level.tag {
                    LevelTag::Stock(current_index) => {
                        if core::level::stock::is_last(&level.tag) {
                            game_state.set(GameState::stock_selection()).unwrap();
                        } else {
                            core::save_file::stock::unlock(&mut save_file, &level);
                            core::level::stock::insert(
                                &mut commands,
                                *current_index + 1,
                                &save_file,
                                &level_handles,
                                &level_states_assets,
                            );
                            game_state.set(GameState::Level).unwrap();
                        }
                    }
                }
            }
            GameInput::Action(Action::Exit) => {
                let audio_source = sounds.sfx.push_box.clone();
                let channel_id = &sounds.channels.sfx;
                audio.play_in_channel(audio_source, channel_id);
                game_state.set(GameState::Title).unwrap();
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
