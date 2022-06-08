mod ui;

use bevy::{
    input::{keyboard::KeyboardInput, ElementState},
    prelude::*,
};
use bevy_kira_audio::Audio;

use crate::{
    game::{
        self,
        level::{CameraMarker, PlayerMarker},
    },
    resources::prelude::*,
    state::GameState,
    ui::{TextKind, TextMarker},
};
use ui::{spawn_ui, UiMarker};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameInputBuffer::new())
            .add_system_set(
                SystemSet::on_enter(GameState::Level)
                    .with_system(spawn_scene)
                    .with_system(start_music),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Level)
                    .with_system(gather_input)
                    .with_system(handle_input)
                    .with_system(update_player_position)
                    .with_system(update_counters)
                    .with_system(update_map)
                    .with_system(check_level_state),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Level)
                    .with_system(cleanup)
                    .with_system(stop_music),
            );
    }
}

fn spawn_scene(mut commands: Commands, level: Res<Level>, images: Res<Images>, fonts: Res<Fonts>) {
    game::level::spawn(&mut commands, &level, &images);
    spawn_ui(&mut commands, &level, &fonts);
}

fn start_music(audio: Res<Audio>, sounds: Res<Sounds>) {
    let audio_source = sounds.music.level.clone();
    let channel_id = &sounds.channels.music;
    audio.play_looped_in_channel(audio_source, channel_id);
}

// TODO: Implement keybindings
fn gather_input(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut input_buffer: ResMut<GameInputBuffer>,
) {
    for event in keyboard_events.iter() {
        if let ElementState::Pressed = event.state {
            if let Some(keycode) = event.key_code {
                // workaround for input persistence between states
                keyboard_input.reset(keycode);

                let input = match keycode {
                    KeyCode::W => GameInput::up(),
                    KeyCode::S => GameInput::down(),
                    KeyCode::A => GameInput::left(),
                    KeyCode::D => GameInput::right(),
                    KeyCode::U => GameInput::undo(),
                    KeyCode::R => GameInput::reload(),
                    KeyCode::L => GameInput::selection(),
                    KeyCode::Escape => GameInput::exit(),
                    _ => return,
                };

                input_buffer.insert(input);
            };
        };
    }
}

fn handle_input(
    mut level: ResMut<Level>,
    mut input: ResMut<GameInputBuffer>,
    mut game_state: ResMut<State<GameState>>,
    levels: Res<LevelHandles>,
    level_states: Res<Assets<LevelState>>,
) {
    if let Some(input) = input.pop() {
        game::input::process(&input, &mut level, &mut game_state, &levels, &level_states);
    }
}

fn update_player_position(level: Res<Level>, mut query: Query<&mut Transform, With<PlayerMarker>>) {
    let mut transform = query.single_mut();
    game::level::position::update_player_translation(
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
        game::ui::update_dynamic_text(&mut text, value.to_string());
    }
}

fn update_map(
    level: Res<Level>,
    images: Res<Images>,
    mut query: Query<(&mut Handle<Image>, &mut Transform, &MapPosition)>,
) {
    for (mut image, mut transform, position) in query.iter_mut() {
        let map_entity = level.get_entity(position);

        *image = game::level::entity::to_image(map_entity, &images);
        game::level::position::update_entity_translation(position, &mut transform.translation);

        if matches!(map_entity, MapEntity::B | MapEntity::P) {
            transform.translation.y += game::BOX_ENTITY_OFFSET as f32;
            transform.translation.z += 1.0;
        }
    }
}

fn check_level_state(level: Res<Level>, mut game_state: ResMut<State<GameState>>) {
    if level.no_remaining_zones() {
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

fn stop_music(audio: Res<Audio>, sounds: Res<Sounds>) {
    let channel_id = &sounds.channels.music;
    audio.stop_channel(channel_id)
}
