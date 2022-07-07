mod ui;

use bevy::prelude::{Input, *};
use bevy_kira_audio::Audio;

use crate::{
    core::{self, state::GameState},
    resources::prelude::*,
    ui::{ButtonKind, ButtonMarker, LevelKind},
};

use ui::{spawn_ui, UiMarker};

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        add_systems_lifecycle(app, GameState::stock_selection());
    }
}

fn add_systems_lifecycle(app: &mut App, state: GameState) {
    app.add_system_set(
        SystemSet::on_enter(state.clone())
            .with_system(setup)
            .with_system(start_audio),
    )
    .add_system_set(
        SystemSet::on_update(state.clone())
            .with_system(select_level)
            .with_system(keyboard_input),
    )
    .add_system_set(
        SystemSet::on_exit(state)
            .with_system(cleanup)
            .with_system(stop_audio),
    );
}

fn setup(mut commands: Commands, fonts: Res<Fonts>, save_file: Res<SaveFile>) {
    spawn_ui(&mut commands, &fonts, &save_file);
}

fn start_audio(sounds: Res<Sounds>, audio: Res<Audio>) {
    let audio_source = sounds.music.selection.clone();
    let channel_id = &sounds.channels.music;
    audio.play_looped_in_channel(audio_source, channel_id);
}

fn select_level(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    mut mouse_button_input: ResMut<Input<MouseButton>>,
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
                // workaround for input persistence between states
                // see: https://github.com/bevyengine/bevy/issues/1700#issuecomment-886999222
                mouse_button_input.reset(MouseButton::Left);

                if let ButtonKind::Level(level_kind) = &button.kind {
                    let LevelKind::Stock(index) = level_kind;
                    core::level::stock::insert(
                        &mut commands,
                        *index,
                        &save_file,
                        &level_handles,
                        &level_states_assets,
                    );
                    state.set(GameState::Level).unwrap();
                }

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

fn keyboard_input(mut game_state: ResMut<State<GameState>>, mut keyboard: ResMut<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Title).unwrap();
    }

    // workaround for input persistence between states
    keyboard.clear();
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
