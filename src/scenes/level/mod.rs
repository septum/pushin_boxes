mod ui;

use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::{
    input::{ActionKind, DirectionKind, InputBuffer, InputKind},
    level::{Level, LevelState, LevelTag, MapPosition, PlayerMarker},
    resources::ResourcesHandles,
    state::{GameState, SelectionKind},
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
    resources: Res<ResourcesHandles>,
    level: Res<Level>,
    audio: Res<Audio>,
) {
    level.spawn_map(&mut commands, &resources.assets.images);
    level.spawn_player(&mut commands, &resources.assets.images, CleanupMarker);

    ui::spawn(&mut commands, &resources.assets, &level);

    audio.play_looped(resources.assets.sounds.music.level.clone());
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
    resources: Res<ResourcesHandles>,
) {
    if let Some(input) = input.buffer.pop() {
        match input {
            InputKind::Direction(direction) => level.handle_direction_input(&direction),
            InputKind::Action(action) => {
                match action {
                    ActionKind::Undo => level.restore_snapshot(),
                    ActionKind::Reload => level.reload(&loaded_levels, &resources.assets.levels),
                    ActionKind::Selection => match &level.tag {
                        LevelTag::Stock(_) => state
                            .set(GameState::Selection(SelectionKind::Stock))
                            .unwrap(),
                        LevelTag::Custom(_) => state
                            .set(GameState::Selection(SelectionKind::Custom))
                            .unwrap(),
                        LevelTag::Test(_) => state.set(GameState::Editor).unwrap(),
                    },
                    ActionKind::Exit => state.set(GameState::Title).unwrap(),
                };
            }
        }
    }
}

fn update_player_position(level: Res<Level>, mut query: Query<&mut Transform, With<PlayerMarker>>) {
    let mut transform = query.single_mut();
    level.update_player_position(&mut transform);
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
    resources: Res<ResourcesHandles>,
    mut query: Query<(&mut Handle<Image>, &MapPosition), With<MapPosition>>,
) {
    for (mut handle, position) in query.iter_mut() {
        level.update_entity_texture(position, &mut handle, &resources.assets.images);
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
