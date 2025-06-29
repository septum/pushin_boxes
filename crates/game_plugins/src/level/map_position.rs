use bevy::prelude::*;

use crate::{
    assets::prelude::CharacterMarker,
    input::DirectionInput,
    level::internal::{MAP_ROWS, MapPosition},
};

const SPRITE_SIZE: usize = 64;
const SPRITE_OFFSET: usize = 32;

const ENTITY_SURFACE: usize = 36;
const ENTITY_SURFACE_OFFSET: usize = 18;

const MAP_WIDTH: f32 = 640.0;
const MAP_HEIGHT: f32 = 388.0;

#[derive(Component, Deref, DerefMut, PartialEq, Clone, Copy, Default)]
pub struct MapPositionComponent(MapPosition);

pub trait MapPositionExtension {
    fn update_translation(&self, translation: &mut Vec3);

    fn update_position(&mut self, direction: &DirectionInput);

    fn spawn_entity(&self, commands: &mut Commands, image: Handle<Image>);

    fn spawn_character(&self, commands: &mut Commands, atlas: TextureAtlas, image: Handle<Image>);
}

impl MapPositionExtension for MapPosition {
    fn update_translation(&self, translation: &mut Vec3) {
        // calculate coords with the correct sprite dimension
        // and moving the origin/pivot from the center to the top-left
        let x = ((self.x() * SPRITE_SIZE) + SPRITE_OFFSET) as f32;
        let y = (((MAP_ROWS - self.y()) * ENTITY_SURFACE) - ENTITY_SURFACE_OFFSET) as f32;

        // take into account the camera's default position (0, 0)
        translation.x = x - (MAP_WIDTH / 2.0);
        translation.y = y - (MAP_HEIGHT / 2.0);

        // adaptation of depthness in a 2D plane
        translation.z = self.y() as f32;
    }

    fn update_position(&mut self, direction: &DirectionInput) {
        match direction {
            DirectionInput::Up => self.decrement_y(),
            DirectionInput::Left => self.decrement_x(),
            DirectionInput::Down => self.increment_y(),
            DirectionInput::Right => self.increment_x(),
        }
    }

    fn spawn_entity(&self, commands: &mut Commands, image: Handle<Image>) {
        let mut translation = Vec3::default();
        self.update_translation(&mut translation);

        let transform = Transform::from_translation(translation);
        let sprite = Sprite { image, ..default() };
        commands
            .spawn((sprite, transform))
            .insert(MapPositionComponent(*self));
    }

    fn spawn_character(&self, commands: &mut Commands, atlas: TextureAtlas, image: Handle<Image>) {
        let mut translation = Vec3::default();
        self.update_translation(&mut translation);

        // TODO: There should be another way to do this proper
        translation.z += 1.;

        let transform = Transform::from_translation(translation);
        let sprite = Sprite {
            image,
            texture_atlas: Some(atlas),
            ..default()
        };
        commands.spawn((sprite, transform)).insert(CharacterMarker);
    }
}
