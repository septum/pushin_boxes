use bevy::{prelude::*, window::WindowMode};

pub struct ConfigPlugin;

const GAME_TITLE: &str = "Pushin' Boxes";

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: GAME_TITLE.to_string(),
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        });
    }
}
