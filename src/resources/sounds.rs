use bevy::{asset::LoadState, prelude::*};
use bevy_kira_audio::{AudioChannel, AudioSource};

pub struct Channels {
    pub sfx: AudioChannel,
    pub music: AudioChannel,
}

pub struct Sfx {
    pub move_player: Handle<AudioSource>,
    pub push_box: Handle<AudioSource>,
    pub set_zone: Handle<AudioSource>,
}

pub struct Music {
    pub level: Handle<AudioSource>,
    pub selection: Handle<AudioSource>,
    pub title: Handle<AudioSource>,
    pub win: Handle<AudioSource>,
}

pub struct Sounds {
    pub channels: Channels,
    pub sfx: Sfx,
    pub music: Music,
}

impl Sounds {
    pub fn load(asset_server: &Res<AssetServer>) -> Sounds {
        let channels = Channels {
            sfx: AudioChannel::new("sfx".to_string()),
            music: AudioChannel::new("music".to_string()),
        };
        let sfx = Sfx {
            move_player: asset_server.load("sounds/fx/move_player.wav"),
            push_box: asset_server.load("sounds/fx/push_box.wav"),
            set_zone: asset_server.load("sounds/fx/set_zone.wav"),
        };
        let music = Music {
            level: asset_server.load("sounds/music/level.wav"),
            selection: asset_server.load("sounds/music/selection.wav"),
            title: asset_server.load("sounds/music/title.wav"),
            win: asset_server.load("sounds/music/win.wav"),
        };

        Sounds {
            channels,
            sfx,
            music,
        }
    }

    pub fn all_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        let sounds_untyped = vec![
            self.sfx.move_player.clone_untyped(),
            self.sfx.push_box.clone_untyped(),
            self.sfx.set_zone.clone_untyped(),
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
