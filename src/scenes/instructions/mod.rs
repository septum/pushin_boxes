mod ui;

use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use bevy_rust_arcade::ArcadeInputEvent;

use crate::{
    core::state::GameState,
    resources::{input::Action, prelude::*},
};

use ui::{spawn_ui, UiMarker};
pub struct InstructionsPlugin;

impl Plugin for InstructionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Instructions)
                .with_system(setup)
                .with_system(start_audio),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Instructions)
                .with_system(gather_input)
                .with_system(handle_input),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Instructions)
                .with_system(cleanup)
                .with_system(stop_audio),
        );
    }
}

fn setup(
    mut commands: Commands,
    fonts: Res<Fonts>,
    mut ignore_input_counter: ResMut<IgnoreInputCounter>,
) {
    spawn_ui(&mut commands, &fonts);
    ignore_input_counter.start();
}

fn start_audio(sounds: Res<Sounds>, music: Res<AudioChannel<Music>>) {
    music.play_looped(sounds.music.title.clone());
}
fn gather_input(
    mut arcade_input_events: EventReader<ArcadeInputEvent>,
    mut input_buffer: ResMut<GameInputBuffer>,
    mut ignore_input_counter: ResMut<IgnoreInputCounter>,
) {
    if ignore_input_counter.done() {
        for event in arcade_input_events.iter() {
            if event.value > 0.0 {
                input_buffer.insert(GameInput::pick());
            }
        }
    } else {
        ignore_input_counter.tick();
    }
}

fn handle_input(
    sfx: Res<AudioChannel<Sfx>>,
    sounds: Res<Sounds>,
    mut game_state: ResMut<State<GameState>>,
    mut input: ResMut<GameInputBuffer>,
) {
    if let Some(GameInput::Action(Action::Pick)) = input.pop() {
        sfx.play(sounds.sfx.set_zone.clone());
        game_state.set(GameState::stock_selection()).unwrap();
    }
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<UiMarker>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn stop_audio(music: Res<AudioChannel<Music>>) {
    music.stop();
}
