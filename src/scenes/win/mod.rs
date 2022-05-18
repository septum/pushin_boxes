mod ui;

use bevy::prelude::{Input, *};
use bevy_kira_audio::Audio;

use crate::{game, resources::prelude::*, state::GameState};

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
    mut level_handles: ResMut<LevelHandles>,
    asset_server: Res<AssetServer>,
    keyboard: Res<Input<KeyCode>>,
    level_states_assets: Res<Assets<LevelState>>,
    level: Res<Level>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        match &level.tag {
            LevelTag::Stock(current_index) => {
                if !game::level::stock::is_last(&level.tag) {
                    game::save_file::stock::unlock(&mut save_file, &level);
                    game::level::stock::insert(
                        &mut commands,
                        *current_index + 1,
                        &save_file,
                        &level_handles,
                        &level_states_assets,
                    );
                    game_state.set(GameState::Level).unwrap();
                } else {
                    game_state.set(GameState::stock_selection()).unwrap();
                }
            }
            LevelTag::Custom(_) => {
                game_state.set(GameState::custom_selection()).unwrap();
            }
            LevelTag::Test(level_state) => {
                game::level::custom::write(
                    &level,
                    &mut level_handles,
                    level_state,
                    &asset_server,
                    &mut save_file,
                );
                game_state.set(GameState::Title).unwrap();
            }
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
