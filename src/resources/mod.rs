mod colors;
mod fonts;
mod images;
mod sounds;

pub mod animations;
pub mod input;
pub mod level;
pub mod prelude;
pub mod save_file;

use bevy::prelude::*;
use bevy_asset_ron::RonAssetPlugin;
use bevy_kira_audio::{AudioApp, AudioChannel};

use crate::core::{self, state::GameState};

use level::prelude::*;
use save_file::SaveFile;

use prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_channel::<Sfx>()
            .add_audio_channel::<Music>()
            .add_plugin(RonAssetPlugin::<SaveFile>::new(&["dat"]))
            .add_plugin(RonAssetPlugin::<LevelState>::new(&["lvl"]))
            .add_system_set(SystemSet::on_enter(GameState::Startup).with_system(startup))
            .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_loading));
    }
}

fn startup(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
) {
    let fonts = Fonts::load(&asset_server);
    let images = Images::load(&asset_server);
    let sounds = Sounds::load(&asset_server);
    let levels = LevelHandles::load_stock(&asset_server);
    let save_file = SaveFileHandle::load(&asset_server);

    commands.insert_resource(fonts);
    commands.insert_resource(images);
    commands.insert_resource(sounds);
    commands.insert_resource(levels);
    commands.insert_resource(save_file);

    state.set(GameState::Loading).unwrap();
}

fn check_loading(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    levels: Res<LevelHandles>,
    asset_server: Res<AssetServer>,
    sfx: Res<AudioChannel<Sfx>>,
    music: Res<AudioChannel<Music>>,
    images: Res<Images>,
    fonts: Res<Fonts>,
    sounds: Res<Sounds>,
    save_file_handle: Res<SaveFileHandle>,
    save_file: Res<Assets<SaveFile>>,
) {
    let all_loaded = fonts.all_loaded(&asset_server)
        && images.all_loaded(&asset_server)
        && sounds.all_loaded(&asset_server)
        && levels.all_stock_loaded(&asset_server)
        && save_file_handle.check_loaded_or_failed(&asset_server);

    if all_loaded {
        core::save_file::insert(&mut commands, &save_file_handle, &asset_server, &save_file);

        music.set_volume(sounds.volume / 2.0);
        sfx.set_volume(sounds.volume);

        state.set(GameState::Title).unwrap();
    }
}
