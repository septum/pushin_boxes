use bevy::{
    app::Plugin as BevyPlugin, prelude::*, render::texture::ImageSettings, window::WindowMode,
};
use iyes_loopless::prelude::*;

use crate::resources::prelude::*;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "Pushin' Boxes".to_string(),
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(ClearColor(Colors::DARK))
        .add_loopless_state(GameState::Loading)
        .add_event::<ActionEvent>()
        .add_event::<DirectionEvent>()
        .add_event::<SceneTransitionEvent>()
        .add_event::<LevelInsertionEvent>();
    }
}
