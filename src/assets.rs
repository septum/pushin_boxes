use std::vec;

use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::level::LevelData;

pub struct Fonts {
    pub fredoka: Handle<Font>,
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

pub struct Levels {
    pub one: Handle<LevelData>,
    pub two: Handle<LevelData>,
    pub three: Handle<LevelData>,
    pub four: Handle<LevelData>,
    pub five: Handle<LevelData>,
    pub six: Handle<LevelData>,
    pub seven: Handle<LevelData>,
    pub eight: Handle<LevelData>,
    pub nine: Handle<LevelData>,
    pub ten: Handle<LevelData>,
    pub elven: Handle<LevelData>,
    pub twelve: Handle<LevelData>,
    pub thirteen: Handle<LevelData>,
    pub fourteen: Handle<LevelData>,
    pub fifteen: Handle<LevelData>,
    pub sixteen: Handle<LevelData>,
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

pub struct GameAssets {
    pub fonts: Fonts,
    pub images: Images,
    pub levels: Levels,
    pub sounds: Sounds,
}

impl GameAssets {
    pub fn load(asset_server: Res<AssetServer>) -> GameAssets {
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
            one: asset_server.load("levels/1.lvl"),
            two: asset_server.load("levels/2.lvl"),
            three: asset_server.load("levels/3.lvl"),
            four: asset_server.load("levels/4.lvl"),
            five: asset_server.load("levels/5.lvl"),
            six: asset_server.load("levels/6.lvl"),
            seven: asset_server.load("levels/7.lvl"),
            eight: asset_server.load("levels/8.lvl"),
            nine: asset_server.load("levels/9.lvl"),
            ten: asset_server.load("levels/10.lvl"),
            elven: asset_server.load("levels/11.lvl"),
            twelve: asset_server.load("levels/12.lvl"),
            thirteen: asset_server.load("levels/13.lvl"),
            fourteen: asset_server.load("levels/14.lvl"),
            fifteen: asset_server.load("levels/15.lvl"),
            sixteen: asset_server.load("levels/16.lvl"),
        };

        GameAssets {
            fonts,
            images,
            sounds,
            levels,
        }
    }

    pub fn as_array_untyped(&self) -> Vec<HandleUntyped> {
        vec![
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
            self.levels.one.clone_untyped(),
            self.levels.two.clone_untyped(),
            self.levels.three.clone_untyped(),
            self.levels.four.clone_untyped(),
            self.levels.five.clone_untyped(),
            self.levels.six.clone_untyped(),
            self.levels.seven.clone_untyped(),
            self.levels.eight.clone_untyped(),
            self.levels.nine.clone_untyped(),
            self.levels.ten.clone_untyped(),
            self.levels.elven.clone_untyped(),
            self.levels.twelve.clone_untyped(),
            self.levels.thirteen.clone_untyped(),
            self.levels.fourteen.clone_untyped(),
            self.levels.fifteen.clone_untyped(),
            self.levels.sixteen.clone_untyped(),
            self.sounds.fx_move_player.clone_untyped(),
            self.sounds.fx_push_box.clone_untyped(),
            self.sounds.fx_set_zone.clone_untyped(),
            self.sounds.music_level.clone_untyped(),
            self.sounds.music_selection.clone_untyped(),
            self.sounds.music_title.clone_untyped(),
            self.sounds.music_win.clone_untyped(),
        ]
    }
}

pub struct Colors {
    pub primary: Color,
    pub light: Color,
    pub dark: Color,
    pub transparent: Color,
}

pub const GAME_COLORS: Colors = Colors {
    primary: Color::Rgba {
        red: 245.0 / u8::MAX as f32,
        green: 210.0 / u8::MAX as f32,
        blue: 70.0 / u8::MAX as f32,
        alpha: 1.0,
    },
    light: Color::Rgba {
        red: 227.0 / u8::MAX as f32,
        green: 227.0 / u8::MAX as f32,
        blue: 227.0 / u8::MAX as f32,
        alpha: 1.0,
    },
    dark: Color::Rgba {
        red: 28.0 / u8::MAX as f32,
        green: 28.0 / u8::MAX as f32,
        blue: 28.0 / u8::MAX as f32,
        alpha: 1.0,
    },
    transparent: Color::Rgba {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
        alpha: 0.0,
    },
};
