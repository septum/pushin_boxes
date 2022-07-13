use bevy::prelude::*;

pub struct PlayerAnimation {
    pub timer: Timer,
    pub initial_index: usize,
    pub index: usize,
}
