use bevy::{asset::LoadState, prelude::*};
use bevy_kira_audio::AudioSource;

pub struct Sfx {
    pub move_player: Handle<AudioSource>,
    pub push_box: Handle<AudioSource>,
    pub set_zone: Handle<AudioSource>,
    pub toggle_volume: Handle<AudioSource>,
    pub undo_move: Handle<AudioSource>,
    pub reload_level: Handle<AudioSource>,
}

pub struct Music {
    pub level: Handle<AudioSource>,
    pub selection: Handle<AudioSource>,
    pub title: Handle<AudioSource>,
    pub win: Handle<AudioSource>,
}

pub struct Sounds {
    pub volume: f32,
    pub sfx: Sfx,
    pub music: Music,
}

impl Sounds {
    #[must_use]
    pub fn load(asset_server: &Res<AssetServer>) -> Sounds {
        let sfx = Sfx {
            move_player: asset_server.load("sounds/sfx/move_player.wav"),
            push_box: asset_server.load("sounds/sfx/push_box.wav"),
            set_zone: asset_server.load("sounds/sfx/set_zone.wav"),
            toggle_volume: asset_server.load("sounds/sfx/toggle_volume.wav"),
            undo_move: asset_server.load("sounds/sfx/undo_move.wav"),
            reload_level: asset_server.load("sounds/sfx/reload_level.wav"),
        };
        let music = Music {
            level: asset_server.load("sounds/music/level.wav"),
            selection: asset_server.load("sounds/music/selection.wav"),
            title: asset_server.load("sounds/music/title.wav"),
            win: asset_server.load("sounds/music/win.wav"),
        };

        Sounds {
            volume: 0.75,
            sfx,
            music,
        }
    }

    #[must_use]
    pub fn all_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        let sounds_untyped = vec![
            self.sfx.move_player.clone_untyped(),
            self.sfx.push_box.clone_untyped(),
            self.sfx.set_zone.clone_untyped(),
            self.sfx.toggle_volume.clone_untyped(),
            self.sfx.undo_move.clone_untyped(),
            self.sfx.reload_level.clone_untyped(),
            self.music.level.clone_untyped(),
            self.music.selection.clone_untyped(),
            self.music.title.clone_untyped(),
            self.music.win.clone_untyped(),
        ];

        for sound_handle in sounds_untyped {
            if asset_server.get_load_state(sound_handle) != LoadState::Loaded {
                return false;
            }
        }

        true
    }
}
