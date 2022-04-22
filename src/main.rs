use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;

use pushin_boxes::{
    config::ConfigPlugin,
    resources::{Colors, ResourcesPlugin},
    scenes::ScenesPlugin,
    state::GameState,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Colors::DARK))
        .add_plugin(ConfigPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(ResourcesPlugin)
        .add_plugin(ScenesPlugin)
        .add_state(GameState::Startup)
        .add_startup_system(setup)
        .run();
}

// TODO: Move this for each scene
fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
