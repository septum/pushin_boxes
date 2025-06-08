use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::resources::prelude::*;

use super::{MAP_COLS, MAP_ROWS};

pub const SPRITE_SIZE: usize = 64;
pub const SPRITE_OFFSET: usize = 32;

pub const ENTITY_SURFACE: usize = 36;
pub const ENTITY_SURFACE_OFFSET: usize = 18;

pub const MAP_WIDTH: f32 = 640.0;
pub const MAP_HEIGHT: f32 = 388.0;

#[derive(Component, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub struct MapPosition {
    pub x: usize,
    pub y: usize,
}

impl MapPosition {
    pub fn new(x: usize, y: usize) -> MapPosition {
        MapPosition { x, y }
    }

    pub fn equals(&self, other: &MapPosition) -> bool {
        self.x == other.x && self.y == other.y
    }

    pub fn increment_x(&mut self) {
        if self.x < MAP_COLS - 1 {
            self.x = self.x.saturating_add(1);
        }
    }

    pub fn increment_y(&mut self) {
        if self.y < MAP_ROWS - 1 {
            self.y = self.y.saturating_add(1);
        }
    }

    pub fn decrement_x(&mut self) {
        if self.x > 0 {
            self.x = self.x.saturating_sub(1);
        }
    }

    pub fn decrement_y(&mut self) {
        if self.y > 0 {
            self.y = self.y.saturating_sub(1);
        }
    }

    pub fn update_translation(&self, translation: &mut Vec3) {
        // calculate coords with the correct sprite dimension
        // and moving the origin/pivot from the center to the top-left
        let x = ((self.x * SPRITE_SIZE) + SPRITE_OFFSET) as f32;
        let y = (((MAP_ROWS - self.y) * ENTITY_SURFACE) - ENTITY_SURFACE_OFFSET) as f32;

        // take into account the camera's default position (0, 0)
        translation.x = x - (MAP_WIDTH / 2.0);
        translation.y = y - (MAP_HEIGHT / 2.0);

        // adaptation of depthness in a 2D plane
        translation.z = self.y as f32;
    }

    pub fn update_position(&mut self, direction: &DirectionInput) {
        match direction {
            DirectionInput::Up => self.decrement_y(),
            DirectionInput::Left => self.decrement_x(),
            DirectionInput::Down => self.increment_y(),
            DirectionInput::Right => self.increment_x(),
        };
    }

    pub fn spawn_entity(&self, commands: &mut Commands, texture: Handle<Image>) {
        let mut translation = Vec3::default();
        self.update_translation(&mut translation);

        let transform = Transform::from_translation(translation);
        let bundle = SpriteBundle {
            transform,
            texture,
            ..default()
        };
        commands.spawn(bundle).insert(*self);
    }

    pub fn spawn_character(
        &self,
        commands: &mut Commands,
        atlas: TextureAtlas,
        texture: Handle<Image>,
    ) {
        let mut translation = Vec3::default();
        self.update_translation(&mut translation);

        let transform = Transform::from_translation(translation);
        let sprite = SpriteBundle {
            texture,
            transform,
            ..default()
        };
        commands.spawn((sprite, atlas)).insert(CharacterMarker);
    }
}
