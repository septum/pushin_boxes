use bevy::{
    app::Plugin as BevyPlugin, prelude::*, render::texture::ImageSamplerDescriptor,
    window::WindowMode,
};

use crate::resources::prelude::*;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Pushin' Boxes".to_string(),
                        mode: WindowMode::BorderlessFullscreen,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor::nearest(),
                }),
        )
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Colors::DARK))
        .add_state::<GameState>()
        .add_event::<ActionInputEvent>()
        .add_event::<DirectionInputEvent>()
        .add_event::<SceneTransitionEvent>()
        .add_event::<LevelInsertionEvent>();
    }
}
