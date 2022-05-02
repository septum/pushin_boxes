mod ui;

use bevy::{app::AppExit, prelude::*};
use bevy_kira_audio::Audio;

use crate::{
    game::save_file,
    resources::prelude::*,
    state::GameState,
    ui::{ButtonKind, ButtonMarker},
};

use ui::{spawn_ui, UiMarker};

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Title)
                .with_system(setup)
                .with_system(start_audio),
        )
        .add_system_set(SystemSet::on_update(GameState::Title).with_system(interactions))
        .add_system_set(
            SystemSet::on_exit(GameState::Title)
                .with_system(cleanup)
                .with_system(stop_audio),
        );
    }
}

fn setup(mut commands: Commands, fonts: Res<Fonts>) {
    spawn_ui(&mut commands, &fonts);
}

fn start_audio(sounds: Res<Sounds>, audio: Res<Audio>) {
    let audio_source = sounds.music.title.clone();
    let channel_id = &sounds.channels.music;
    audio.play_looped_in_channel(audio_source, channel_id);
}

fn interactions(
    mut exit_event: EventWriter<AppExit>,
    mut game_state: ResMut<State<GameState>>,
    mut query: Query<
        (&ButtonMarker, &Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    save_file: Res<SaveFile>,
) {
    for (button, interaction, mut color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                match button.kind {
                    ButtonKind::Play => {
                        if save_file::is_default(&save_file) {
                            game_state.set(GameState::Instructions).unwrap();
                        } else {
                            game_state.set(GameState::stock_selection()).unwrap();
                        }
                    }
                    ButtonKind::Editor => {
                        game_state.set(GameState::Editor).unwrap();
                    }
                    ButtonKind::Options => {
                        // TODO: Set game_state to show options scene
                    }
                    ButtonKind::Quit => {
                        exit_event.send(AppExit);
                    }
                    _ => (),
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
