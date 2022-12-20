use bevy::{
    app::Plugin as BevyPlugin, prelude::*, render::texture::ImageSampler, window::WindowMode,
};
use iyes_loopless::prelude::*;

use crate::resources::prelude::*;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Pushin' Boxes".to_string(),
                        mode: WindowMode::BorderlessFullscreen,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSampler::nearest_descriptor(),
                }),
        )
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Colors::DARK))
        .add_loopless_state(GameState::Loading)
        .add_event::<ActionInputEvent>()
        .add_event::<DirectionInputEvent>()
        .add_event::<SceneTransitionEvent>()
        .add_event::<LevelInsertionEvent>();
    }
}
