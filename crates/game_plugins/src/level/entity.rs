use bevy::prelude::*;

use game_core::map::MapPosition;

use crate::level::apply_position_to_translation;

#[derive(Component, Deref, DerefMut, PartialEq, Clone, Copy, Default)]
pub struct EntityComponent(MapPosition);

impl EntityComponent {
    pub fn spawn(position: MapPosition, commands: &mut Commands, image: Handle<Image>) {
        let mut translation = Vec3::default();
        apply_position_to_translation(&position, &mut translation);

        let transform = Transform::from_translation(translation);
        let sprite = Sprite { image, ..default() };
        commands
            .spawn((sprite, transform))
            .insert(EntityComponent(position));
    }
}
