mod ui;

use bevy::prelude::*;
use bevy_kira_audio::Audio;

use super::loading::LoadedAssetsHandles;
use crate::{
    level::{Level, LevelState},
    state::GameState,
    ui::{ButtonKind, ButtonMarker},
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
    // TODO: Get unlocked levels from save file
    let unlocked_levels = vec![0];
    ui::spawn(&mut commands, &assets_handles.assets, &unlocked_levels);
    audio.play_looped(assets_handles.assets.sounds.music_selection.clone());
}

fn select_level(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    loaded_levels: Res<Assets<LevelState>>,
    assets_handles: Res<LoadedAssetsHandles>,
    query: Query<(&ButtonMarker, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    if let Ok((button, interaction)) = query.get_single() {
        match interaction {
            Interaction::Clicked => {
                if let ButtonKind::Level(index) = button.kind {
                    // TODO: Get unlocked levels from save file
                    let unlocked_levels = vec![0];
                    let level = Level::load(
                        index,
                        unlocked_levels[index],
                        &loaded_levels,
                        &assets_handles.assets.levels,
                    );

                    commands.insert_resource(level);

                    state.set(GameState::Level).unwrap();
                }
            }
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
