mod ui;

use bevy::prelude::*;
use bevy_kira_audio::Audio;

use super::loading::LoadedAssetsHandles;
use crate::{
    state::GameState,
    ui::{ButtonKind, ButtonMarker},
};

#[derive(Component)]
struct CleanupMarker;

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Title).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Title).with_system(interactions))
            .add_system_set(SystemSet::on_exit(GameState::Title).with_system(cleanup));
    }
}

fn setup(mut commands: Commands, assets_handles: Res<LoadedAssetsHandles>, audio: Res<Audio>) {
    ui::spawn(&mut commands, &assets_handles.assets);
    audio.play_looped(assets_handles.assets.sounds.music_title.clone());
}

fn interactions(
    mut state: ResMut<State<GameState>>,
    query: Query<(&ButtonMarker, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    if let Ok((button, interaction)) = query.get_single() {
        match interaction {
            Interaction::Clicked => match button.kind {
                ButtonKind::Play => {
                    state.set(GameState::Selection).unwrap();
                }
                ButtonKind::Options => { /* TODO: Set state to show options scene */ }
                ButtonKind::Quit => { /* TODO: Set state to quit game */ }
                _ => (),
            },
            Interaction::Hovered => { /* TODO: Modify button style */ }
            Interaction::None => {}
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
