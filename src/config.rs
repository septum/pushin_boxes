use bevy::prelude::*;

pub struct ConfigPlugin;

const GAME_TITLE: &str = "Pushin' Boxes";
const GAME_WIDTH: f32 = 640.0;
const GAME_HEIGHT: f32 = 640.0;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: GAME_TITLE.to_string(),
            width: GAME_WIDTH,
            height: GAME_HEIGHT,
            ..Default::default()
        });
    }
}
