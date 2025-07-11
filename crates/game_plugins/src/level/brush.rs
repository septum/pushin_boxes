use bevy::prelude::*;

use game_core::map::MapPosition;

use crate::{assets::prelude::Images, level::apply_position_to_translation};

#[derive(Default, Resource)]
pub struct LevelValidity {
    pub zones: usize,
    pub boxes: usize,
}

impl LevelValidity {
    pub fn reset(&mut self) {
        self.zones = 0;
        self.boxes = 0;
    }
}

#[derive(Default)]
pub enum BrushEntity {
    #[default]
    Floor,
    Void,
    Zone,
    BoxInFloor,
    BoxInZone,
    Character,
}

#[derive(Resource)]
pub struct Brush {
    pub entity: BrushEntity,
    pub position: MapPosition,
    pub blink_timer: Timer,
}

#[derive(Component)]
pub struct BrushSprite;

impl Default for Brush {
    fn default() -> Self {
        Self {
            entity: BrushEntity::default(),
            position: MapPosition::default(),
            blink_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

impl Brush {
    pub fn insert(mut commands: Commands, images: Res<Images>) {
        let brush = Brush::default();

        let mut translation = Vec3::default();
        apply_position_to_translation(&brush.position, &mut translation);

        translation.y += 20.0;
        translation.z = 20.0;

        let transform = Transform::from_translation(translation);
        let sprite = Sprite {
            image: images.brush_floor.clone(),
            ..default()
        };

        commands.insert_resource(brush);
        commands.spawn((sprite, transform)).insert(BrushSprite);
    }

    pub fn cycle(&mut self) {
        self.entity = match self.entity {
            BrushEntity::Floor => BrushEntity::Void,
            BrushEntity::Void => BrushEntity::Zone,
            BrushEntity::Zone => BrushEntity::BoxInFloor,
            BrushEntity::BoxInFloor => BrushEntity::BoxInZone,
            BrushEntity::BoxInZone => BrushEntity::Character,
            BrushEntity::Character => BrushEntity::Floor,
        };
    }
}
