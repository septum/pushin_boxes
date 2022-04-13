mod ui;

use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::{
    assets::{LoadedHandles, SaveFile},
    config::MAX_TOTAL_LEVELS,
    level::{Level, LevelState},
    state::GameState,
};

#[derive(Component)]
struct CleanupMarker;

pub struct WinPlugin;

impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Win).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Win).with_system(interactions))
            .add_system_set(SystemSet::on_exit(GameState::Win).with_system(cleanup));
    }
}

fn setup(
    mut commands: Commands,
    save_file: Res<SaveFile>,
    loaded_handles: Res<LoadedHandles>,
    level: Res<Level>,
    audio: Res<Audio>,
) {
    ui::spawn(&mut commands, &loaded_handles.assets, &level, &save_file);
    audio.play_looped(loaded_handles.assets.sounds.music_win.clone());
}

fn interactions(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    mut save_file: ResMut<SaveFile>,
    keyboard: Res<Input<KeyCode>>,
    level: Res<Level>,
    loaded_levels: Res<Assets<LevelState>>,
    loaded_handles: Res<LoadedHandles>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        save_file.set_if_new_record(level.index, level.moves);
        save_file.unlock_next_level(level.index);
        save_file.save();

        if level.index + 1 < MAX_TOTAL_LEVELS {
            let level = Level::load(
                level.index + 1,
                save_file.get_record(level.index + 1),
                &loaded_levels,
                &loaded_handles.assets.levels,
            );

            commands.insert_resource(level);

            state.set(GameState::Level).unwrap();
        } else {
            state.set(GameState::Selection).unwrap();
        }
    }
}

fn cleanup(
    mut commands: Commands,
    entities: Query<Entity, With<CleanupMarker>>,
    audio: Res<Audio>,
) {
    // TODO: Move this somewhere else
    audio.stop();

    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
