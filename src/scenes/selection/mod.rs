mod ui;

use bevy::prelude::*;
use bevy_kira_audio::Audio;

use super::loading::LoadedAssetsHandles;
use crate::{
    level::{Counters, Level},
    state::GameState,
};

#[derive(Component)]
struct CleanupMarker;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Selection).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Selection).with_system(select_level))
            .add_system_set(SystemSet::on_exit(GameState::Selection).with_system(cleanup));
    }
}

fn setup(mut commands: Commands, assets_handles: Res<LoadedAssetsHandles>, audio: Res<Audio>) {
    ui::spawn(&mut commands, &assets_handles.assets);
    audio.play_looped(assets_handles.assets.sounds.music_selection.clone());
}

// this will currently auto select the level 1
// TODO: Implement the expected functionality
fn select_level(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    assets_handles: Res<LoadedAssetsHandles>,
) {
    commands.insert_resource(Level {
        number: 1,
        record: 0,
        data_handle: assets_handles.assets.levels.collection[0].clone(),
    });
    commands.insert_resource(Counters { moves: 0, undos: 4 });
    state.set(GameState::Level).unwrap();
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
