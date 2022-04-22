mod ui;

use bevy::prelude::*;

use crate::{resources::ResourcesHandles, state::GameState};

#[derive(Component)]
struct CleanupMarker;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Loading).with_system(setup))
            .add_system_set(SystemSet::on_exit(GameState::Loading).with_system(cleanup));
    }
}

fn setup(mut commands: Commands, resources: Res<ResourcesHandles>) {
    ui::spawn(&mut commands, &resources.assets);
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<CleanupMarker>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
