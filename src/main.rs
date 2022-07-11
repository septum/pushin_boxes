#![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use bevy_rust_arcade::RustArcadePlugin;

use pushin_boxes::{
    config::ConfigPlugin,
    core::state::GameState,
    resources::{prelude::*, ResourcesPlugin},
    scenes::ScenesPlugin,
};

fn main() {
    App::new()
        .add_plugin(ConfigPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(RustArcadePlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(ResourcesPlugin)
        .add_plugin(ScenesPlugin)
        .insert_resource(IgnoreInputCounter::default())
        .insert_resource(ClearColor(Colors::DARK))
        .add_state(GameState::Startup)
        .run();
}
