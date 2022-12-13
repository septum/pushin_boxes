mod editor;
mod instructions;
mod level;
mod options;
mod passed;
mod selection;
mod title;
mod win;

use bevy::{app::Plugin as BevyPlugin, prelude::*};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(title::Plugin)
            .add_plugin(instructions::Plugin)
            .add_plugin(editor::Plugin)
            .add_plugin(passed::Plugin)
            .add_plugin(options::Plugin)
            .add_plugin(selection::Plugin)
            .add_plugin(level::Plugin)
            .add_plugin(win::Plugin);
    }
}
