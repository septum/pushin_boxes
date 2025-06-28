mod animations;
mod camera;
mod fonts;
mod images;
mod input;
mod save_file;
mod scene;
mod sounds;
mod state;
mod systems;

pub mod prelude {
    pub use super::animations::{BLINK_ROW_LAST_FRAME_INDEX, CharacterAnimation, CharacterMarker};
    pub use super::fonts::Fonts;
    pub use super::images::Images;
    pub use super::input::{ActionInput, ActionInputEvent, DirectionInput, DirectionInputEvent};
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
use uuid::Uuid;

use crate::level::{self, LevelHandles};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            state::Plugin,
            level::Plugin,
            input::Plugin,
            sounds::Plugin,
            RonAssetPlugin::<SaveFile>::new(&["dat"]),
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

fn insert_custom_level_handles(
    save_file: Res<SaveFile>,
    mut level_handles: ResMut<LevelHandles>,
    asset_server: Res<AssetServer>,
) {
    for (_, (key, _)) in save_file.ordered_custom_records() {
        let split_key: Vec<&str> = key.split('$').collect();
        let uuid = Uuid::parse_str(split_key[1]).expect("Cannot parse uuid");
        let path = format!("levels/custom/{}.lvl", &split_key[1]);
        level_handles.insert_custom(uuid, asset_server.load(path));
    }
}
