#![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;

use pushin_boxes::{
    config::ConfigPlugin,
    resources::{prelude::*, ResourcesPlugin},
    scenes::ScenesPlugin,
    state::GameState,
};

fn main() {
    App::new()
        .add_plugin(ConfigPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(ResourcesPlugin)
        .add_plugin(ScenesPlugin)
        .insert_resource(ClearColor(Colors::DARK))
        .add_state(GameState::Startup)
        .run();
}
