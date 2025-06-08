mod editor;
mod instructions;
mod level;
mod limit;
mod options;
mod passed;
mod selection;
mod title;
mod win;

use bevy::{app::Plugin as BevyPlugin, prelude::*};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            title::Plugin,
            instructions::Plugin,
            editor::Plugin,
            limit::Plugin,
            passed::Plugin,
            options::Plugin,
            selection::Plugin,
            level::Plugin,
            win::Plugin,
        ));
    }
}
