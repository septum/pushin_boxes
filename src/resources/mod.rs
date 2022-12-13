mod animations;
mod brush;
mod camera;
mod colors;
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
    pub use super::animations::{CharacterAnimation, CharacterMarker, BLINK_ROW_LAST_FRAME_INDEX};
    pub use super::brush::{Brush, BrushEntity, BrushSprite, LevelValidity};
    pub use super::colors::Colors;
    pub use super::fonts::Fonts;
    pub use super::images::Images;
    pub use super::input::{ActionInput, ActionInputEvent, DirectionInput, DirectionInputEvent};
    pub use super::level::prelude::*;
    pub use super::save_file::{SaveFile, SaveFileHandle};
    pub use super::scene::SceneTransitionEvent;
    pub use super::sounds::{Music, Sfx, Sounds, INITIAL_VOLUME};
    pub use super::state::GameState;
    pub use super::systems::cleanup;
}

use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_kira_audio::AudioApp;
use iyes_loopless::prelude::*;

use prelude::*;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(state::Plugin)
            .add_plugin(level::Plugin)
            .add_plugin(input::Plugin)
            .add_plugin(sounds::Plugin)
            .add_plugin(RonAssetPlugin::<SaveFile>::new(&["dat"]))
            .add_plugin(RonAssetPlugin::<LevelState>::new(&["lvl"]))
            .add_audio_channel::<Sfx>()
            .add_audio_channel::<Music>()
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Title)
                    .with_collection::<LevelHandles>()
                    .with_collection::<Fonts>()
                    .with_collection::<Images>()
                    .with_collection::<Sounds>(),
            )
            .add_enter_system(GameState::Loading, SaveFileHandle::load)
            .add_system(
                SaveFile::insert
                    .run_in_state(GameState::Loading)
                    .run_if(SaveFileHandle::check_loaded_or_failed),
            )
            .add_exit_system_set(
                GameState::Loading,
                ConditionSet::new()
                    .run_if_resource_added::<SaveFile>()
                    .with_system(camera::setup)
                    .with_system(sounds::setup)
                    .with_system(level::insert_custom_level_handles)
                    .into(),
            );
    }
}
