mod ui;

use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::{
    resources::{ResourcesHandles, SaveFile},
    state::{GameState, SelectionKind},
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

fn setup(mut commands: Commands, resources: Res<ResourcesHandles>, audio: Res<Audio>) {
    ui::spawn(&mut commands, &resources.assets);
    audio.play_looped(resources.assets.sounds.music.title.clone());
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
                        state
                            .set(GameState::Selection(SelectionKind::Stock))
                            .unwrap();
                    }
                }
                ButtonKind::Options => { /* TODO: Set state to show options scene */ }
                ButtonKind::Quit => { /* TODO: Set state to quit game */ }
                ButtonKind::Editor => {
                    state.set(GameState::Editor).unwrap();
                }
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
    entities: Query<Entity, With<CleanupMarker>>,
) {
    // TODO: Move this somewhere else
    audio.stop();

    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
