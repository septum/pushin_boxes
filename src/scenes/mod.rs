mod loading;
mod title;
mod level;


use bevy::prelude::*;

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(loading::LoadingPlugin)
            .add_plugin(title::TitlePlugin)
            .add_plugin(level::LevelPlugin);
    }
}
