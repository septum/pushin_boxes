use bevy::prelude::*;

use game_core::map::MapPosition;

use crate::level::MapPositionExtension;

#[derive(Component)]
pub struct Character;

impl Character {
    pub fn spawn(
        position: MapPosition,
        commands: &mut Commands,
        atlas: TextureAtlas,
        image: Handle<Image>,
    ) {
        let mut translation = Vec3::default();
        position.update_translation(&mut translation);

        // TODO: There should be another way to do this proper
        translation.z += 1.;

        let transform = Transform::from_translation(translation);
        let sprite = Sprite {
            image,
            texture_atlas: Some(atlas),
            ..default()
        };
        commands.spawn((sprite, transform)).insert(Character);
    }
}
