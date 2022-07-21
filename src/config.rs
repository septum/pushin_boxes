use bevy::{prelude::*, window::WindowMode};

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "Pushin' Boxes".to_string(),
            width: 1280.0,
            height: 1024.0,
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        });
    }
}
