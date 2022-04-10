use bevy::prelude::*;

use crate::{assets::Images, level::MapPosition};

#[derive(Component)]
pub struct PlayerMarker;

pub struct Player {
    pub starting_position: MapPosition,
}

impl Player {
    pub fn new(starting_position: MapPosition) -> Player {
        Player { starting_position }
    }

    pub fn spawn(self, commands: &mut Commands, images: &Images, marker: impl Component) {
        let mut transform = Transform::from_xyz(0.0, 0.0, 2.0);
        self.starting_position.apply_to_transform(&mut transform);

        commands
            .spawn_bundle(SpriteBundle {
                texture: images.player_idle.clone(),
                transform,
                ..Default::default()
            })
            .insert(PlayerMarker)
            .insert(marker);
    }
}
