use bevy::{asset::LoadState, prelude::*};

pub struct EntitiesImages {
    pub pbox: Handle<Image>,
    pub floor: Handle<Image>,
    pub wall: Handle<Image>,
    pub zone: Handle<Image>,
}

pub struct PlayerImages {
    pub up: Handle<Image>,
    pub down: Handle<Image>,
    pub idle: Handle<Image>,
    pub left: Handle<Image>,
    pub right: Handle<Image>,
}

pub struct VolumeImages {
    pub max: Handle<Image>,
    pub min: Handle<Image>,
    pub muted: Handle<Image>,
}

pub struct Images {
    pub entities: EntitiesImages,
    pub player: PlayerImages,
    pub volume: VolumeImages,
    pub background: Handle<Image>,
    pub button: Handle<Image>,
    pub controls: Handle<Image>,
}

impl Images {
    #[must_use] pub fn load(asset_server: &Res<AssetServer>) -> Images {
        let entities = EntitiesImages {
            pbox: asset_server.load("images/entities/box.png"),
            floor: asset_server.load("images/entities/floor.png"),
            wall: asset_server.load("images/entities/wall.png"),
            zone: asset_server.load("images/entities/zone.png"),
        };
        let player = PlayerImages {
            up: asset_server.load("images/player/up.png"),
            down: asset_server.load("images/player/down.png"),
            idle: asset_server.load("images/player/idle.png"),
            left: asset_server.load("images/player/left.png"),
            right: asset_server.load("images/player/right.png"),
        };
        let volume = VolumeImages {
            max: asset_server.load("images/volume/max.png"),
            min: asset_server.load("images/volume/min.png"),
            muted: asset_server.load("images/volume/muted.png"),
        };

        Images {
            entities,
            player,
            volume,
            background: asset_server.load("images/background.png"),
            button: asset_server.load("images/button.png"),
            controls: asset_server.load("images/controls.png"),
        }
    }

    #[must_use] pub fn all_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        let images_untyped = vec![
            self.entities.pbox.clone_untyped(),
            self.entities.floor.clone_untyped(),
            self.entities.wall.clone_untyped(),
            self.entities.zone.clone_untyped(),
            self.player.up.clone_untyped(),
            self.player.down.clone_untyped(),
            self.player.idle.clone_untyped(),
            self.player.left.clone_untyped(),
            self.player.right.clone_untyped(),
            self.volume.max.clone_untyped(),
            self.volume.min.clone_untyped(),
            self.volume.muted.clone_untyped(),
            self.background.clone_untyped(),
            self.button.clone_untyped(),
            self.controls.clone_untyped(),
        ];

        for image_handle in images_untyped {
            if asset_server.get_load_state(image_handle) != LoadState::Loaded {
                return false;
            }
        }

        true
    }
}
