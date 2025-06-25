mod animations;
mod brush;
mod camera;
mod fonts;
mod images;
mod input;
mod level;
mod save_file;
mod scene;
mod sounds;
mod state;
mod systems;

pub mod prelude {
    pub use super::animations::{BLINK_ROW_LAST_FRAME_INDEX, CharacterAnimation, CharacterMarker};
    pub use super::brush::{Brush, BrushEntity, BrushSprite, LevelValidity};
    pub use super::fonts::Fonts;
    pub use super::images::Images;
    pub use super::input::{ActionInput, ActionInputEvent, DirectionInput, DirectionInputEvent};
    pub use super::level::*;
    pub use super::save_file::{SaveFile, SaveFileHandle};
    pub use super::scene::SceneTransitionEvent;
    pub use super::sounds::{INITIAL_VOLUME, Music, Sfx, Sounds};
    pub use super::state::{GameState, SelectionKind};
    pub use super::systems::cleanup;
}

use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_kira_audio::AudioApp;

use prelude::*;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            state::Plugin,
            level::Plugin,
            input::Plugin,
            sounds::Plugin,
            RonAssetPlugin::<SaveFile>::new(&["dat"]),
            RonAssetPlugin::<LevelState>::new(&["lvl"]),
        ))
        .add_audio_channel::<Sfx>()
        .add_audio_channel::<Music>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Title)
                .load_collection::<LevelHandles>()
                .load_collection::<Fonts>()
                .load_collection::<Images>()
                .load_collection::<Sounds>(),
        )
        .add_systems(OnEnter(GameState::Loading), SaveFileHandle::load)
        .add_systems(
            Update,
            SaveFile::insert
                .run_if(SaveFileHandle::check_loaded_or_failed)
                .run_if(in_state(GameState::Loading)),
        )
        .add_systems(
            OnExit(GameState::Loading),
            (
                camera::setup.run_if(resource_added::<SaveFile>),
                sounds::setup.run_if(resource_added::<SaveFile>),
                insert_custom_level_handles.run_if(resource_added::<SaveFile>),
            ),
        );
    }
}
