use bevy_kira_audio::AudioPlugin;

use bevy::{
    app::Plugin as BevyPlugin, asset::AssetMetaCheck, image::ImageSamplerDescriptor, prelude::*,
    window::WindowMode,
};

use crate::{level::LevelInsertionEvent, resources::prelude::*};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Pushin' Boxes".to_string(),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor::nearest(),
                }),
            AudioPlugin,
        ))
        .insert_resource(ClearColor(game_ui::Colors::DARK))
        .init_state::<GameState>()
        .add_event::<ActionInputEvent>()
        .add_event::<DirectionInputEvent>()
        .add_event::<SceneTransitionEvent>()
        .add_event::<LevelInsertionEvent>();
    }
}
