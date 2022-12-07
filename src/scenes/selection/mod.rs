mod ui;

use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_kira_audio::{AudioChannel, AudioControl};
use iyes_loopless::prelude::*;

use crate::{
    resources::prelude::*,
    ui::{GameButtonData, OverlayMarker},
};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        for custom in [false, true] {
            app.add_enter_system(GameState::Selection(custom), self::ui::spawn)
                .add_system_set(
                    ConditionSet::new()
                        .run_in_state(GameState::Selection(custom))
                        .with_system(handle_action_input.run_on_event::<ActionInputEvent>())
                        .with_system(handle_direction_input.run_on_event::<DirectionInputEvent>())
                        .with_system(play_direction_sfx.run_on_event::<DirectionInputEvent>())
                        .into(),
                )
                .add_exit_system(GameState::Selection(custom), cleanup::<OverlayMarker>);
        }
    }
}

fn handle_direction_input(
    mut query: Query<(&mut GameButtonData, &mut UiColor)>,
    mut direction_event_reader: EventReader<DirectionInputEvent>,
    game_state: Res<CurrentState<GameState>>,
    save_file: Res<SaveFile>,
) {
    let is_custom_selection = if let GameState::Selection(custom) = game_state.0 {
        custom
    } else {
        unreachable!("The current game state is invalid, it should be Selection");
    };

    for direction_event in direction_event_reader.iter() {
        let mut selected_index: Option<usize> = None;
        for (button, _) in query.iter() {
            if button.selected {
                let index = match direction_event.value {
                    DirectionInput::Up => button.id.saturating_sub(4),
                    DirectionInput::Down => button.id + 4,
                    DirectionInput::Left => button.id.saturating_sub(1),
                    DirectionInput::Right => button.id + 1,
                };

                let max_value = if is_custom_selection {
                    save_file.total_custom_levels()
                } else {
                    save_file.unlocked_levels()
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
            for (mut button, mut color) in query.iter_mut() {
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
    game_state: Res<CurrentState<GameState>>,
) {
    let is_custom_selection = if let GameState::Selection(custom) = game_state.0 {
        custom
    } else {
        unreachable!("The current game state is invalid, it should be Selection");
    };

    for action_event in action_event_reader.iter() {
        match action_event.value {
            ActionInput::Pick => {
                for button in query.iter_mut() {
                    if button.selected {
                        let tag = if is_custom_selection {
                            LevelTag::Custom(
                                button
                                    .payload
                                    .clone()
                                    .expect("The button payload was empty"),
                            )
                        } else {
                            LevelTag::Stock(button.id)
                        };

                        level_insertion_event_writer.send(LevelInsertionEvent::new(tag));
                    }
                }
            }
            ActionInput::Selection => {
                scene_transition_event_writer
                    .send(SceneTransitionEvent::selection(!is_custom_selection));
            }
            ActionInput::Exit => {
                scene_transition_event_writer.send(SceneTransitionEvent::title());
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
    for _ in direction_event_reader.iter() {
        sfx.play(sounds.sfx_move_character.clone());
    }
}
