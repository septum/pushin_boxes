use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use bevy::prelude::*;
use game_core::{
    level::{Level, LevelKind, LevelState},
    map::MapEntity,
};

use crate::{
    assets::prelude::Images,
    character::Character,
    level::{EntityComponent, done_timer::LevelDoneTimer},
};

pub const TOTAL_STOCK_LEVELS: usize = 16;
pub const TOTAL_CUSTOM_LEVELS: usize = 16;

#[derive(Resource, Default)]
pub struct LevelResource {
    inner: Level,
    done_timer: LevelDoneTimer,
}

impl Deref for LevelResource {
    type Target = Level;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for LevelResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl LevelResource {
    pub fn new(kind: LevelKind, state: LevelState) -> LevelResource {
        LevelResource {
            inner: Level::new(kind, state),
            ..LevelResource::default()
        }
    }

    pub fn spawn(&mut self, commands: &mut Commands, images: &Images) {
        self.reset_done_timer();

        let position = self.inner.character_position();
        let level_character_facing_direction = self.inner.character_facing_direction();

        Character::spawn(
            position,
            commands,
            TextureAtlas {
                layout: images.character_layout.clone(),
                index: level_character_facing_direction,
            },
            images.character.clone(),
        );

        self.inner
            .loop_over_entity_and_position(|entity, position| {
                let texture = match entity {
                    MapEntity::V => images.entity_void.clone(),
                    MapEntity::F => images.entity_floor.clone(),
                    MapEntity::Z => images.entity_zone.clone(),
                    MapEntity::B => images.entity_box.clone(),
                    MapEntity::P => images.entity_placed_box.clone(),
                };
                EntityComponent::spawn(position, commands, texture);
            });
    }

    pub fn tick(&mut self, delta: Duration) {
        if self.inner.no_remaining_zones() {
            self.done_timer.tick(delta);
        } else {
            self.inner.tick_record(delta);
        }
    }

    pub fn reset_done_timer(&mut self) {
        self.done_timer.reset();
    }

    pub fn finished(&self) -> bool {
        self.done_timer.just_finished()
    }

    pub fn is_last(&self) -> bool {
        match self.inner.kind() {
            LevelKind::Stock(index) => *index == TOTAL_STOCK_LEVELS - 1,
            _ => false,
        }
    }
}
