mod ui;

use std::{env, fs::remove_file, path::PathBuf};

use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{
    resources::prelude::*,
    ui::{GameButtonData, OverlayMarker},
};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        for state in [GameState::SelectionStock, GameState::SelectionCustom] {
            app.add_systems(OnEnter(state), self::ui::spawn)
                .add_systems(
                    Update,
                    (
                        handle_action_input.run_if(on_event::<ActionInputEvent>),
                        handle_direction_input.run_if(on_event::<DirectionInputEvent>),
                        play_action_sfx.run_if(on_event::<ActionInputEvent>),
                        play_direction_sfx.run_if(on_event::<DirectionInputEvent>),
                    )
                        .run_if(in_state(state)),
                )
                .add_systems(OnExit(state), cleanup::<OverlayMarker>);
        }
    }
}

fn handle_direction_input(
    mut query: Query<(&mut GameButtonData, &mut BackgroundColor)>,
    mut direction_event_reader: EventReader<DirectionInputEvent>,
    game_state: Res<State<GameState>>,
    save_file: Res<SaveFile>,
) {
    let is_stock = game_state.get_selection_kind().is_stock();

    for direction_event in direction_event_reader.read() {
        let mut selected_index: Option<usize> = None;
        for (button, _) in query.iter() {
            if button.selected {
                let index = match direction_event.value {
                    DirectionInput::Up => button.id.saturating_sub(4),
                    DirectionInput::Down => button.id + 4,
                    DirectionInput::Left => button.id.saturating_sub(1),
                    DirectionInput::Right => button.id + 1,
                };

                let max_value = if is_stock {
                    save_file.unlocked_levels()
                } else {
                    save_file.number_custom_levels()
                };

                selected_index = Some(if index < max_value {
                    index
                } else {
                    max_value - 1
                });

                break;
            }
        }

        if let Some(selected_index) = selected_index {
            for (mut button, mut color) in &mut query {
                if selected_index == button.id {
                    button.selected = true;
                    *color = Colors::PRIMARY_DARK.into();
                } else {
                    button.selected = false;
                    *color = Colors::TRANSPARENT.into();
                }
            }
        }
    }
}

fn handle_action_input(
    mut level_insertion_event_writer: EventWriter<LevelInsertionEvent>,
    mut scene_transition_event_writer: EventWriter<SceneTransitionEvent>,
    mut query: Query<&mut GameButtonData>,
    mut action_event_reader: EventReader<ActionInputEvent>,
    mut save_file: ResMut<SaveFile>,
    game_state: Res<State<GameState>>,
) {
    let is_stock = game_state.get_selection_kind().is_stock();

    for action_event in action_event_reader.read() {
        match action_event.value {
            ActionInput::Select => {
                for button in &mut query {
                    if button.selected {
                        let kind = if is_stock {
                            LevelKind::Stock(button.id)
                        } else {
                            LevelKind::Custom(
                                button
                                    .payload
                                    .clone()
                                    .expect("The button payload was empty"),
                            )
                        };

                        level_insertion_event_writer.write(LevelInsertionEvent::new(kind));
                    }
                }
            }
            ActionInput::Toggle => {
                scene_transition_event_writer.write(SceneTransitionEvent::selection(if is_stock {
                    SelectionKind::Custom
                } else {
                    SelectionKind::Stock
                }));
            }

            ActionInput::Delete => {
                for button in &mut query {
                    if button.selected && !is_stock {
                        let payload = button
                            .payload
                            .clone()
                            .expect("The button payload was empty");
                        let parsed_payload: Vec<&str> = payload.split('$').collect();
                        save_file.delete_custom_level_record(&payload);
                        save_file.save();
                        let levels_path = format!("levels/custom/{}.lvl", parsed_payload[1]);
                        let assets_path = format!("assets/{}", &levels_path);
                        let path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
                            PathBuf::from(manifest_dir).join(assets_path)
                        } else {
                            PathBuf::from(assets_path)
                        };
                        remove_file(path).expect("File cannot be removed");
                        scene_transition_event_writer
                            .write(SceneTransitionEvent::selection(SelectionKind::Custom));
                    }
                }
            }
            ActionInput::Exit => {
                scene_transition_event_writer.write(SceneTransitionEvent::title());
            }
            _ => (),
        }
    }
}

fn play_action_sfx(
    mut action_event_reader: EventReader<ActionInputEvent>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    for action_event in action_event_reader.read() {
        match action_event.value {
            ActionInput::Exit | ActionInput::Delete => {
                sfx.play(sounds.sfx_push_box.clone());
            }
            ActionInput::Toggle => {
                sfx.play(sounds.sfx_toggle_volume.clone());
            }
            ActionInput::Select => {
                sfx.play(sounds.sfx_set_zone.clone());
            }
            _ => (),
        }
    }
}

pub fn play_direction_sfx(
    mut direction_event_reader: EventReader<DirectionInputEvent>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    for _ in direction_event_reader.read() {
        sfx.play(sounds.sfx_move_character.clone());
    }
}
