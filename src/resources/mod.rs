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
        app.add_plugin(state::Plugin)
            .add_plugin(level::Plugin)
            .add_plugin(input::Plugin)
            .add_plugin(sounds::Plugin)
            .add_plugin(RonAssetPlugin::<SaveFile>::new(&["dat"]))
            .add_plugin(RonAssetPlugin::<LevelState>::new(&["lvl"]))
            .add_audio_channel::<Sfx>()
            .add_audio_channel::<Music>()
            .add_loading_state(
                LoadingState::new(GameState::Loading).continue_to_state(GameState::Title),
            )
            .add_collection_to_loading_state::<_, LevelHandles>(GameState::Loading)
            .add_collection_to_loading_state::<_, Fonts>(GameState::Loading)
            .add_collection_to_loading_state::<_, Images>(GameState::Loading)
            .add_collection_to_loading_state::<_, Sounds>(GameState::Loading)
            .add_system(SaveFileHandle::load.in_schedule(OnEnter(GameState::Loading)))
            .add_system(
                SaveFile::insert
                    .in_set(OnUpdate(GameState::Loading))
                    .run_if(SaveFileHandle::check_loaded_or_failed),
            )
            .add_systems(
                (
                    camera::setup.run_if(resource_added::<SaveFile>()),
                    sounds::setup.run_if(resource_added::<SaveFile>()),
                    level::insert_custom_level_handles.run_if(resource_added::<SaveFile>()),
                )
                    .in_schedule(OnExit(GameState::Loading)),
            );
    }
}
