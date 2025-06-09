mod ui;

use std::{
    env,
    fs::{File, create_dir_all},
    io::Write,
    path::PathBuf,
};

use bevy::{
    app::Plugin as BevyPlugin,
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
};
use bevy_kira_audio::{AudioChannel, AudioControl};

use regex::Regex;
use uuid::Uuid;

use crate::{
    resources::prelude::*,
    ui::{DynamicTextData, OverlayMarker},
};

#[derive(Resource)]
pub struct LevelNameRegex {
    pub value: Regex,
}

#[derive(Resource)]
pub struct TextCursor {
    pub blink_timer: Timer,
    pub blink_toggle: bool,
}

const LEVEL_NAME_ID: usize = 1;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelNameRegex {
            value: Regex::new(r"^[a-zA-Z ]$").unwrap(),
        })
        .insert_resource(TextCursor {
            blink_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            blink_toggle: true,
        })
        .add_systems(OnEnter(GameState::Passed), self::ui::spawn)
        .add_systems(
            Update,
            (
                handle_action_input.run_if(on_event::<ActionInputEvent>),
                handle_text_input,
            )
                .run_if(in_state(GameState::Passed)),
        )
        .add_systems(OnExit(GameState::Passed), cleanup::<OverlayMarker>);
    }
}

fn handle_action_input(
    mut game_state_event_writer: EventWriter<SceneTransitionEvent>,
    mut action_event_reader: EventReader<ActionInputEvent>,
) {
    for action_event in action_event_reader.read() {
        if matches!(action_event.value, ActionInput::Exit) {
            game_state_event_writer.write(SceneTransitionEvent::title());
        }
    }
}

// TODO: Refactor this
#[allow(clippy::too_many_arguments)]
pub fn handle_text_input(
    time: Res<Time>,
    level: Res<Level>,
    level_name_regex: Res<LevelNameRegex>,
    asset_server: Res<AssetServer>,
    mut game_state_event_writer: EventWriter<SceneTransitionEvent>,
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
                    let serialized_string =
                        ron::ser::to_string(&level.kind.get_playtest_state()).unwrap();
                    let levels_path = format!("levels/custom/{}.lvl", &uuid);
                    let assets_path = format!("assets/{}", &levels_path);
                    let path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
                        PathBuf::from(manifest_dir).join(assets_path)
                    } else {
                        PathBuf::from(assets_path)
                    };

                    let parent_path = path.parent().unwrap();
                    create_dir_all(parent_path).unwrap();

                    let mut file = File::create(path).unwrap();
                    file.write_all(serialized_string.as_bytes()).unwrap();

                    level_handles.insert_custom(uuid, asset_server.load(&levels_path));

                    let lower_level_name = level_name.to_lowercase();

                    save_file.insert_custom_level_record(
                        format!("{lower_level_name}${uuid}"),
                        level.get_set_record(),
                    );

                    *level_name = String::new();
                    *writer.text(entity, 0) = String::new();

                    save_file.save();
                    game_state_event_writer
                        .write(SceneTransitionEvent::selection(SelectionKind::Custom));
                }
            }
            _ => {}
        }
    }
}
