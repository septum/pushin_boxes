mod ui;

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_kira_audio::Audio;

use crate::{
    game::{
        self,
        level::{CameraMarker, PlayerMarker},
        BOX_ENTITY_OFFSET, ENTITY_ON_TOP_OFFSET, ENTITY_SURFACE, MAP_HEIGHT, MAP_WIDTH,
        SPRITE_SIZE,
    },
    resources::{
        brush::{Brush, BrushSprite},
        level::map::MAP_COLS,
        prelude::*,
    },
    state::GameState,
};

use ui::{spawn_ui, UiMarker};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameInputBuffer::new())
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
    windows: Res<Windows>,
    mut cursor_moved_event: EventReader<CursorMoved>,
    mut query: Query<(&mut Transform, &Brush)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraMarker>>,
) {
    for event in cursor_moved_event.iter() {
        let (mut transform, brush) = query.single_mut();
        let (camera, camera_transform) = camera_query.single();
        game::brush::lock_to_grid(
            brush,
            &event.position,
            &camera,
            &camera_transform,
            &windows,
            &mut transform.translation,
        );
    }
}

fn handle_mouse_scroll(
    images: Res<Images>,
    windows: Res<Windows>,
    mut scroll_event: EventReader<MouseWheel>,
    mut query: Query<(&mut Handle<Image>, &mut Transform, &mut Brush)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraMarker>>,
) {
    for scroll in scroll_event.iter() {
        if let MouseScrollUnit::Line = scroll.unit {
            let (mut sprite, mut transform, mut brush) = query.single_mut();
            // from: https://bevy-cheatbook.github.io/cookbook/cursor2world.html
            let window = windows.get_primary().unwrap();

            if let Some(position) = window.cursor_position() {
                let (camera, camera_transform) = camera_query.single();
                let window_size = Vec2::new(window.width() as f32, window.height() as f32);
                let ndc = (position / window_size) * 2.0 - Vec2::ONE;
                let ndc_to_world =
                    camera_transform.compute_matrix() * camera.projection_matrix.inverse();
                let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();

                let x = world_pos.x + (MAP_WIDTH / 2.0);
                let y = world_pos.y + (MAP_HEIGHT / 2.0);

                let x = x as usize / SPRITE_SIZE;
                let y = (MAP_COLS - 1) - (y as usize / ENTITY_SURFACE);

                let position = MapPosition::new(x, y);
                game::level::position::update_brush_translation(
                    &position,
                    &mut transform.translation,
                );

                brush.rotate_sprite(scroll.y > 0.0);

                *sprite = game::brush::to_image(&brush, &images);

                if matches!(brush.current_sprite(), &BrushSprite::Box) {
                    transform.translation.y += BOX_ENTITY_OFFSET as f32;
                } else if matches!(brush.current_sprite(), &BrushSprite::Player) {
                    transform.translation.y += ENTITY_ON_TOP_OFFSET as f32;
                }
            }
        }
    }
}

fn handle_mouse_click(
    windows: Res<Windows>,
    mut buttons: ResMut<Input<MouseButton>>,
    mut level: ResMut<Level>,
    query: Query<&Brush>,
    camera_query: Query<(&Camera, &GlobalTransform), With<CameraMarker>>,
) {
    if buttons.pressed(MouseButton::Left) {
        // workaround for input persistence between systems
        buttons.reset(MouseButton::Left);

        let window = windows.get_primary().unwrap();
        if let Some(position) = window.cursor_position() {
            let brush = query.single();
            let (camera, camera_transform) = camera_query.single();
            game::brush::add_entity_to_map(
                &position,
                &camera,
                &camera_transform,
                &windows,
                &mut level,
                brush,
            );
        }
    }
}

fn handle_keyboard_input(
    mut state: ResMut<State<GameState>>,
    mut level: ResMut<Level>,
    mut keyboard: ResMut<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        level.tag = LevelTag::Test(level.state);
        state.set(GameState::Level).unwrap();
    }

    // workaround for input persistence between systems
    keyboard.clear();
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
