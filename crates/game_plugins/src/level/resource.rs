use bevy::prelude::*;
use uuid::Uuid;

use crate::{
    assets::prelude::Images,
    input::DirectionInput,
    level::{
        LevelHandles, MapPosition, MapPositionExtension,
        internal::{Level, LevelKind, LevelRecord, LevelState, MapEntity},
    },
};

#[derive(Resource, Deref, DerefMut)]
pub struct LevelResource(Level);

impl LevelResource {
    pub fn new(kind: LevelKind, state: LevelState, record: LevelRecord) -> LevelResource {
        LevelResource(Level::new(kind, state, record))
    }

    pub fn editable() -> LevelResource {
        let state = LevelState {
            character_position: MapPosition::new(4, 4),
            ..Default::default()
        };
        let kind = LevelKind::Editable(state);
        let record = LevelRecord::default();
        LevelResource::new(kind, state, record)
    }

    pub fn set_character_facing_direction_with(&mut self, direction: &DirectionInput) {
        match direction {
            DirectionInput::Down => self.set_character_facing_direction(0),
            DirectionInput::Up => self.set_character_facing_direction(1),
            DirectionInput::Left => self.set_character_facing_direction(2),
            DirectionInput::Right => self.set_character_facing_direction(3),
        }
    }

    pub fn reload(
        &mut self,
        level_handles: &LevelHandles,
        level_states_assets: &Assets<LevelState>,
    ) -> bool {
        if self.get_moves() != 0 || self.get_undos() < 4 {
            let level_kind = self.kind().clone();
            match level_kind {
                LevelKind::Stock(index) => {
                    self.set_state(
                        *level_states_assets
                            .get(level_handles.get_stock(index))
                            .unwrap(),
                    );
                }
                LevelKind::Custom(key) => {
                    let parsed_key: Vec<&str> = key.split('$').collect();
                    let uuid = Uuid::parse_str(parsed_key[1]).expect("Cannot parse uuid");
                    self.set_state(
                        *level_states_assets
                            .get(level_handles.get_custom(&uuid).unwrap())
                            .unwrap(),
                    );
                }
                LevelKind::Editable(state) => {
                    self.set_state(state);
                }
            }
            true
        } else {
            false
        }
    }

    pub fn spawn(&mut self, commands: &mut Commands, images: &Images) {
        let position = self.get_character_position();
        let level_character_facing_direction = self.get_character_facing_direction();

        position.spawn_character(
            commands,
            TextureAtlas {
                layout: images.character_layout.clone(),
                index: level_character_facing_direction,
            },
            images.character.clone(),
        );

        self.loop_over_entity_and_position(|entity, position| {
            let texture = match entity {
                MapEntity::V => images.entity_void.clone(),
                MapEntity::F => images.entity_floor.clone(),
                MapEntity::Z => images.entity_zone.clone(),
                MapEntity::B => images.entity_box.clone(),
                MapEntity::P => images.entity_placed_box.clone(),
            };
            position.spawn_entity(commands, texture);
        });
    }
}
