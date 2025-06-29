mod fonts;
mod images;
mod sounds;
mod systems;

pub mod prelude {
    pub use super::fonts::Fonts;
    pub use super::images::{
        BLINK_ROW_LAST_FRAME_INDEX, CharacterAnimation, CharacterMarker, Images,
    };
    pub use super::sounds::{Music, Sfx, Sounds};
    pub use super::systems::cleanup;
}

use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioApp;

use prelude::*;

use crate::{level::LevelHandles, save_file::SaveFile, state::GameState};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(sounds::Plugin)
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
            .add_systems(
                OnExit(GameState::Loading),
                (sounds::setup.run_if(resource_added::<SaveFile>),),
            );
    }
}
