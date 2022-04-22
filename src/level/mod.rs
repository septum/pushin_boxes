mod map;
mod player;
mod snapshots;
mod state;
mod tag;

use bevy::prelude::*;

use crate::{
    config::{MAP_COLS, MAP_ROWS},
    input::DirectionKind,
    resources::{Images, Levels, SaveFile},
};

pub use map::{LevelMap, MapEntity, MapPosition};
pub use player::PlayerMarker;
pub use snapshots::LevelSnapshots;
pub use state::LevelState;
pub use tag::LevelTag;

pub const MAX_LEVEL_STATES: usize = 4;

pub struct Level {
    pub tag: LevelTag,
    pub snapshots: LevelSnapshots,
    pub state: LevelState,
    pub record: usize,
    pub undos: usize,
    pub moves: usize,
}

impl Default for Level {
    fn default() -> Level {
        Level::new()
    }
}

impl Level {
    pub fn new() -> Level {
        let state = LevelState {
            map: [[MapEntity::F; MAP_COLS]; MAP_ROWS],
            remaining_zones: 0,
            player_position: MapPosition::new(0, 0),
        };

        Level {
            tag: LevelTag::Test(state.clone()),
            snapshots: [None; MAX_LEVEL_STATES],
            state,
            record: 0,
            undos: 0,
            moves: 0,
        }
    }

    pub fn load(
        tag: LevelTag,
        save_file: &Res<SaveFile>,
        level_states: &Res<Assets<LevelState>>,
        levels: &Levels,
    ) -> Level {
        let record = save_file.get_record(&tag);
        let state = match tag {
            LevelTag::Stock(index) => {
                let handle = levels.stock[index].clone();
                *level_states.get(handle).unwrap()
            }
            LevelTag::Custom(uuid) => {
                let handle = levels.custom.get(&uuid).unwrap().clone();
                *level_states.get(handle).unwrap()
            }
            LevelTag::Test(state) => state,
        };

        Level {
            tag,
            snapshots: [None; MAX_LEVEL_STATES],
            state,
            record,
            undos: 4,
            moves: 0,
        }
    }

    pub fn no_zones_left(&self) -> bool {
        self.state.remaining_zones == 0
    }

    pub fn reload(&mut self, level_states: &Res<Assets<LevelState>>, levels: &Levels) {
        let state = match &self.tag {
            LevelTag::Stock(index) => {
                let handle = levels.stock[*index].clone();
                level_states.get(handle).unwrap().clone()
            }
            LevelTag::Custom(uuid) => {
                let handle = levels.custom.get(&uuid).unwrap().clone();
                level_states.get(handle).unwrap().clone()
            }
            LevelTag::Test(state) => state.clone(),
        };

        self.snapshots = [None; MAX_LEVEL_STATES];
        self.state = state;
        self.undos = 4;
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

    pub fn spawn_player(&self, commands: &mut Commands, images: &Images, marker: impl Component) {
        let mut transform = Transform::from_xyz(0.0, 0.0, 2.0);
        self.state
            .player_position
            .apply_to_transform(&mut transform);

        commands
            .spawn_bundle(SpriteBundle {
                texture: images.player.idle.clone(),
                transform,
                ..Default::default()
            })
            .insert(PlayerMarker)
            .insert(marker);
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

    pub fn update_player_position(&self, transform: &mut Transform) {
        self.state.player_position.apply_to_transform(transform);
    }

    pub fn player_in(&mut self, position: &MapPosition) -> bool {
        self.state.player_position.x == position.x && self.state.player_position.y == position.y
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

    pub fn increment_remaining_zones(&mut self) {
        self.state.remaining_zones += 1;
    }

    pub fn decrement_remaining_zones(&mut self) {
        self.state.remaining_zones -= 1;
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
        let entity = self.state.map[position.y][position.x];
        *handle = images.from_map_entity(&entity);
    }
}
