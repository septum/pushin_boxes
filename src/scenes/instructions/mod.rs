mod ui;

use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::{assets::LoadedHandles, state::GameState};

#[derive(Component)]
struct CleanupMarker;

pub struct InstructionsPlugin;

impl Plugin for InstructionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Instructions).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Instructions).with_system(interactions))
            .add_system_set(SystemSet::on_exit(GameState::Instructions).with_system(cleanup));
    }
}

fn setup(mut commands: Commands, loaded_handles: Res<LoadedHandles>) {
    ui::spawn(&mut commands, &loaded_handles.assets);
}

fn interactions(mut state: ResMut<State<GameState>>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        state.set(GameState::Selection).unwrap();
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
