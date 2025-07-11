use std::{
    env,
    fs::{File, create_dir_all},
    io::Write,
    path::PathBuf,
};

use bevy::{
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
};
use bevy_kira_audio::{AudioChannel, AudioControl};
use regex::Regex;
use uuid::Uuid;

use game_core::{
    input::{Action, Input},
    level::LevelKind,
};
use game_ui::{Colors, DynamicTextData};

use crate::{
    assets::prelude::*,
    input::InputEvent,
    level::{LevelHandles, LevelResource, LevelStateAsset},
    save_file::SaveFile,
    state::{GameStateTransitionEvent, SelectionKind},
};

use super::ui::LEVEL_NAME_ID;

#[derive(Resource)]
pub struct LevelNameRegex {
    pub value: Regex,
}

#[derive(Resource)]
pub struct TextCursor {
    pub blink_timer: Timer,
    pub blink_toggle: bool,
}

pub fn handle_input(
    mut game_state_event_writer: EventWriter<GameStateTransitionEvent>,
    mut input_event_reader: EventReader<InputEvent>,
) {
    for input_event in input_event_reader.read() {
        if matches!(**input_event, Input::Action(Action::Exit)) {
            game_state_event_writer.write(GameStateTransitionEvent::title());
        }
    }
}

// TODO: Refactor this
#[allow(clippy::too_many_arguments)]
pub fn handle_text_input(
    time: Res<Time>,
    level: Res<LevelResource>,
    level_name_regex: Res<LevelNameRegex>,
    asset_server: Res<AssetServer>,
    mut game_state_event_writer: EventWriter<GameStateTransitionEvent>,
    mut save_file: ResMut<SaveFile>,
    mut level_handles: ResMut<LevelHandles>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut text_cursor: ResMut<TextCursor>,
    mut writer: TextUiWriter,
    mut query_entity: Query<(Entity, &DynamicTextData)>,
    mut level_name: Local<String>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    let (entity, data) = query_entity.single_mut().unwrap();
    if text_cursor.blink_timer.tick(time.delta()).just_finished() {
        text_cursor.blink_toggle = !text_cursor.blink_toggle;
    }

    if text_cursor.blink_toggle {
        *writer.color(entity, 1) = TextColor(Colors::TRANSPARENT);
    } else {
        *writer.color(entity, 1) = TextColor(Colors::SECONDARY);
    }

    for event in keyboard_input_events.read() {
        if !event.state.is_pressed() {
            continue;
        }

        match &event.logical_key {
            Key::Character(character) => {
                if level_name.len() < 16 {
                    if level_name_regex.value.is_match(character) {
                        sfx.play(sounds.sfx_move_character.clone());
                        level_name.push_str(character);
                    }

                    *writer.text(entity, 0) = match data.id {
                        LEVEL_NAME_ID => level_name.to_string(),
                        _ => unreachable!("The text id does not exists"),
                    };
                }
            }
            Key::Space => {
                if level_name.len() < 16 {
                    let character = " ";

                    if level_name_regex.value.is_match(character) {
                        sfx.play(sounds.sfx_move_character.clone());
                        level_name.push_str(character);
                    }

                    *writer.text(entity, 0) = match data.id {
                        LEVEL_NAME_ID => level_name.to_string(),
                        _ => unreachable!("The text id does not exists"),
                    };
                }
            }
            Key::Backspace => {
                sfx.play(sounds.sfx_undo_move.clone());
                level_name.pop();

                *writer.text(entity, 0) = match data.id {
                    LEVEL_NAME_ID => level_name.to_string(),
                    _ => unreachable!("The text id does not exists"),
                };
            }
            Key::Enter => {
                if !level_name.is_empty() {
                    sfx.play(sounds.sfx_set_zone.clone());
                    let uuid = Uuid::new_v4();
                    let serialized_string = match level.kind() {
                        LevelKind::Editable(state) => {
                            ron::ser::to_string(&LevelStateAsset::new(*state)).unwrap()
                        }
                        _ => panic!("Cannot get the state if the level kind is not playtest"),
                    };
                    let levels_path = format!("levels/custom/{}.lvl", &uuid);
                    let assets_path = format!("assets/{}", &levels_path);

                    #[cfg(not(target_family = "wasm"))]
                    {
                        let path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
                            PathBuf::from(manifest_dir).join(assets_path)
                        } else {
                            PathBuf::from(assets_path)
                        };

                        let parent_path = path.parent().unwrap();
                        create_dir_all(parent_path).unwrap();

                        let mut file = File::create(path).unwrap();
                        file.write_all(serialized_string.as_bytes()).unwrap();
                    }

                    level_handles.insert_custom(uuid, asset_server.load(&levels_path));

                    let lower_level_name = level_name.to_lowercase();

                    save_file.insert_custom_level_record(
                        format!("{lower_level_name}${uuid}"),
                        level.record().clone(),
                    );

                    *level_name = String::new();
                    *writer.text(entity, 0) = String::new();

                    save_file.save();
                    game_state_event_writer
                        .write(GameStateTransitionEvent::selection(SelectionKind::Custom));
                }
            }
            _ => {}
        }
    }
}
