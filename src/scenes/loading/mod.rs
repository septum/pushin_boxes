mod ui;

use bevy::prelude::*;

use crate::{resources::prelude::*, state::GameState};

use ui::{spawn_ui, UiMarker};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Loading).with_system(setup))
            .add_system_set(SystemSet::on_exit(GameState::Loading).with_system(cleanup));
    }
}

fn setup(mut commands: Commands, fonts: Res<Fonts>) {
    spawn_ui(&mut commands, &fonts);
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<UiMarker>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
