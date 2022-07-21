use bevy::{asset::LoadState, prelude::*};

pub struct EntitiesImages {
    pub pbox: Handle<Image>,
    pub floor: Handle<Image>,
    pub zone: Handle<Image>,
}

pub struct PlayerImages {
    pub spritesheet: Handle<Image>,
    pub pushin: Handle<Image>,
}

pub struct Images {
    pub entities: EntitiesImages,
    pub player: PlayerImages,
    pub background: Handle<Image>,
    pub instructions: Handle<Image>,
}

impl Images {
    #[must_use]
    pub fn load(asset_server: &Res<AssetServer>) -> Images {
        let entities = EntitiesImages {
            pbox: asset_server.load("images/entities/box.png"),
            floor: asset_server.load("images/entities/floor.png"),
            zone: asset_server.load("images/entities/zone.png"),
        };
        let player = PlayerImages {
            spritesheet: asset_server.load("images/player/spritesheet.png"),
            pushin: asset_server.load("images/player/pushin.png"),
        };

        Images {
            entities,
            player,
            background: asset_server.load("images/background.png"),
            instructions: asset_server.load("images/instructions.png"),
        }
    }

    #[must_use]
    pub fn all_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        let images_untyped = vec![
            self.entities.pbox.clone_untyped(),
            self.entities.floor.clone_untyped(),
            self.entities.zone.clone_untyped(),
            self.player.spritesheet.clone_untyped(),
            self.player.pushin.clone_untyped(),
            self.background.clone_untyped(),
            self.instructions.clone_untyped(),
        ];

        for image_handle in images_untyped {
            if asset_server.get_load_state(image_handle) != LoadState::Loaded {
                return false;
            }
        }

        true
    }
}
