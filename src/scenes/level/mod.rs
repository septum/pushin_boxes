mod ui;

use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::{
    level::{Counters, Level, LevelData},
    state::GameState,
    ui::DynamicText,
};

use super::loading::LoadedAssetsHandles;

#[derive(Component)]
struct CleanupMarker;

enum CounterKind {
    Moves,
    Undos,
}

#[derive(Component)]
struct CounterMarker {
    kind: CounterKind,
}

impl CounterMarker {
    fn new(kind: CounterKind) -> CounterMarker {
        CounterMarker { kind }
    }
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Level).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Level).with_system(update_counters))
            .add_system_set(SystemSet::on_exit(GameState::Level).with_system(cleanup));
    }
}

fn setup(
    mut commands: Commands,
    assets_handles: Res<LoadedAssetsHandles>,
    level_data_assets: Res<Assets<LevelData>>,
    current_level: Res<Level>,
    audio: Res<Audio>,
) {
    // TODO: Draw the level
    let _level_data = level_data_assets.get(&current_level.data_handle).unwrap();
    ui::spawn(&mut commands, &assets_handles.assets, &current_level);
    audio.play_looped(assets_handles.assets.sounds.music_level.clone());
}

fn update_counters(
    counters: Res<Counters>,
    mut texts: Query<(&mut Text, &CounterMarker), With<CounterMarker>>,
) {
    for (mut text, counter) in texts.iter_mut() {
        match counter.kind {
            CounterKind::Moves => DynamicText::update(&mut text, counters.moves.to_string()),
            CounterKind::Undos => DynamicText::update(&mut text, counters.undos.to_string()),
        }
    }
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<CleanupMarker>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
