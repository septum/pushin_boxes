use bevy::prelude::*;

use crate::{
    resources::prelude::*,
    ui::{ActionButton, ButtonMarker, EmbossedText, Housing, Overlay, SimpleText},
};

#[derive(Component)]
pub struct UiMarker;

fn spawn_ui_camera(commands: &mut Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiMarker);
}

pub fn spawn_ui(commands: &mut Commands, fonts: &Fonts) {
    let font = &fonts.upheavtt;
    let button_size = Size::new(Val::Px(400.0), Val::Px(60.0));

    let mut overlay = Overlay::new();
    let top = Housing::percent(100.0, 25.0);
    let mut bottom = Housing::percent(100.0, 35.0);
    let mut actions = Housing::percent(100.0, 60.0);
    let footer = Housing::percent(100.0, 30.0);

    let mut title = EmbossedText::big("Pushin'\nBoxes", font);
    let notice = SimpleText::small("By @septum (gh)\nand @andresweyman (ig)", font);
    let play = ActionButton::new("Play", font, button_size);
    let instructions = ActionButton::new("Instructions", font, button_size);
    let quit = ActionButton::new("Quit", font, button_size);

    title.size(112.0);

    overlay.justify_content(JustifyContent::SpaceEvenly);
    bottom.justify_content(JustifyContent::SpaceBetween);
    actions
        .justify_content(JustifyContent::SpaceEvenly)
        .align_items(AlignItems::Center);

    overlay.spawn(
        commands,
        |parent| {
            top.spawn(parent, |parent| {
                title.spawn(parent);
            });
            bottom.spawn(parent, |parent| {
                actions.spawn(parent, |parent| {
                    play.spawn(parent, ButtonMarker::play(true));
                    instructions.spawn(parent, ButtonMarker::instructions(false));
                    quit.spawn(parent, ButtonMarker::quit(false));
                });
                footer.spawn(parent, |parent| {
                    notice.spawn(parent);
                });
            });
        },
        UiMarker,
    );

    spawn_ui_camera(commands);
}
