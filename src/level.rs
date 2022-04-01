use bevy::reflect::TypeUuid;
use serde::Deserialize;

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
struct PlayerPosition {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone, TypeUuid)]
#[uuid = "d1e78377-22a5-49f7-a675-60d348abc837"]
pub struct LevelData {
    map: LevelMap,
    zones: usize,
    pushing_input: Option<Input>,
    player_position: PlayerPosition,
}
