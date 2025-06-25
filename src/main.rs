#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;

use game_plugins::{config, resources, scenes};

fn main() {
    App::new()
        .add_plugins((config::Plugin, resources::Plugin, scenes::Plugin))
        .run();
}
