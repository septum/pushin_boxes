mod ui;

use bevy::prelude::{Input, *};
use bevy_kira_audio::Audio;

use crate::{resources::prelude::*, state::GameState};

use ui::{spawn_ui, UiMarker};
pub struct InstructionsPlugin;

impl Plugin for InstructionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Instructions)
                .with_system(setup)
                .with_system(start_audio),
        )
        .add_system_set(SystemSet::on_update(GameState::Instructions).with_system(keyboard_input))
        .add_system_set(
            SystemSet::on_exit(GameState::Instructions)
                .with_system(cleanup)
                .with_system(stop_audio),
        );
    }
}

fn setup(mut commands: Commands, images: Res<Images>, fonts: Res<Fonts>) {
    spawn_ui(&mut commands, &images, &fonts);
}

fn start_audio(sounds: Res<Sounds>, audio: Res<Audio>) {
    let audio_source = sounds.music.title.clone();
    let channel_id = &sounds.channels.music;
    audio.play_looped_in_channel(audio_source, channel_id);
}

fn keyboard_input(mut game_state: ResMut<State<GameState>>, mut keyboard: ResMut<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        game_state.set(GameState::stock_selection()).unwrap();
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Title).unwrap();
    }

    // workaround for input persistence between states
    keyboard.clear();
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