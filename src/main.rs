#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;

use pushin_boxes::{config, resources, scenes};

fn main() {
    App::new()
        .add_plugins((
            config::Plugin,
            AudioPlugin,
            resources::Plugin,
            scenes::Plugin,
        ))
        .run();
}
