use bevy::{input::system::exit_on_esc_system, prelude::*};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pushin Boxes".to_string(),
            width: 640.0,
            height: 640.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb_u8(28, 28, 28)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(exit_on_esc_system)
        .run();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Title,
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
