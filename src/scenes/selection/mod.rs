mod ui;

use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_kira_audio::{AudioChannel, AudioControl};
use iyes_loopless::prelude::*;

use crate::{
    resources::prelude::{Direction, *},
    ui::{GameButtonData, OverlayMarker},
};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Selection, self::ui::spawn)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Selection)
                    .with_system(handle_action_input.run_on_event::<ActionEvent>())
                    .with_system(handle_direction_input.run_on_event::<DirectionEvent>())
                    .with_system(play_direction_sfx.run_on_event::<DirectionEvent>())
                    .into(),
            )
            .add_exit_system(GameState::Selection, cleanup::<OverlayMarker>);
    }
}

fn handle_direction_input(
    mut query: Query<(&mut GameButtonData, &mut UiColor)>,
    mut direction_event_reader: EventReader<DirectionEvent>,
    save_file: Res<SaveFile>,
) {
    for direction_event in direction_event_reader.iter() {
        let mut selected_index: Option<usize> = None;
        for (button, _) in query.iter() {
            if button.selected {
                let index = match direction_event.value {
                    Direction::Up => button.id.saturating_sub(4),
                    Direction::Down => button.id + 4,
                    Direction::Left => button.id.saturating_sub(1),
                    Direction::Right => button.id + 1,
                };

                selected_index = Some(if index < save_file.stock_levels_len() {
                    index
                } else {
                    save_file.stock_levels_len() - 1
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
    mut action_event_reader: EventReader<ActionEvent>,
) {
    for action_event in action_event_reader.iter() {
        match action_event.value {
            Action::Pick => {
                for button in query.iter_mut() {
                    if button.selected {
                        level_insertion_event_writer
                            .send(LevelInsertionEvent::new(LevelTag::Stock(button.id)));
                    }
                }
            }
            Action::Exit => {
                scene_transition_event_writer.send(SceneTransitionEvent::title());
            }
            _ => (),
        }
    }
}

pub fn play_direction_sfx(
    mut direction_event_reader: EventReader<DirectionEvent>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    for _ in direction_event_reader.iter() {
        sfx.play(sounds.sfx_move_player.clone());
    }
}
