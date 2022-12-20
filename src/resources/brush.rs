use bevy::prelude::*;

use super::prelude::*;

#[derive(Default)]
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
            blink_timer: Timer::from_seconds(0.1, true),
        }
    }
}

impl Brush {
    pub fn insert(mut commands: Commands, images: Res<Images>) {
        let brush = Brush::default();

        let mut translation = Vec3::default();
        brush.position.update_translation(&mut translation);

        translation.y += 20.0;
        translation.z = 20.0;

        let transform = Transform::from_translation(translation);
        let bundle = SpriteBundle {
            transform,
            texture: images.brush_floor.clone(),
            ..default()
        };

        commands.insert_resource(brush);
        commands.spawn_bundle(bundle).insert(BrushSprite);
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
