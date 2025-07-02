use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::level::internal::map::{Map, MapEntity, MapPosition};

#[derive(Asset, TypePath, Serialize, Deserialize, Default, Clone, Copy)]
pub struct LevelState {
    map: Map,
    character_position: MapPosition,
    character_facing_direction: usize,
    remaining_zones: usize,
}

impl LevelState {
    pub fn get_entity(&self, position: &MapPosition) -> &MapEntity {
        &self.map[position]
    }

    pub fn set_entity(&mut self, position: &MapPosition, entity: MapEntity) {
        self.map[position] = entity;
    }

    pub fn character_position(&self) -> MapPosition {
        self.character_position
    }

    pub fn move_character(&mut self, position: MapPosition) {
        self.character_position = position;
    }

    pub fn character_facing_direction(&self) -> usize {
        self.character_facing_direction
    }

    pub fn set_character_facing_direction(&mut self, direction: usize) {
        self.character_facing_direction = direction;
    }

    pub fn increment_remaining_zones(&mut self) {
        self.remaining_zones += 1;
    }

    pub fn decrement_remaining_zones(&mut self) {
        self.remaining_zones -= 1;
    }

    pub fn no_remaining_zones(&self) -> bool {
        self.remaining_zones == 0
    }
}
