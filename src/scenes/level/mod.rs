mod ui;

use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use bevy_rust_arcade::{ArcadeInput, ArcadeInputEvent};

use crate::{
    core::{
        self,
        level::{CameraMarker, PlayerMarker},
        state::GameState,
    },
    resources::prelude::*,
    ui::{TextKind, TextMarker},
};
use ui::{spawn_ui, UiMarker};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerAnimation {
            timer: Timer::from_seconds(0.25, true),
            idle_timer: Timer::from_seconds(7.0, false),
            long_idle_timer: Timer::from_seconds(10.0, false),
            initial_index: 0,
            index: 0,
        })
        .add_system_set(
            SystemSet::on_enter(GameState::Level)
                .with_system(spawn_scene)
                .with_system(start_audio),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Level)
                .with_system(gather_input)
                .with_system(handle_input)
                .with_system(update_player_position)
                .with_system(update_counters)
                .with_system(update_map)
                .with_system(check_level_state)
                .with_system(lever_timer_finished),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Level)
                .with_system(cleanup)
                .with_system(stop_audio),
        );
    }
}

fn spawn_scene(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut player_animation: ResMut<PlayerAnimation>,
    level: Res<Level>,
    images: Res<Images>,
    fonts: Res<Fonts>,
) {
    let level_sprite_index = level.get_sprite_index();
    player_animation.timer.reset();
    player_animation.idle_timer.reset();
    player_animation.long_idle_timer.reset();
    player_animation.initial_index = level_sprite_index;
    core::level::spawn(&mut commands, &mut texture_atlases, &level, &images);
    spawn_ui(&mut commands, &level, &fonts);
}

fn start_audio(sounds: Res<Sounds>, music: Res<AudioChannel<Music>>) {
    music.play_looped(sounds.music.level.clone());
}

fn gather_input(
    mut arcade_input_events: EventReader<ArcadeInputEvent>,
    mut input_buffer: ResMut<GameInputBuffer>,
) {
    for event in arcade_input_events.iter() {
        if event.value > 0.0 {
            let input = match event.arcade_input {
                ArcadeInput::JoyUp => GameInput::up(),
                ArcadeInput::JoyDown => GameInput::down(),
                ArcadeInput::JoyLeft => GameInput::left(),
                ArcadeInput::JoyRight => GameInput::right(),
                ArcadeInput::ButtonTop1 => GameInput::undo(),
                ArcadeInput::ButtonTop2 => GameInput::reload(),
                ArcadeInput::ButtonTop3 => GameInput::selection(),
                ArcadeInput::ButtonFront1 => GameInput::exit(),
                ArcadeInput::ButtonFront2 => GameInput::volume(),
                _ => return,
            };
            input_buffer.insert(input);
        }
    }
}

fn handle_input(
    sfx: Res<AudioChannel<Sfx>>,
    music: Res<AudioChannel<Music>>,
    mut sounds: ResMut<Sounds>,
    mut level: ResMut<Level>,
    mut input: ResMut<GameInputBuffer>,
    mut game_state: ResMut<State<GameState>>,
    mut player_animation: ResMut<PlayerAnimation>,
    levels: Res<LevelHandles>,
    level_states: Res<Assets<LevelState>>,
) {
    if !level.no_remaining_zones() {
        if let Some(input) = input.pop() {
            core::input::process(
                &input,
                &mut level,
                &mut game_state,
                &levels,
                &level_states,
                &sfx,
                &music,
                &mut sounds,
                &mut player_animation,
            );
        }
    }
}

fn update_player_position(
    time: Res<Time>,
    level: Res<Level>,
    mut player_animation: ResMut<PlayerAnimation>,
    mut query: Query<(&mut Transform, &mut TextureAtlasSprite), With<PlayerMarker>>,
) {
    let (mut transform, mut sprite) = query.single_mut();
    let level_sprite_index = level.get_sprite_index();
    let delta = time.delta();

    player_animation.timer.tick(delta);
    player_animation.idle_timer.tick(delta);
    player_animation.long_idle_timer.tick(delta);

    if level_sprite_index == 0 {
        if player_animation.idle_timer.just_finished() {
            player_animation.initial_index = 4;
            player_animation.index = 0;
            player_animation.timer.reset();
        }

        if player_animation.long_idle_timer.just_finished() {
            player_animation.initial_index = 5;
            player_animation.index = 0;
            player_animation.timer.reset();
        }
    } else {
        player_animation.idle_timer.reset();
        player_animation.long_idle_timer.reset();
    }

    if player_animation.initial_index != level_sprite_index
        && !player_animation.idle_timer.finished()
        && !player_animation.long_idle_timer.finished()
    {
        player_animation.initial_index = level_sprite_index;
        player_animation.index = 0;
        player_animation.timer.reset();
    }

    if player_animation.timer.just_finished() {
        player_animation.index = (player_animation.index + 1) % 4;
    }

    sprite.index = player_animation.index + (4 * player_animation.initial_index);

    core::level::position::update_player_translation(
        &level.state.player_position,
        &mut transform.translation,
    );
}

fn update_counters(
    level: Res<Level>,
    mut texts: Query<(&mut Text, &TextMarker), With<TextMarker>>,
) {
    for (mut text, counter) in texts.iter_mut() {
        let value = match counter.kind {
            TextKind::Moves => level.moves,
            TextKind::Undos => level.undos,
        };
        core::ui::update_dynamic_text(&mut text, value.to_string());
    }
}

fn update_map(
    level: Res<Level>,
    images: Res<Images>,
    mut query: Query<(&mut Handle<Image>, &mut Transform, &MapPosition)>,
) {
    for (mut image, mut transform, position) in query.iter_mut() {
        let map_entity = level.get_entity(position);

        *image = core::level::entity::to_image(map_entity, &images);
        core::level::position::update_entity_translation(position, &mut transform.translation);

        if matches!(map_entity, MapEntity::B | MapEntity::P) {
            transform.translation.y += core::BOX_ENTITY_OFFSET as f32;
            transform.translation.z += 1.0;
        }
    }
}

fn check_level_state(time: Res<Time>, mut level: ResMut<Level>) {
    if level.no_remaining_zones() {
        level.tick_timer(time.delta());
    }
}

fn lever_timer_finished(level: Res<Level>, mut game_state: ResMut<State<GameState>>) {
    if level.timer_finished() {
        game_state.set(GameState::Win).unwrap();
    }
}

fn cleanup(
    mut commands: Commands,
    entities: Query<
        Entity,
        Or<(
            With<UiMarker>,
            With<PlayerMarker>,
            With<CameraMarker>,
            With<MapPosition>,
        )>,
    >,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn stop_audio(music: Res<AudioChannel<Music>>) {
    music.stop();
}
