mod ui;

use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use bevy_rust_arcade::{ArcadeInput, ArcadeInputEvent};

use crate::{
    core::{self, state::GameState},
    resources::{input::Action, prelude::*},
};

use ui::{spawn_ui, UiMarker};

pub struct WinPlugin;

impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Win)
                .with_system(save_record)
                .with_system(setup)
                .with_system(start_audio),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Win)
                .with_system(gather_input)
                .with_system(handle_input),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Win)
                .with_system(cleanup)
                .with_system(stop_audio),
        );
    }
}

fn save_record(mut save_file: ResMut<SaveFile>, level: Res<Level>) {
    core::save_file::set_if_new_record(&mut save_file, &level.tag, level.moves);
    core::save_file::stock::unlock(&mut save_file, &level);
    core::save_file::save(&save_file);
}

fn setup(mut commands: Commands, save_file: Res<SaveFile>, fonts: Res<Fonts>, level: Res<Level>) {
    spawn_ui(&mut commands, &fonts, &level, &save_file);
}

fn start_audio(sounds: Res<Sounds>, music: Res<AudioChannel<Music>>) {
    music.play_looped(sounds.music.win.clone());
}

fn gather_input(
    mut arcade_input_events: EventReader<ArcadeInputEvent>,
    mut input_buffer: ResMut<GameInputBuffer>,
    mut ignore_input_counter: ResMut<IgnoreInputCounter>,
) {
    if ignore_input_counter.done() {
        for event in arcade_input_events.iter() {
            if event.value > 0.0 {
                match event.arcade_input {
                    ArcadeInput::JoyUp
                    | ArcadeInput::JoyDown
                    | ArcadeInput::JoyLeft
                    | ArcadeInput::JoyRight => return,
                    _ => input_buffer.insert(GameInput::pick()),
                }
            }
        }
    } else {
        ignore_input_counter.tick();
    }
}

fn handle_input(
    mut commands: Commands,
    save_file: Res<SaveFile>,
    level_handles: Res<LevelHandles>,
    level_states_assets: Res<Assets<LevelState>>,
    level: Res<Level>,
    sfx: Res<AudioChannel<Sfx>>,
    sounds: Res<Sounds>,
    mut game_state: ResMut<State<GameState>>,
    mut input: ResMut<GameInputBuffer>,
) {
    if let Some(GameInput::Action(Action::Pick)) = input.pop() {
        sfx.play(sounds.sfx.set_zone.clone());

        match &level.tag {
            LevelTag::Stock(current_index) => {
                if core::level::stock::is_last(&level.tag) {
                    game_state.set(GameState::stock_selection()).unwrap();
                } else {
                    core::level::stock::insert(
                        &mut commands,
                        *current_index + 1,
                        &save_file,
                        &level_handles,
                        &level_states_assets,
                    );
                    game_state.set(GameState::Level).unwrap();
                }
            }
        }
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
