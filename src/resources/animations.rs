use bevy::prelude::*;

pub struct PlayerAnimation {
    pub timer: Timer,
    pub idle_timer: Timer,
    pub long_idle_timer: Timer,
    pub initial_index: usize,
    pub index: usize,
}
