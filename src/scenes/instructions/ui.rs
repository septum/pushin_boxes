use bevy::prelude::*;

use crate::{
    resources::prelude::*,
    ui::{EmbossedText, Housing, Overlay, Picture, SimpleText},
};

#[derive(Component)]
pub struct UiMarker;

fn spawn_ui_camera(commands: &mut Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiMarker);
}

pub fn spawn_ui(commands: &mut Commands, images: &Images, fonts: &Fonts) {
    let font = &fonts.upheavtt;

    let overlay = Overlay::new();
    let line_a = Housing::percent(100.0, 10.0);
    let line_b = Housing::percent(100.0, 20.0);
    let line_c = Housing::percent(100.0, 20.0);
    let line_d = Housing::percent(100.0, 20.0);
    let line_e = Housing::percent(100.0, 20.0);
    let line_f = Housing::percent(100.0, 10.0);

    let controls = Picture::full(&images.controls);
    let mut pushin = Picture::px(64.0, 64.0, &images.player.pushin);
    let mut pbox = Picture::px(64.0, 64.0, &images.entities.pbox);
    let mut zone = Picture::px(64.0, 64.0, &images.entities.zone);

    let how_to_play = EmbossedText::medium("How to Play", font);
    let press = SimpleText::medium("Press", font);
    let or = SimpleText::medium("or", font);
    let to_move = SimpleText::medium("to move        ,", font);
    let to_win = SimpleText::medium("pushing all         into         to win!", font);
    let press_space = EmbossedText::small("Press [SPACE] to continue", font);

    pushin.left_position(Val::Percent(56.0));
    pbox.left_position(Val::Percent(39.0));
    zone.left_position(Val::Percent(62.0));

    overlay.spawn(
        commands,
        |parent| {
            line_a.spawn(parent, |parent| {
                how_to_play.spawn(parent);
            });
            line_b.spawn(parent, |parent| {
                press.spawn(parent);
            });
            line_c.spawn(parent, |parent| {
                controls.spawn(parent);
                or.spawn(parent);
            });
            line_d.spawn(parent, |parent| {
                pushin.spawn(parent);
                to_move.spawn(parent);
            });
            line_e.spawn(parent, |parent| {
                pbox.spawn(parent);
                zone.spawn(parent);
                to_win.spawn(parent);
            });
            line_f.spawn(parent, |parent| {
                press_space.spawn(parent);
            });
        },
        UiMarker,
    );

    spawn_ui_camera(commands);
}
