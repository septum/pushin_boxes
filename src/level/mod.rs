use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;

use crate::{
    assets::{Images, Levels},
    config::{GAME_HEIGHT, GAME_WIDTH, MAP_COLS, MAP_ROWS, SPRITE_OFFSET, SPRITE_SIZE},
    input::DirectionKind,
};

pub const MAX_LEVEL_STATES: usize = 4;

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum MapEntity {
    /// Wall
    W,
    /// Floor
    F,
    /// Zone
    Z,
    /// Box
    B,
    /// Box in Zone
    P,
}

type LevelMap = [[MapEntity; MAP_COLS]; MAP_ROWS];

#[derive(Component, Deserialize, Clone, Copy)]
pub struct MapPosition {
    pub x: usize,
    pub y: usize,
}

impl MapPosition {
    pub fn new(x: usize, y: usize) -> MapPosition {
        MapPosition { x, y }
    }

    fn increment_x(&mut self) {
        if self.x < MAP_COLS - 1 {
            self.x = self.x.saturating_add(1);
        }
    }

    fn increment_y(&mut self) {
        if self.y < MAP_ROWS - 1 {
            self.y = self.y.saturating_add(1);
        }
    }

    fn decrement_x(&mut self) {
        self.x = self.x.saturating_sub(1);
    }

    fn decrement_y(&mut self) {
        self.y = self.y.saturating_sub(1);
    }

    pub fn advance(&mut self, direction: &DirectionKind) {
        match direction {
            DirectionKind::Up => self.decrement_y(),
            DirectionKind::Left => self.decrement_x(),
            DirectionKind::Down => self.increment_y(),
            DirectionKind::Right => self.increment_x(),
        };
    }

    pub fn apply_to_transform(&self, transform: &mut Transform) {
        // calculate coords with the correct sprite dimension
        // and moving the origin/pivot from the center to the top-left
        let x = ((self.x * SPRITE_SIZE) + SPRITE_OFFSET) as f32;
        let y = (((MAP_ROWS - self.y) * SPRITE_SIZE) - SPRITE_OFFSET) as f32;

        // take into account the camera default position (0, 0)
        transform.translation.x = x - (GAME_WIDTH / 2.0);
        transform.translation.y = y - (GAME_HEIGHT / 2.0);
    }
}

#[derive(TypeUuid, Deserialize, Clone, Copy)]
#[uuid = "d1e78377-22a5-49f7-a675-60d348abc837"]
pub struct LevelState {
    pub map: LevelMap,
    pub remaining_zones: usize,
    pub player_position: MapPosition,
}

type LevelSnapshots = [Option<LevelState>; MAX_LEVEL_STATES];

pub struct Level {
    pub index: usize,
    pub snapshots: LevelSnapshots,
    pub state: LevelState,
    pub record: usize,
    pub undos: usize,
    pub moves: usize,
}

impl Level {
    pub fn load(
        index: usize,
        record: usize,
        loaded_levels: &Res<Assets<LevelState>>,
        level_handles: &Levels,
    ) -> Level {
        let undos = 4 - (index / 4);
        let loaded_state = loaded_levels
            .get(level_handles.collection[index].clone())
            .unwrap();

        Level {
            index,
            snapshots: [None; MAX_LEVEL_STATES],
            state: *loaded_state,
            record,
            undos,
            moves: 0,
        }
    }

    pub fn reload(&mut self, loaded_levels: &Res<Assets<LevelState>>, level_handles: &Levels) {
        let undos = 4 - (self.index / 4);
        let loaded_state = loaded_levels
            .get(level_handles.collection[self.index].clone())
            .unwrap();

        self.snapshots = [None; MAX_LEVEL_STATES];
        self.state = *loaded_state;
        self.undos = undos;
        self.moves = 0;
    }

    pub fn spawn_map(&self, commands: &mut Commands, images: &Images) {
        for column in 0..MAP_COLS {
            for row in 0..MAP_ROWS {
                let mut transform = Transform::from_xyz(0.0, 0.0, 1.0);
                let position = MapPosition::new(column, row);
                let texture = images.from_map_entity(self.get_entity(&position));

                position.apply_to_transform(&mut transform);
                commands
                    .spawn_bundle(SpriteBundle {
                        texture,
                        transform,
                        ..Default::default()
                    })
                    .insert(position);
            }
        }
    }

    pub fn get_entity(&self, position: &MapPosition) -> &MapEntity {
        &self.state.map[position.y][position.x]
    }

    pub fn set_entity(&mut self, position: &MapPosition, entity: MapEntity) {
        self.state.map[position.y][position.x] = entity;
    }

    pub fn move_player(&mut self, position: MapPosition) {
        self.state.player_position = position;
        self.moves += 1; // is this too hidden?
    }

    pub fn restore_snapshot(&mut self) {
        if self.undos > 0 {
            if let Some(level_state) = self.snapshots[0] {
                self.state = level_state;
                self.snapshots.rotate_left(1);
                self.snapshots[MAX_LEVEL_STATES - 1] = None;
                self.undos -= 1;
                self.moves -= 1;
            }
        }
    }

    fn save_snapshot(&mut self) {
        self.snapshots.rotate_right(1);
        self.snapshots[0] = Some(self.state);
    }

    pub fn handle_direction_input(&mut self, direction: &DirectionKind) {
        self.save_snapshot();

        let mut next_position = self.state.player_position;
        next_position.advance(direction);

        let next_entity = self.get_entity(&next_position);
        match next_entity {
            MapEntity::B | MapEntity::P => {
                let in_zone = matches!(next_entity, MapEntity::P);
                let updated_next_entity = if in_zone { MapEntity::Z } else { MapEntity::F };

                let mut adjacent_position = next_position;
                adjacent_position.advance(direction);

                let adjacent_entity = self.get_entity(&adjacent_position);
                match adjacent_entity {
                    MapEntity::F => {
                        self.set_entity(&next_position, updated_next_entity);
                        self.set_entity(&adjacent_position, MapEntity::B);
                        self.move_player(next_position);

                        if in_zone {
                            self.state.remaining_zones += 1;
                        }
                    }
                    MapEntity::Z => {
                        self.set_entity(&next_position, updated_next_entity);
                        self.set_entity(&adjacent_position, MapEntity::P);
                        self.move_player(next_position);

                        if !in_zone {
                            self.state.remaining_zones -= 1;
                        }
                    }
                    _ => (),
                }
            }
            MapEntity::W => {}
            _ => {
                self.move_player(next_position);
            }
        }
    }

    pub fn update_entity_texture(
        &self,
        position: &MapPosition,
        handle: &mut Handle<Image>,
        images: &Images,
    ) {
        match &self.state.map[position.y][position.x] {
            MapEntity::W => *handle = images.entity_wall.clone(),
            MapEntity::F => *handle = images.entity_floor.clone(),
            MapEntity::Z => *handle = images.entity_zone.clone(),
            MapEntity::B => *handle = images.entity_box.clone(),
            MapEntity::P => *handle = images.entity_box.clone(),
        };
    }
}
