use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;

pub struct Counters {
    pub moves: usize,
    pub undos: usize,
}

pub struct Level {
    pub number: usize,
    pub record: usize,
    pub data_handle: Handle<LevelData>,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub enum Entity {
    Wall,
    Floor,
    Zone,
    PushBox(bool),
}

type LevelMap = [[Entity; 10]; 10];

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub enum Input {
    Up,
    Left,
    Down,
    Right,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct PlayerPosition {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone, TypeUuid)]
#[uuid = "d1e78377-22a5-49f7-a675-60d348abc837"]
pub struct LevelData {
    pub map: LevelMap,
    pub zones: usize,
    pub pushing_input: Option<Input>,
    pub player_position: PlayerPosition,
}
