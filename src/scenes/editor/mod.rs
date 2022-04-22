mod ui;

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_kira_audio::Audio;

use crate::{
    config::{GAME_HEIGHT, GAME_WIDTH, MAP_COLS, SPRITE_SIZE},
    input::InputBuffer,
    level::{Level, LevelTag, MapEntity, MapPosition, PlayerMarker},
    resources::ResourcesHandles,
    state::GameState,
};

#[derive(Component)]
struct CleanupMarker;

enum BrushSprite {
    Box,
    Player,
    Wall,
    Floor,
    Zone,
}

impl BrushSprite {
    pub fn next(&self) -> BrushSprite {
        match self {
            BrushSprite::Box => BrushSprite::Player,
            BrushSprite::Player => BrushSprite::Wall,
            BrushSprite::Wall => BrushSprite::Floor,
            BrushSprite::Floor => BrushSprite::Zone,
            BrushSprite::Zone => BrushSprite::Box,
        }
    }

    pub fn prev(&self) -> BrushSprite {
        match self {
            BrushSprite::Box => BrushSprite::Zone,
            BrushSprite::Player => BrushSprite::Box,
            BrushSprite::Wall => BrushSprite::Player,
            BrushSprite::Floor => BrushSprite::Wall,
            BrushSprite::Zone => BrushSprite::Floor,
        }
    }
}

#[derive(Component)]
struct Brush {
    sprite: BrushSprite,
}

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputBuffer::new())
            .add_system_set(SystemSet::on_enter(GameState::Editor).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameState::Editor)
                    .with_system(cursor)
                    .with_system(scroll)
                    .with_system(click)
                    .with_system(keyboard)
                    .with_system(update_player_position)
                    .with_system(update_map),
            )
            .add_system_set(SystemSet::on_exit(GameState::Editor).with_system(cleanup));
    }
}

fn setup(mut commands: Commands, resources: Res<ResourcesHandles>, audio: Res<Audio>) {
    let mut level = Level::new();
    let player_position = MapPosition::new(4, 4);

    level.state.player_position = player_position;

    level.spawn_map(&mut commands, &resources.assets.images);
    level.spawn_player(&mut commands, &resources.assets.images, CleanupMarker);

    commands.insert_resource(level);

    ui::spawn(&mut commands, &resources.assets);

    commands
        .spawn_bundle(SpriteBundle {
            texture: resources.assets.images.entities.pbox.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            ..Default::default()
        })
        .insert(Brush {
            sprite: BrushSprite::Box,
        });

    audio.play_looped(resources.assets.sounds.music.level.clone());
}

fn keyboard(
    mut state: ResMut<State<GameState>>,
    mut level: ResMut<Level>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        level.tag = LevelTag::Test(level.state.clone());
        state.set(GameState::Level).unwrap();
    }
}

fn cursor(
    mut cursor_moved_event: EventReader<CursorMoved>,
    mut query: Query<&mut Transform, With<Brush>>,
) {
    for event in cursor_moved_event.iter() {
        if event.position.x < GAME_WIDTH && event.position.y < GAME_HEIGHT {
            let mut transform = query.single_mut();
            let x = event.position.x as usize / SPRITE_SIZE;
            let y = (MAP_COLS - 1) - (event.position.y as usize / SPRITE_SIZE);
            let position = MapPosition::new(x, y);

            position.apply_to_transform(&mut transform);
        }
    }
}

fn scroll(
    resources: Res<ResourcesHandles>,
    mut scroll_event: EventReader<MouseWheel>,
    mut query: Query<(&mut Handle<Image>, &mut Brush), With<Brush>>,
) {
    for scroll in scroll_event.iter() {
        if let MouseScrollUnit::Line = scroll.unit {
            let (mut handle, mut brush) = query.single_mut();
            let images = &resources.assets.images;

            brush.sprite = if scroll.y > 0.0 {
                brush.sprite.next()
            } else {
                brush.sprite.prev()
            };

            match brush.sprite {
                BrushSprite::Box => *handle = images.entities.pbox.clone(),
                BrushSprite::Player => *handle = images.player.idle.clone(),
                BrushSprite::Wall => *handle = images.entities.wall.clone(),
                BrushSprite::Floor => *handle = images.entities.floor.clone(),
                BrushSprite::Zone => *handle = images.entities.zone.clone(),
            }
        }
    }
}

fn click(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut level: ResMut<Level>,
    query: Query<&Brush>,
) {
    if buttons.pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        if let Some(position) = window.cursor_position() {
            if position.x < GAME_WIDTH && position.y < GAME_HEIGHT {
                let brush = query.single();

                let x = position.x as usize / SPRITE_SIZE;
                let y = (MAP_COLS - 1) - (position.y as usize / SPRITE_SIZE);
                let position = MapPosition::new(x, y);

                match brush.sprite {
                    BrushSprite::Player => match level.get_entity(&position) {
                        MapEntity::F | MapEntity::Z => level.move_player(position),
                        _ => {}
                    },
                    _ => {
                        if !level.player_in(&position) {
                            if let MapEntity::Z = level.get_entity(&position) {
                                level.decrement_remaining_zones();
                            }

                            match brush.sprite {
                                BrushSprite::Box => match level.get_entity(&position) {
                                    MapEntity::Z => level.set_entity(&position, MapEntity::P),
                                    _ => level.set_entity(&position, MapEntity::B),
                                },
                                BrushSprite::Wall => level.set_entity(&position, MapEntity::W),
                                BrushSprite::Floor => level.set_entity(&position, MapEntity::F),
                                BrushSprite::Zone => {
                                    level.set_entity(&position, MapEntity::Z);
                                    level.increment_remaining_zones();
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
}

fn update_player_position(level: Res<Level>, mut query: Query<&mut Transform, With<PlayerMarker>>) {
    let mut transform = query.single_mut();
    level.update_player_position(&mut transform);
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
