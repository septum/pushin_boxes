use bevy::prelude::*;

use crate::{resources::AssetsHandles, ui};

use super::CleanupMarker;

pub fn spawn(commands: &mut Commands, _assets: &AssetsHandles) {
    let overlay = ui::Overlay::new();

    overlay.spawn(commands, CleanupMarker, |_parent| {});
}
