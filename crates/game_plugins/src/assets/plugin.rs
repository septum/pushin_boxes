use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_asset_loader::prelude::*;

use super::prelude::*;

use crate::{assets::sounds, level::LevelHandles, state::GameState};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(sounds::Plugin).add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Title)
                .load_collection::<LevelHandles>()
                .load_collection::<Fonts>()
                .load_collection::<Images>()
                .load_collection::<Sounds>(),
        );
    }
}
