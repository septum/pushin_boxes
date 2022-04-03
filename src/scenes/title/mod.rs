mod ui;

use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::state::GameState;

use super::loading::LoadedAssetsHandles;

#[derive(Component)]
struct CleanupMarker;

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Title).with_system(setup))
            .add_system_set(SystemSet::on_exit(GameState::Title).with_system(cleanup));
    }
}

fn setup(mut commands: Commands, assets_handles: Res<LoadedAssetsHandles>, audio: Res<Audio>) {
    ui::spawn(&mut commands, &assets_handles);
    audio.play_looped(assets_handles.assets.sounds.music_title.clone());
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<CleanupMarker>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
