use bevy::{app::Plugin as BevyPlugin, prelude::*};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            super::title::Plugin,
            super::instructions::Plugin,
            super::editor::Plugin,
            super::limit::Plugin,
            super::passed::Plugin,
            super::options::Plugin,
            super::selection::Plugin,
            super::level::Plugin,
            super::win::Plugin,
        ));
    }
}
