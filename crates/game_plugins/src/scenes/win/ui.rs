use bevy::prelude::*;
use bevy_ui_bits::{Container, Root, SimpleText, UiText};

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
        format!("NEW RECORD:\n{}", level.moves_in_time(' '))
    } else {
        " \n ".to_string()
    };

    let root = Root::default();
    let center = Container::size(Val::Px(540.0), Val::Px(200.0))
        .margin(UiRect::bottom(Val::Px(100.0)))
        .justify_between();

    let record = SimpleText::medium(&record, font).color(crate::theme::SECONDARY.into());
    let title = SimpleText::large("You Win!   ", font).color(crate::theme::PRIMARY.into());
    let press_button = SimpleText::small("Press SPACE to continue", font);

    commands.spawn((
        root,
        children![(center, children![record, title, press_button])],
    ));
}
