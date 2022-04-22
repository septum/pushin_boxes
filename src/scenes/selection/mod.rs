mod ui;

use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::{
    level::{Level, LevelState, LevelTag},
    resources::{ResourcesHandles, SaveFile},
    state::{GameState, SelectionKind},
    ui::{ButtonKind, ButtonMarker, LevelKind},
};

#[derive(Component)]
struct CleanupMarker;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Selection(SelectionKind::Stock)).with_system(setup),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Selection(SelectionKind::Custom)).with_system(setup),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Selection(SelectionKind::Stock))
                .with_system(select_level),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Selection(SelectionKind::Custom))
                .with_system(select_level),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Selection(SelectionKind::Stock)).with_system(cleanup),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Selection(SelectionKind::Custom)).with_system(cleanup),
        );
    }
}

fn setup(
    mut commands: Commands,
    state: Res<State<GameState>>,
    resources: Res<ResourcesHandles>,
    audio: Res<Audio>,
    save_file: Res<SaveFile>,
) {
    if let GameState::Selection(selection_kind) = state.current() {
        ui::spawn(&mut commands, &resources.assets, &save_file, selection_kind);
    }

    audio.play_looped(resources.assets.sounds.music.selection.clone());
}

fn select_level(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    loaded_levels: Res<Assets<LevelState>>,
    resources: Res<ResourcesHandles>,
    save_file: Res<SaveFile>,
    query: Query<(&ButtonMarker, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    if let Ok((button, interaction)) = query.get_single() {
        match interaction {
            Interaction::Clicked => match &button.kind {
                ButtonKind::Level(level_kind) => {
                    let level = match level_kind {
                        LevelKind::Stock(index) => Level::load(
                            LevelTag::Stock(index.clone()),
                            &save_file,
                            &loaded_levels,
                            &resources.assets.levels,
                        ),
                        LevelKind::Custom(uuid) => Level::load(
                            LevelTag::Custom(uuid.clone()),
                            &save_file,
                            &loaded_levels,
                            &resources.assets.levels,
                        ),
                    };

                    commands.insert_resource(level);

                    state.set(GameState::Level).unwrap();
                }
                ButtonKind::Levels => {
                    if let GameState::Selection(selection_kind) = state.current() {
                        match selection_kind {
                            SelectionKind::Stock => state
                                .set(GameState::Selection(SelectionKind::Custom))
                                .unwrap(),
                            SelectionKind::Custom => state
                                .set(GameState::Selection(SelectionKind::Stock))
                                .unwrap(),
                        }
                    }
                }
                _ => {}
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
