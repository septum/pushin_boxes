use bevy::prelude::*;

use game_core::input::Input;

#[derive(Event, Deref, DerefMut)]
pub struct InputEvent(Input);

impl From<Input> for InputEvent {
    fn from(value: Input) -> Self {
        InputEvent(value)
    }
}
