mod ui;

use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::{
    game,
    resources::prelude::*,
    state::{GameState, Selection},
    ui::{ButtonKind, ButtonMarker, LevelKind},
};

use ui::{spawn_ui, UiMarker};

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Cleanup this mess
        app.add_system_set(
            SystemSet::on_enter(GameState::stock_selection())
                .with_system(setup)
                .with_system(start_audio),
        )
        .add_system_set(
            SystemSet::on_update(GameState::stock_selection()).with_system(select_level),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::stock_selection())
                .with_system(cleanup)
                .with_system(stop_audio),
        );

        app.add_system_set(
            SystemSet::on_enter(GameState::custom_selection())
                .with_system(setup)
                .with_system(start_audio),
        )
        .add_system_set(
            SystemSet::on_update(GameState::custom_selection()).with_system(select_level),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::custom_selection())
                .with_system(cleanup)
                .with_system(stop_audio),
        );
    }
}

fn setup(
    mut commands: Commands,
    state: Res<State<GameState>>,
    fonts: Res<Fonts>,
    save_file: Res<SaveFile>,
) {
    if let GameState::Selection(selection_kind) = state.current() {
        spawn_ui(&mut commands, &fonts, &save_file, selection_kind);
    }
}

fn start_audio(sounds: Res<Sounds>, audio: Res<Audio>) {
    let audio_source = sounds.music.selection.clone();
    let channel_id = &sounds.channels.music;
    audio.play_looped_in_channel(audio_source, channel_id);
}

fn select_level(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    level_states_assets: Res<Assets<LevelState>>,
    level_handles: Res<LevelHandles>,
    save_file: Res<SaveFile>,
    mut query: Query<
        (&ButtonMarker, &Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    if let Ok((button, interaction, mut color)) = query.get_single_mut() {
        match interaction {
            Interaction::Clicked => {
                match &button.kind {
                    ButtonKind::Level(level_kind) => {
                        match level_kind {
                            LevelKind::Stock(index) => {
                                game::level::stock::insert(
                                    &mut commands,
                                    *index,
                                    &save_file,
                                    &level_handles,
                                    &level_states_assets,
                                );
                            }
                            LevelKind::Custom(uuid) => {
                                game::level::custom::insert(
                                    &mut commands,
                                    uuid,
                                    &save_file,
                                    &level_handles,
                                    &level_states_assets,
                                );
                            }
                        };

                        state.set(GameState::Level).unwrap();
                    }
                    ButtonKind::Levels => {
                        if let GameState::Selection(selection_kind) = state.current() {
                            match selection_kind {
                                Selection::Stock => {
                                    state.set(GameState::custom_selection()).unwrap()
                                }
                                Selection::Custom => {
                                    state.set(GameState::stock_selection()).unwrap()
                                }
                            }
                        }
                    }
                    _ => {}
                };

                *color = Colors::PRIMARY_DARK.into();
            }
            Interaction::Hovered => {
                *color = Colors::PRIMARY_LIGHT.into();
            }
            Interaction::None => {
                *color = Colors::PRIMARY.into();
            }
        }
    }
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<UiMarker>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn stop_audio(sounds: Res<Sounds>, audio: Res<Audio>) {
    let channel_id = &sounds.channels.music;
    audio.stop_channel(channel_id);
}
