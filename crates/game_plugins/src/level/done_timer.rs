use bevy::{
    prelude::{Deref, DerefMut},
    time::{Timer, TimerMode},
};

#[derive(Deref, DerefMut)]
pub struct LevelDoneTimer(Timer);

impl Default for LevelDoneTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.25, TimerMode::Once))
    }
}
