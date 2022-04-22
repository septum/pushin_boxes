mod ui;

use std::{env, fs::File, io::Write, path::PathBuf};

use bevy::prelude::*;
use bevy_kira_audio::Audio;
use uuid::Uuid;

use crate::{
    config::MAX_TOTAL_LEVELS,
    level::{Level, LevelState, LevelTag},
    resources::{ResourcesHandles, SaveFile},
    state::{GameState, SelectionKind},
};

#[derive(Component)]
struct CleanupMarker;

pub struct WinPlugin;

impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Win).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Win).with_system(interactions))
            .add_system_set(SystemSet::on_exit(GameState::Win).with_system(cleanup));
    }
}

fn setup(
    mut commands: Commands,
    save_file: Res<SaveFile>,
    resources: Res<ResourcesHandles>,
    level: Res<Level>,
    audio: Res<Audio>,
) {
    ui::spawn(&mut commands, &resources.assets, &level, &save_file);
    audio.play_looped(resources.assets.sounds.music.win.clone());
}

fn interactions(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    mut save_file: ResMut<SaveFile>,
    mut resources: ResMut<ResourcesHandles>,
    asset_server: Res<AssetServer>,
    keyboard: Res<Input<KeyCode>>,
    level: Res<Level>,
    loaded_levels: Res<Assets<LevelState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        match &level.tag {
            LevelTag::Stock(index) => {
                save_file.set_if_new_record(&level.tag, &level.moves);
                save_file.unlock_next_stock_level(&level.tag);
                save_file.save();

                // TODO: Move this somewhere else
                if index + 1 < MAX_TOTAL_LEVELS {
                    let level = Level::load(
                        LevelTag::Stock(index + 1),
                        &save_file.into(),
                        &loaded_levels,
                        &resources.assets.levels,
                    );
                    commands.insert_resource(level);

                    state.set(GameState::Level).unwrap();
                } else {
                    state
                        .set(GameState::Selection(SelectionKind::Stock))
                        .unwrap();
                }
            }
            LevelTag::Custom(_) => {
                save_file.set_if_new_record(&level.tag, &level.moves);
                save_file.save();
                state
                    .set(GameState::Selection(SelectionKind::Custom))
                    .unwrap();
            }
            LevelTag::Test(level_state) => {
                let uuid = Uuid::new_v4();
                let tag = LevelTag::Custom(uuid);
                let path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
                    PathBuf::from(manifest_dir)
                        .join("assets/levels/custom")
                        .join(format!("{}.lvl", uuid))
                } else {
                    PathBuf::from("assets/levels/custom").join(format!("{}.lvl", uuid))
                };

                if let Ok(serialized_string) = ron::ser::to_string(&level_state) {
                    let mut file = File::create(path).unwrap();
                    file.write_all(serialized_string.as_bytes()).unwrap();
                }

                resources.assets.levels.custom.insert(
                    uuid,
                    asset_server.load(&format!("levels/custom/{}.lvl", uuid)),
                );

                save_file.set_record(&tag, level.moves.clone());
                save_file.save();

                state.set(GameState::Title).unwrap();
            }
        }
    }
}

fn cleanup(
    mut commands: Commands,
    entities: Query<Entity, With<CleanupMarker>>,
    audio: Res<Audio>,
) {
    // TODO: Move this somewhere else
    audio.stop();

    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
