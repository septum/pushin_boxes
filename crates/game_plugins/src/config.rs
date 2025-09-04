use bevy_kira_audio::AudioPlugin;

use bevy::{
    app::Plugin as BevyPlugin, asset::AssetMetaCheck, image::ImageSamplerDescriptor, prelude::*,
    window::WindowMode,
};

use crate::{
    input::{self, InputEvent},
    level::{self, LevelInsertionEvent},
    save_file,
    state::{self, GameState, GameStateTransitionEvent},
};

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
        .insert_resource(ClearColor(crate::theme::DARK))
        .init_state::<GameState>()
        .add_event::<InputEvent>()
        .add_event::<GameStateTransitionEvent>()
        .add_event::<LevelInsertionEvent>()
        .add_plugins((
            save_file::Plugin,
            level::Plugin,
            input::Plugin,
            state::Plugin,
        ))
        .add_systems(OnExit(GameState::Loading), camera_setup);
    }
}

pub fn camera_setup(mut commands: Commands) {
    commands.spawn((Camera2d, Msaa::Off));
}
