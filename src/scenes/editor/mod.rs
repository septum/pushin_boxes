mod ui;

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::{Input, *},
};
use bevy_kira_audio::Audio;

use crate::{
    game::{
        self,
        level::{CameraMarker, PlayerMarker},
    },
    resources::{brush::Brush, prelude::*},
    state::GameState,
};

use ui::{spawn_ui, UiMarker};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputBuffer::new())
            .add_system_set(
                SystemSet::on_enter(GameState::Editor)
                    .with_system(spawn_scene)
                    .with_system(start_music),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Editor)
                    .with_system(handle_cursor_movement)
                    .with_system(handle_mouse_scroll)
                    .with_system(handle_mouse_click)
                    .with_system(handle_keyboard_input)
                    .with_system(update_player_position)
                    .with_system(update_map),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Editor)
                    .with_system(cleanup)
                    .with_system(stop_music),
            );
    }
}

fn spawn_scene(mut commands: Commands, images: Res<Images>) {
    let level = Level::default();

    spawn_ui(&mut commands);
    game::level::spawn(&mut commands, &level, &images);
    game::brush::spawn(&mut commands, &images);

    commands.insert_resource(level);
}

fn start_music(audio: Res<Audio>, sounds: Res<Sounds>) {
    let audio_source = sounds.music.selection.clone();
    let channel_id = &sounds.channels.music;
    audio.play_looped_in_channel(audio_source, channel_id);
}

fn handle_cursor_movement(
    mut cursor_moved_event: EventReader<CursorMoved>,
    mut query: Query<&mut Transform, With<Brush>>,
) {
    for event in cursor_moved_event.iter() {
        let mut transform = query.single_mut();
        game::brush::lock_brush_to_map_grid(&event.position, &mut transform.translation);
    }
}

fn handle_mouse_scroll(
    images: Res<Images>,
    mut scroll_event: EventReader<MouseWheel>,
    mut query: Query<(&mut Handle<Image>, &mut Brush), With<Brush>>,
) {
    for scroll in scroll_event.iter() {
        if let MouseScrollUnit::Line = scroll.unit {
            let (mut sprite, mut brush) = query.single_mut();
            brush.rotate_sprite(scroll.y > 0.0);
            *sprite = game::brush::to_image(&brush, &images);
        }
    }
}

fn handle_mouse_click(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut level: ResMut<Level>,
    query: Query<&Brush>,
) {
    if buttons.pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        if let Some(position) = window.cursor_position() {
            let brush = query.single();
            game::brush::add_entity_to_map(&position, &mut level, brush);
        }
    }
}

fn handle_keyboard_input(
    mut state: ResMut<State<GameState>>,
    mut level: ResMut<Level>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        level.tag = LevelTag::Test(level.state);
        state.set(GameState::Level).unwrap();
    }
}

fn update_player_position(level: Res<Level>, mut query: Query<&mut Transform, With<PlayerMarker>>) {
    let mut transform = query.single_mut();
    game::level::position::update_player_translation(
        &level.state.player_position,
        &mut transform.translation,
    );
}

fn update_map(
    level: Res<Level>,
    images: Res<Images>,
    mut query: Query<(&mut Handle<Image>, &MapPosition)>,
) {
    for (mut image, position) in query.iter_mut() {
        let entity = level.get_entity(position);
        let new_image = game::level::entity::to_image(entity, &images);
        *image = new_image;
    }
}

fn cleanup(
    mut commands: Commands,
    entities: Query<
        Entity,
        Or<(
            With<UiMarker>,
            With<CameraMarker>,
            With<PlayerMarker>,
            With<MapPosition>,
            With<Brush>,
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
