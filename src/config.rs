use bevy::{
    app::Plugin as BevyPlugin, image::ImageSamplerDescriptor, prelude::*, window::WindowMode,
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
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor::nearest(),
                }),
        )
        .insert_resource(ClearColor(Colors::DARK))
        .init_state::<GameState>()
        .add_event::<ActionInputEvent>()
        .add_event::<DirectionInputEvent>()
        .add_event::<SceneTransitionEvent>()
        .add_event::<LevelInsertionEvent>();
    }
}
