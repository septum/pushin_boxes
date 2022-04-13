use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use ron;

use bevy::prelude::*;
use bevy::{asset::LoadState, reflect::TypeUuid};
use bevy_asset_ron::RonAssetPlugin;
use bevy_kira_audio::AudioSource;
use serde::{Deserialize, Serialize};

use crate::config::MAX_TOTAL_LEVELS;
use crate::{
    level::{LevelState, MapEntity},
    state::GameState,
};

pub struct LoadedHandles {
    pub assets: AssetsHandles,
    pub save_file: SaveFileHandle,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<SaveFileData>::new(&["dat"]))
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
    let assets = AssetsHandles::load(&asset_server);
    let save_file = SaveFileHandle::load(&asset_server);

    commands.insert_resource(LoadedHandles { assets, save_file });

    state.set(GameState::Loading).unwrap();
}

fn check_loading(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    loaded_handles: Res<LoadedHandles>,
    asset_server: Res<AssetServer>,
    loaded_save_file_data: Res<Assets<SaveFileData>>,
) {
    if loaded_handles.assets.all_loaded(&asset_server)
        && loaded_handles.save_file.check_load_state(
            &mut commands,
            &asset_server,
            &loaded_save_file_data,
        )
    {
        state.set(GameState::Title).unwrap();
    }
}

pub struct Fonts {
    pub fredoka: Handle<Font>,
}

#[derive(TypeUuid, Deserialize, Serialize, Clone)]
#[uuid = "2e5bbfc2-8dfd-4547-8c85-cbaf27533998"]
pub struct SaveFileData(pub Vec<usize>);

pub struct SaveFile {
    pub level_records: Vec<usize>,
}

impl SaveFile {
    pub fn new(level_records: Vec<usize>) -> SaveFile {
        SaveFile { level_records }
    }

    fn set_record(&mut self, level: usize, moves: usize) {
        self.level_records[level] = moves;
    }

    pub fn get_record(&self, level: usize) -> usize {
        self.level_records[level]
    }

    pub fn set_if_new_record(&mut self, level: usize, moves: usize) {
        let record = self.get_record(level);

        if record == 0 || record > moves {
            self.set_record(level, moves);
        }
    }

    pub fn unlock_next_level(&mut self, level: usize) {
        let next_level = level + 1;

        if next_level < MAX_TOTAL_LEVELS && next_level >= self.level_records.len() {
            self.level_records.push(0);
        }
    }

    pub fn final_record(&self) -> usize {
        self.level_records.iter().sum()
    }

    pub fn save(&self) {
        let data = SaveFileData(self.level_records.clone());
        let path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            PathBuf::from(manifest_dir).join("assets").join("game.dat")
        } else {
            PathBuf::from("./assets").join("game.dat")
        };

        if let Ok(serialized_string) = ron::ser::to_string(&data) {
            let mut file = File::create(path).unwrap();
            file.write_all(serialized_string.as_bytes()).unwrap();
        }
    }
}

pub struct SaveFileHandle {
    pub save_file: Handle<SaveFileData>,
}

impl SaveFileHandle {
    pub fn load(asset_server: &Res<AssetServer>) -> SaveFileHandle {
        SaveFileHandle {
            save_file: asset_server.load("game.dat"),
        }
    }

    pub fn check_load_state(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        loaded_save_file_data: &Res<Assets<SaveFileData>>,
    ) -> bool {
        match asset_server.get_load_state(self.save_file.clone()) {
            LoadState::Loaded => {
                let data = loaded_save_file_data.get(self.save_file.clone()).unwrap();
                commands.insert_resource(SaveFile::new(data.0.to_owned()));
                true
            }
            LoadState::Failed => {
                commands.insert_resource(SaveFile::new(vec![0]));
                true
            }
            _ => false,
        }
    }
}

pub struct Images {
    pub entity_box: Handle<Image>,
    pub entity_floor: Handle<Image>,
    pub entity_wall: Handle<Image>,
    pub entity_zone: Handle<Image>,
    pub player_down: Handle<Image>,
    pub player_idle: Handle<Image>,
    pub player_left: Handle<Image>,
    pub player_right: Handle<Image>,
    pub player_up: Handle<Image>,
    pub volume_max: Handle<Image>,
    pub volume_min: Handle<Image>,
    pub volume_muted: Handle<Image>,
    pub background: Handle<Image>,
    pub button: Handle<Image>,
    pub controls: Handle<Image>,
}

impl Images {
    pub fn spawn_background(&self, commands: &mut Commands, marker: impl Component) {
        commands
            .spawn_bundle(SpriteBundle {
                texture: self.background.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            })
            .insert(marker);
    }

    pub fn from_map_entity(&self, entity: &MapEntity) -> Handle<Image> {
        match entity {
            MapEntity::W => self.entity_wall.clone(),
            MapEntity::F => self.entity_floor.clone(),
            MapEntity::Z => self.entity_zone.clone(),
            MapEntity::B => self.entity_box.clone(),
            MapEntity::P => self.entity_box.clone(),
        }
    }
}

pub struct Levels {
    pub collection: Vec<Handle<LevelState>>,
}

pub struct Sounds {
    pub fx_move_player: Handle<AudioSource>,
    pub fx_push_box: Handle<AudioSource>,
    pub fx_set_zone: Handle<AudioSource>,
    pub music_level: Handle<AudioSource>,
    pub music_selection: Handle<AudioSource>,
    pub music_title: Handle<AudioSource>,
    pub music_win: Handle<AudioSource>,
}

pub struct AssetsHandles {
    pub fonts: Fonts,
    pub images: Images,
    pub levels: Levels,
    pub sounds: Sounds,
}

impl AssetsHandles {
    pub fn load(asset_server: &Res<AssetServer>) -> AssetsHandles {
        let fonts = Fonts {
            fredoka: asset_server.load("fonts/fredoka/FredokaOne-Regular.ttf"),
        };

        let images = Images {
            entity_box: asset_server.load("images/entities/box.png"),
            entity_floor: asset_server.load("images/entities/floor.png"),
            entity_wall: asset_server.load("images/entities/wall.png"),
            entity_zone: asset_server.load("images/entities/zone.png"),
            player_down: asset_server.load("images/player/down.png"),
            player_idle: asset_server.load("images/player/idle.png"),
            player_left: asset_server.load("images/player/left.png"),
            player_right: asset_server.load("images/player/right.png"),
            player_up: asset_server.load("images/player/up.png"),
            volume_max: asset_server.load("images/volume/max.png"),
            volume_min: asset_server.load("images/volume/min.png"),
            volume_muted: asset_server.load("images/volume/muted.png"),
            background: asset_server.load("images/background.png"),
            button: asset_server.load("images/button.png"),
            controls: asset_server.load("images/controls.png"),
        };

        let sounds = Sounds {
            fx_move_player: asset_server.load("sounds/fx/move_player.wav"),
            fx_push_box: asset_server.load("sounds/fx/push_box.wav"),
            fx_set_zone: asset_server.load("sounds/fx/set_zone.wav"),
            music_level: asset_server.load("sounds/music/level.wav"),
            music_selection: asset_server.load("sounds/music/selection.wav"),
            music_title: asset_server.load("sounds/music/title.wav"),
            music_win: asset_server.load("sounds/music/win.wav"),
        };

        let levels = Levels {
            collection: vec![
                asset_server.load("levels/1.lvl"),
                asset_server.load("levels/2.lvl"),
                asset_server.load("levels/3.lvl"),
                asset_server.load("levels/4.lvl"),
                asset_server.load("levels/5.lvl"),
                asset_server.load("levels/6.lvl"),
                asset_server.load("levels/7.lvl"),
                asset_server.load("levels/8.lvl"),
                asset_server.load("levels/9.lvl"),
                asset_server.load("levels/10.lvl"),
                asset_server.load("levels/11.lvl"),
                asset_server.load("levels/12.lvl"),
                asset_server.load("levels/13.lvl"),
                asset_server.load("levels/14.lvl"),
                asset_server.load("levels/15.lvl"),
                asset_server.load("levels/16.lvl"),
            ],
        };

        AssetsHandles {
            fonts,
            images,
            sounds,
            levels,
        }
    }

    pub fn all_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        for asset in self.as_vector_untyped() {
            if asset_server.get_load_state(asset) != LoadState::Loaded {
                return false;
            }
        }
        true
    }

    fn as_vector_untyped(&self) -> Vec<HandleUntyped> {
        let mut vector = vec![
            self.fonts.fredoka.clone_untyped(),
            self.images.entity_box.clone_untyped(),
            self.images.entity_floor.clone_untyped(),
            self.images.entity_wall.clone_untyped(),
            self.images.entity_zone.clone_untyped(),
            self.images.player_down.clone_untyped(),
            self.images.player_idle.clone_untyped(),
            self.images.player_left.clone_untyped(),
            self.images.player_right.clone_untyped(),
            self.images.player_up.clone_untyped(),
            self.images.volume_max.clone_untyped(),
            self.images.volume_min.clone_untyped(),
            self.images.volume_muted.clone_untyped(),
            self.images.background.clone_untyped(),
            self.images.button.clone_untyped(),
            self.images.controls.clone_untyped(),
            self.sounds.fx_move_player.clone_untyped(),
            self.sounds.fx_push_box.clone_untyped(),
            self.sounds.fx_set_zone.clone_untyped(),
            self.sounds.music_level.clone_untyped(),
            self.sounds.music_selection.clone_untyped(),
            self.sounds.music_title.clone_untyped(),
            self.sounds.music_win.clone_untyped(),
        ];
        vector.append(
            &mut self
                .levels
                .collection
                .iter()
                .map(|level| level.clone_untyped())
                .collect(),
        );
        vector
    }
}

pub struct Colors;

impl Colors {
    pub const PRIMARY: Color = Color::Rgba {
        red: 245.0 / u8::MAX as f32,
        green: 210.0 / u8::MAX as f32,
        blue: 70.0 / u8::MAX as f32,
        alpha: 1.0,
    };
    pub const SECONDARY: Color = Color::Rgba {
        red: 108.0 / u8::MAX as f32,
        green: 255.0 / u8::MAX as f32,
        blue: 91.0 / u8::MAX as f32,
        alpha: 1.0,
    };
    pub const LIGHT: Color = Color::Rgba {
        red: 227.0 / u8::MAX as f32,
        green: 227.0 / u8::MAX as f32,
        blue: 227.0 / u8::MAX as f32,
        alpha: 1.0,
    };
    pub const DARK: Color = Color::Rgba {
        red: 28.0 / u8::MAX as f32,
        green: 28.0 / u8::MAX as f32,
        blue: 28.0 / u8::MAX as f32,
        alpha: 1.0,
    };
    pub const TRANSPARENT: Color = Color::Rgba {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
        alpha: 0.0,
    };
}
