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
    let button_size = Size::new(Val::Percent(50.0), Val::Px(50.0));

    let overlay = Overlay::new();
    let mut actions = Housing::percent(100.0, 90.0);
    let top = Housing::percent(100.0, 50.0);
    let bottom = Housing::percent(100.0, 50.0);
    let footer = Housing::percent(100.0, 10.0);

    let title = EmbossedText::big("Pushin'\nBoxes", font);
    let notice = SimpleText::small("Created by septum | https://septum.io", font);
    let play = ActionButton::new("Play", font, button_size);
    let editor = ActionButton::new("Editor", font, button_size);
    let options = ActionButton::new("Options", font, button_size);
    let quit = ActionButton::new("Quit", font, button_size);

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
