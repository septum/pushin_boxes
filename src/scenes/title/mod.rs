mod ui;

use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::{
    assets::{LoadedHandles, SaveFile},
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

fn setup(mut commands: Commands, loaded_handles: Res<LoadedHandles>, audio: Res<Audio>) {
    ui::spawn(&mut commands, &loaded_handles.assets);
    audio.play_looped(loaded_handles.assets.sounds.music_title.clone());
}

fn interactions(
    mut state: ResMut<State<GameState>>,
    save_file: Res<SaveFile>,
    query: Query<(&ButtonMarker, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    if let Ok((button, interaction)) = query.get_single() {
        match interaction {
            Interaction::Clicked => match button.kind {
                ButtonKind::Play => {
                    if save_file.no_records() {
                        state.set(GameState::Instructions).unwrap();
                    } else {
                        state.set(GameState::Selection).unwrap();
                    }
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
    audio: Res<Audio>,
    save_file: Res<SaveFile>,
    entities: Query<Entity, With<CleanupMarker>>,
) {
    // TODO: Move this somewhere else
    if !save_file.no_records() {
        audio.stop();
    }

    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
