mod ui;

use std::{
    env,
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use bevy::{app::Plugin as BevyPlugin, input::keyboard::KeyboardInput, prelude::*};
use bevy_kira_audio::{AudioChannel, AudioControl};
use iyes_loopless::prelude::*;
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
        .add_enter_system_set(
            GameState::Passed,
            SystemSet::new().with_system(self::ui::spawn),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Passed)
                .with_system(handle_action_input.run_on_event::<ActionInputEvent>())
                .with_system(handle_text_input)
                .into(),
        )
        .add_exit_system(GameState::Passed, cleanup::<OverlayMarker>);
    }
}

fn handle_action_input(
    mut game_state_event_writer: EventWriter<SceneTransitionEvent>,
    mut action_event_reader: EventReader<ActionInputEvent>,
) {
    for action_event in action_event_reader.iter() {
        if matches!(action_event.value, ActionInput::Exit) {
            game_state_event_writer.send(SceneTransitionEvent::title());
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
    mut character_event_reader: EventReader<ReceivedCharacter>,
    mut query: Query<(&mut Text, &DynamicTextData)>,
    mut level_name: Local<String>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    let (mut text, data) = query.single_mut();
    if level_name.len() < 16 {
        for character_event in character_event_reader.iter() {
            if level_name_regex
                .value
                .is_match(&character_event.char.to_string())
            {
                sfx.play(sounds.sfx_move_character.clone());
                level_name.push(character_event.char);
            }
        }

        text.sections[0].value = match data.id {
            LEVEL_NAME_ID => level_name.to_string(),
            _ => unreachable!("The text id does not exists"),
        };
    }

    if text_cursor.blink_timer.tick(time.delta()).just_finished() {
        text_cursor.blink_toggle = !text_cursor.blink_toggle;
    }

    if text_cursor.blink_toggle {
        text.sections[1].style.color = Colors::TRANSPARENT;
    } else {
        text.sections[1].style.color = Colors::SECONDARY;
    }

    for event in keyboard_input_events.iter() {
        if event.state.is_pressed() {
            if let Some(key_code) = event.key_code {
                match key_code {
                    KeyCode::Back => {
                        sfx.play(sounds.sfx_undo_move.clone());
                        level_name.pop();
                    }
                    KeyCode::Return => {
                        if level_name.len() > 0 {
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

                            save_file.insert_custom_level_record(
                                format!("{}${uuid}", *level_name),
                                level.get_set_record(),
                            );

                            save_file.save();
                            game_state_event_writer.send(SceneTransitionEvent::selection(true));
                        }
                    }
                    _ => (),
                };
            }
        }
    }
}
