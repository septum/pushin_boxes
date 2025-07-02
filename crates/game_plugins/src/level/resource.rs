use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use bevy::prelude::*;
use uuid::Uuid;

use crate::{
    assets::prelude::Images,
    input::DirectionInput,
    level::{
        LevelHandles, MapPositionExtension,
        done_timer::LevelDoneTimer,
        internal::{Level, LevelKind, LevelState, MapEntity},
    },
};

pub enum LevelUpdate {
    PushBox,
    PlaceBox,
    MoveCharacter,
}

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

    pub fn reload(
        &mut self,
        level_handles: &LevelHandles,
        level_states_assets: &Assets<LevelState>,
    ) -> bool {
        if self.inner.record_is_set() || !self.inner.max_undos_available() {
            let level_kind = self.inner.kind().clone();
            match level_kind {
                LevelKind::Stock(index) => {
                    self.inner.set_state(
                        *level_states_assets
                            .get(level_handles.get_stock(index))
                            .unwrap(),
                    );
                }
                LevelKind::Custom(key) => {
                    let parsed_key: Vec<&str> = key.split('$').collect();
                    let uuid = Uuid::parse_str(parsed_key[1]).expect("Cannot parse uuid");
                    self.inner.set_state(
                        *level_states_assets
                            .get(level_handles.get_custom(&uuid).unwrap())
                            .unwrap(),
                    );
                }
                LevelKind::Editable(state) => {
                    self.inner.set_state(state);
                }
            }
            true
        } else {
            false
        }
    }

    pub fn spawn(&mut self, commands: &mut Commands, images: &Images) {
        self.reset_done_timer();

        let position = self.inner.character_position();
        let level_character_facing_direction = self.inner.character_facing_direction();

        position.spawn_character(
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
                position.spawn_entity(commands, texture);
            });
    }

    pub fn update_level(&mut self, direction: &DirectionInput) -> Option<LevelUpdate> {
        match direction {
            DirectionInput::Down => self.inner.set_character_facing_direction(0),
            DirectionInput::Up => self.inner.set_character_facing_direction(1),
            DirectionInput::Left => self.inner.set_character_facing_direction(2),
            DirectionInput::Right => self.inner.set_character_facing_direction(3),
        }

        let mut next_position = self.inner.character_position();
        next_position.update_position(direction);

        let next_entity = self.inner.get_entity(&next_position);
        match next_entity {
            MapEntity::V => None,
            MapEntity::B | MapEntity::P => {
                let in_zone = matches!(next_entity, MapEntity::P);
                let updated_next_entity = if in_zone { MapEntity::Z } else { MapEntity::F };

                let mut adjacent_position = next_position;
                adjacent_position.update_position(direction);

                let adjacent_entity = self.inner.get_entity(&adjacent_position);
                match adjacent_entity {
                    MapEntity::F => {
                        self.inner.save_snapshot();
                        self.inner.set_entity(&next_position, updated_next_entity);
                        self.inner.set_entity(&adjacent_position, MapEntity::B);
                        self.inner.move_character(next_position);
                        self.inner.increment_moves();

                        if in_zone {
                            self.inner.increment_remaining_zones();
                        }

                        Some(LevelUpdate::PushBox)
                    }
                    MapEntity::Z => {
                        self.inner.save_snapshot();
                        self.inner.set_entity(&next_position, updated_next_entity);
                        self.inner.set_entity(&adjacent_position, MapEntity::P);
                        self.inner.move_character(next_position);
                        self.inner.increment_moves();

                        if !in_zone {
                            self.inner.decrement_remaining_zones();
                        }

                        Some(LevelUpdate::PlaceBox)
                    }
                    _ => None,
                }
            }
            _ => {
                self.inner.save_snapshot();
                self.inner.move_character(next_position);
                self.inner.increment_moves();

                Some(LevelUpdate::MoveCharacter)
            }
        }
    }

    pub fn tick(&mut self, delta: Duration) {
        if self.inner.no_remaining_zones() {
            self.done_timer.tick(delta);
        } else {
            self.inner.tick_stopwatch(delta);
        }
    }

    pub fn reset_done_timer(&mut self) {
        self.done_timer.reset();
    }

    pub fn finished(&self) -> bool {
        self.done_timer.just_finished()
    }
}
