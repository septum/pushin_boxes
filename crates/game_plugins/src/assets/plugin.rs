use bevy::{app::Plugin as BevyPlugin, prelude::*};

use crate::assets::sounds;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(sounds::Plugin);
    }
}
