use bevy::prelude::*;
use bevy_ui_bits::{Container, DynamicTextBuilder, Root, SimpleText, UiText};

use crate::assets::prelude::*;

pub const VOLUME_ID: usize = 1;

pub fn spawn(mut commands: Commands, fonts: Res<Fonts>) {
    let font = fonts.primary();

    let root = Root::new();
    let top = Container::size(Val::Auto, Val::Auto);
    let center = Container::size(Val::Auto, Val::Auto);
    let bottom = Container::size(Val::Auto, Val::Auto);

    let how_to_play = SimpleText::medium("Options", font).color(crate::theme::PRIMARY.into());
    let volume = DynamicTextBuilder::medium("Volume: ", font)
        .id(VOLUME_ID)
        .color(crate::theme::SECONDARY.into());
    let press_button = SimpleText::small("Press ESC to return to the title screen", font)
        .color(crate::theme::PRIMARY.into());

    commands.spawn((
        root,
        children![
            (top, children![how_to_play]),
            (center, children![volume.build()]),
            (bottom, children![press_button])
        ],
    ));
}
