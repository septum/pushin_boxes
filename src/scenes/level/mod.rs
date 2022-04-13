mod ui;

use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::{
    assets::LoadedHandles,
    input::{ActionKind, DirectionKind, InputBuffer, InputKind},
    level::{Level, LevelState, MapPosition},
    player::{Player, PlayerMarker},
    state::GameState,
    ui::DynamicText,
};

#[derive(Component)]
struct CleanupMarker;

enum CounterKind {
    Moves,
    Undos,
}

#[derive(Component)]
struct CounterMarker {
    kind: CounterKind,
}

impl CounterMarker {
    fn new(kind: CounterKind) -> CounterMarker {
        CounterMarker { kind }
    }
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputBuffer::new())
            .add_system_set(SystemSet::on_enter(GameState::Level).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameState::Level)
                    .with_system(gather_input)
                    .with_system(handle_input)
                    .with_system(update_player_position)
                    .with_system(update_counters)
                    .with_system(update_map)
                    .with_system(check_level_done),
            )
            .add_system_set(SystemSet::on_exit(GameState::Level).with_system(cleanup));
    }
}

fn setup(
    mut commands: Commands,
    loaded_handles: Res<LoadedHandles>,
    level: Res<Level>,
    audio: Res<Audio>,
) {
    let player = Player::new(level.state.player_position);

    level.spawn_map(&mut commands, &loaded_handles.assets.images);
    player.spawn(&mut commands, &loaded_handles.assets.images, CleanupMarker);
    ui::spawn(&mut commands, &loaded_handles.assets, &level);

    audio.play_looped(loaded_handles.assets.sounds.music_level.clone());
}

// TODO: Implement keybindings
fn gather_input(keyboard: Res<Input<KeyCode>>, mut input: ResMut<InputBuffer>) {
    if keyboard.just_pressed(KeyCode::W) || keyboard.just_pressed(KeyCode::Up) {
        input.insert(InputKind::Direction(DirectionKind::Up));
    }

    if keyboard.just_pressed(KeyCode::S) || keyboard.just_pressed(KeyCode::Down) {
        input.insert(InputKind::Direction(DirectionKind::Down));
    }

    if keyboard.just_pressed(KeyCode::A) || keyboard.just_pressed(KeyCode::Left) {
        input.insert(InputKind::Direction(DirectionKind::Left));
    }

    if keyboard.just_pressed(KeyCode::D) || keyboard.just_pressed(KeyCode::Right) {
        input.insert(InputKind::Direction(DirectionKind::Right));
    }

    if keyboard.just_pressed(KeyCode::U) {
        input.insert(InputKind::Action(ActionKind::Undo));
    }

    if keyboard.just_pressed(KeyCode::R) {
        input.insert(InputKind::Action(ActionKind::Reload));
    }

    if keyboard.just_pressed(KeyCode::L) {
        input.insert(InputKind::Action(ActionKind::Selection));
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        input.insert(InputKind::Action(ActionKind::Exit));
    }
}

// TODO: Add sounds
fn handle_input(
    mut level: ResMut<Level>,
    mut input: ResMut<InputBuffer>,
    mut state: ResMut<State<GameState>>,
    loaded_levels: Res<Assets<LevelState>>,
    loaded_handles: Res<LoadedHandles>,
) {
    if let Some(input) = input.buffer.pop() {
        match input {
            InputKind::Direction(direction) => level.handle_direction_input(&direction),
            InputKind::Action(action) => {
                match action {
                    ActionKind::Undo => level.restore_snapshot(),
                    ActionKind::Reload => {
                        level.reload(&loaded_levels, &loaded_handles.assets.levels)
                    }
                    ActionKind::Selection => state.set(GameState::Selection).unwrap(),
                    ActionKind::Exit => state.set(GameState::Title).unwrap(),
                };
            }
        }
    }
}

fn update_player_position(level: Res<Level>, mut query: Query<&mut Transform, With<PlayerMarker>>) {
    let mut transform = query.single_mut();
    level
        .state
        .player_position
        .apply_to_transform(&mut transform);
}

fn update_counters(
    level: Res<Level>,
    mut texts: Query<(&mut Text, &CounterMarker), With<CounterMarker>>,
) {
    for (mut text, counter) in texts.iter_mut() {
        let counter = match counter.kind {
            CounterKind::Moves => level.moves,
            CounterKind::Undos => level.undos,
        };
        DynamicText::update(&mut text, counter.to_string());
    }
}

fn update_map(
    level: Res<Level>,
    loaded_handles: Res<LoadedHandles>,
    mut query: Query<(&mut Handle<Image>, &MapPosition), With<MapPosition>>,
) {
    for (mut handle, position) in query.iter_mut() {
        level.update_entity_texture(position, &mut handle, &loaded_handles.assets.images);
    }
}

fn check_level_done(mut state: ResMut<State<GameState>>, level: Res<Level>) {
    if level.no_zones_left() {
        state.set(GameState::Win).unwrap()
    }
}

fn cleanup(
    mut commands: Commands,
    entities: Query<Entity, Or<(With<CleanupMarker>, With<MapPosition>)>>,
    audio: Res<Audio>,
) {
    // TODO: Move this somewhere else
    audio.stop();

    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
