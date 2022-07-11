mod ui;

use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_rust_arcade::{ArcadeInput, ArcadeInputEvent};

use crate::{
    core::state::GameState,
    resources::{input::Action, prelude::*},
};

use ui::{spawn_ui, UiMarker};
pub struct InstructionsPlugin;

impl Plugin for InstructionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Instructions)
                .with_system(setup)
                .with_system(start_audio),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Instructions)
                .with_system(gather_input)
                .with_system(handle_input),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Instructions)
                .with_system(cleanup)
                .with_system(stop_audio),
        );
    }
}

fn setup(
    mut commands: Commands,
    images: Res<Images>,
    fonts: Res<Fonts>,
    mut ignore_input_counter: ResMut<IgnoreInputCounter>,
) {
    spawn_ui(&mut commands, &images, &fonts);
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
    audio: Res<Audio>,
    mut sounds: ResMut<Sounds>,
    mut game_state: ResMut<State<GameState>>,
    mut input: ResMut<GameInputBuffer>,
) {
    if let Some(input) = input.pop() {
        match input {
            GameInput::Action(Action::Pick) => {
                let audio_source = sounds.sfx.move_player.clone();
                let channel_id = &sounds.channels.sfx;
                audio.play_in_channel(audio_source, channel_id);
                game_state.set(GameState::stock_selection()).unwrap()
            }
            GameInput::Action(Action::Exit) => {
                let audio_source = sounds.sfx.push_box.clone();
                let channel_id = &sounds.channels.sfx;
                audio.play_in_channel(audio_source, channel_id);
                game_state.set(GameState::Title).unwrap()
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
