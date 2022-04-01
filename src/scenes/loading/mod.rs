mod ui;

use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_asset_ron::RonAssetPlugin;

use crate::{assets::Assets, level::LevelData, state::GameState};

#[derive(Component)]
struct CleanupMarker;

pub struct LoadingPlugin;

pub struct LoadedAssetsHandles {
    pub assets: Assets,
}

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<LevelData>::new(&["lvl"]))
            .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameState::Loading).with_system(check_loading_state),
            )
            .add_system_set(SystemSet::on_exit(GameState::Loading).with_system(cleanup));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = Assets::load(asset_server);

    ui::spawn(&mut commands, assets.fonts.fredoka.clone());

    commands.insert_resource(LoadedAssetsHandles { assets });
}

fn check_loading_state(
    mut state: ResMut<State<GameState>>,
    loaded_assets_handles: Res<LoadedAssetsHandles>,
    asset_server: Res<AssetServer>,
) {
    for asset in loaded_assets_handles.assets.as_array_untyped() {
        if asset_server.get_load_state(asset) != LoadState::Loaded {
            return;
        }
    }
    state.set(GameState::Title).unwrap();
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<CleanupMarker>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
