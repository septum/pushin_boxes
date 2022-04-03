mod ui;

use bevy::prelude::*;
use bevy_asset_ron::RonAssetPlugin;

use crate::{assets::GameAssets, level::LevelData, state::GameState};

#[derive(Component)]
struct CleanupMarker;

pub struct LoadingPlugin;

pub struct LoadedAssetsHandles {
    pub assets: GameAssets,
}

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<LevelData>::new(&["lvl"]))
            .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_loading))
            .add_system_set(SystemSet::on_exit(GameState::Loading).with_system(cleanup));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = GameAssets::load(&asset_server);
    ui::spawn(&mut commands, &assets);
    commands.insert_resource(LoadedAssetsHandles { assets });
}

fn check_loading(
    mut state: ResMut<State<GameState>>,
    loaded_assets_handles: Res<LoadedAssetsHandles>,
    asset_server: Res<AssetServer>,
) {
    if loaded_assets_handles.assets.all_loaded(&asset_server) {
        state.set(GameState::Title).unwrap();
    }
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<CleanupMarker>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
