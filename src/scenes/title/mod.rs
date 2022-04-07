mod ui;

use bevy::prelude::*;
use bevy_kira_audio::Audio;

use super::loading::LoadedAssetsHandles;
use crate::state::GameState;

#[derive(Component)]
struct CleanupMarker;

enum ButtonKind {
    Play,
    Options,
    Quit,
}

#[derive(Component)]
struct ButtonMarker {
    kind: ButtonKind,
}

impl ButtonMarker {
    fn new(kind: ButtonKind) -> ButtonMarker {
        ButtonMarker { kind }
    }
}

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
                    // TODO: Change this to `Selection` when available
                    state.set(GameState::Level).unwrap();
                }
                ButtonKind::Options => { /* TODO: Set state to show options scene */ }
                ButtonKind::Quit => { /* TODO: Set state to quit game */ }
            },
            Interaction::Hovered => { /* TODO: Modify button style */ }
            _ => (),
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
