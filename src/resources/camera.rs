use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, Msaa::Off));
}
