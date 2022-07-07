mod ui;

use bevy::prelude::{Input, *};
use bevy_kira_audio::Audio;

use crate::{
    game::{self, state::GameState},
    resources::prelude::*,
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
        .add_system_set(SystemSet::on_update(GameState::Win).with_system(interactions))
        .add_system_set(
            SystemSet::on_exit(GameState::Win)
                .with_system(cleanup)
                .with_system(stop_audio),
        );
    }
}

fn save_record(mut save_file: ResMut<SaveFile>, level: Res<Level>) {
    game::save_file::set_if_new_record(&mut save_file, &level.tag, level.moves);
    game::save_file::save(&save_file);
}

fn setup(mut commands: Commands, save_file: Res<SaveFile>, fonts: Res<Fonts>, level: Res<Level>) {
    spawn_ui(&mut commands, &fonts, &level, &save_file);
}

fn start_audio(sounds: Res<Sounds>, audio: Res<Audio>) {
    let audio_source = sounds.music.win.clone();
    let channel_id = &sounds.channels.music;
    audio.play_looped_in_channel(audio_source, channel_id);
}

fn interactions(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut save_file: ResMut<SaveFile>,
    mut keyboard: ResMut<Input<KeyCode>>,
    level_handles: Res<LevelHandles>,
    level_states_assets: Res<Assets<LevelState>>,
    level: Res<Level>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        match &level.tag {
            LevelTag::Stock(current_index) => {
                if game::level::stock::is_last(&level.tag) {
                    game_state.set(GameState::stock_selection()).unwrap();
                } else {
                    game::save_file::stock::unlock(&mut save_file, &level);
                    game::level::stock::insert(
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
