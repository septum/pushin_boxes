mod ui;

use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use bevy_rust_arcade::{ArcadeInput, ArcadeInputEvent};

use crate::{
    core::{self, state::GameState},
    resources::{input::Action, prelude::*},
};

use ui::{spawn_ui, UiMarker};

#[derive(Component)]
pub struct CameraMarker;

#[derive(Component)]
pub struct CharacterMarker;

pub struct CharacterAnimation {
    pub timer: Timer,
    pub row: usize,
    pub index: usize,
}

pub struct WinPlugin;

impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CharacterAnimation {
            timer: Timer::from_seconds(0.125, true),
            row: 6,
            index: 0,
        })
        .add_system_set(
            SystemSet::on_enter(GameState::Win)
                .with_system(save_record)
                .with_system(setup)
                .with_system(start_audio),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Win)
                .with_system(gather_input)
                .with_system(handle_input)
                .with_system(unpdate_character_sprite),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Win)
                .with_system(cleanup)
                .with_system(stop_audio),
        );
    }
}

fn save_record(mut save_file: ResMut<SaveFile>, level: Res<Level>) {
    core::save_file::set_if_new_record(
        &mut save_file,
        &level.tag,
        level.moves,
        level.stopwatch.elapsed().as_secs_f32(),
    );
    core::save_file::stock::unlock(&mut save_file, &level);
    core::save_file::save(&save_file);
}

fn setup(
    mut commands: Commands,
    fonts: Res<Fonts>,
    images: Res<Images>,
    level: Res<Level>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale *= 0.75;
    commands.spawn_bundle(camera_bundle).insert(CameraMarker);

    let texture_atlas = TextureAtlas::from_grid_with_padding(
        images.player.spritesheet.clone(),
        Vec2::new(64.0, 64.0),
        4,
        7,
        Vec2::new(4.0, 4.0),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                ..TextureAtlasSprite::default()
            },
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::new(160.0, 16.0, 1.0)),
            ..SpriteSheetBundle::default()
        })
        .insert(CharacterMarker);
    spawn_ui(&mut commands, &fonts, &level);
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

fn unpdate_character_sprite(
    time: Res<Time>,
    mut character_animation: ResMut<CharacterAnimation>,
    mut query: Query<&mut TextureAtlasSprite, With<CharacterMarker>>,
) {
    let mut sprite = query.single_mut();
    let delta = time.delta();

    character_animation.timer.tick(delta);

    if character_animation.timer.just_finished() {
        character_animation.index = (character_animation.index + 1) % 4;
    }

    sprite.index = character_animation.index + (4 * character_animation.row);
}

fn cleanup(
    mut commands: Commands,
    entities: Query<Entity, Or<(With<UiMarker>, With<CharacterMarker>, With<CameraMarker>)>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn stop_audio(music: Res<AudioChannel<Music>>) {
    music.stop();
}
