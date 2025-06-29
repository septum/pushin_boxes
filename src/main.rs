#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;

use game_plugins::{assets, config, scenes};

fn main() {
    App::new()
        .add_plugins((config::Plugin, assets::Plugin, scenes::Plugin))
        .run();
}
