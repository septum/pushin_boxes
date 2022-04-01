use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_kira_audio::AudioPlugin;

use pushin_boxes::{
    assets::GAME_COLORS, config::ConfigPlugin, scenes::ScenesPlugin, state::GameState,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(GAME_COLORS.dark))
        .add_plugin(ConfigPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(ScenesPlugin)
        .add_state(GameState::Loading)
        .add_startup_system(setup)
        .add_system(exit_on_esc_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
