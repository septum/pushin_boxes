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
    let font = &fonts.fredoka;

    let overlay = Overlay::new();
    let mut actions = Housing::percent(100.0, 90.0);
    let top = Housing::percent(100.0, 50.0);
    let bottom = Housing::percent(100.0, 50.0);
    let footer = Housing::percent(100.0, 10.0);

    let title = EmbossedText::big("Pushin'\nBoxes", font);
    let notice = SimpleText::small("Created by septum | https://septum.io", font);
    let play = ActionButton::full("Play", font);
    let editor = ActionButton::full("Editor", font);
    let options = ActionButton::full("Options", font);
    let quit = ActionButton::full("Quit", font);

    actions.justify_content(JustifyContent::SpaceEvenly);

    overlay.spawn(
        commands,
        |parent| {
            top.spawn(parent, |parent| {
                title.spawn(parent);
            });
            bottom.spawn(parent, |parent| {
                actions.spawn(parent, |parent| {
                    play.spawn(parent, ButtonMarker::play());
                    editor.spawn(parent, ButtonMarker::editor());
                    options.spawn(parent, ButtonMarker::options());
                    quit.spawn(parent, ButtonMarker::quit());
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
