use bevy::prelude::*;
use game_ui::{Container, GameText, Overlay, SimpleText};

use crate::{assets::prelude::*, level::LevelResource, save_file::SaveFile};

pub fn spawn(
    mut commands: Commands,
    fonts: Res<Fonts>,
    level: Res<LevelResource>,
    save_file: Res<SaveFile>,
) {
    let font = fonts.primary();

    let old_record = save_file.get_record(level.kind());
    let record = if level.is_new_record(&old_record) {
        format!("NEW RECORD:\n{}", level.moves_in_time(" "))
    } else {
        " \n ".to_string()
    };

    let overlay = Overlay::default();
    let mut center = Container::size(540.0, 200.0);

    let mut record = SimpleText::medium(record, font);
    let mut title = SimpleText::large("You Win!   ", font);
    let press_button = SimpleText::small("Press SPACE to continue", font);

    center.margin_bottom(100.0).justify_between();
    record.secondary();
    title.primary();

    overlay.spawn(&mut commands, |parent| {
        center.spawn(parent, |parent| {
            record.spawn(parent);
            title.spawn(parent);
            press_button.spawn(parent);
        });
    });
}
