mod editor;
mod instructions;
mod level;
mod loading;
mod options;
mod selection;
mod title;
mod win;

use bevy::prelude::*;

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(loading::LoadingPlugin)
            .add_plugin(title::TitlePlugin)
            .add_plugin(editor::EditorPlugin)
            .add_plugin(options::OptionsPlugin)
            .add_plugin(instructions::InstructionsPlugin)
            .add_plugin(selection::SelectionPlugin)
            .add_plugin(level::LevelPlugin)
            .add_plugin(win::WinPlugin);
    }
}
