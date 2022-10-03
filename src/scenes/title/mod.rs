mod ui;

use bevy::{
    app::{AppExit, Plugin as BevyPlugin},
    prelude::*,
};
use bevy_kira_audio::{AudioChannel, AudioControl};
use iyes_loopless::prelude::*;

use crate::{
    resources::prelude::{Direction, *},
    ui::{GameButtonData, OverlayMarker},
};

const PLAY_ID: usize = 0;
const INSTRUCTIONS_ID: usize = 1;
const QUIT_ID: usize = 2;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            GameState::Title,
            SystemSet::new()
                .with_system(self::ui::spawn)
                .with_system(CharacterAnimation::insert_title_character_animation),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Title)
                .with_system(update_character_animation)
                .with_system(handle_action_input.run_on_event::<ActionEvent>())
                .with_system(handle_direction_input.run_on_event::<DirectionEvent>())
                .with_system(play_direction_sfx.run_on_event::<DirectionEvent>())
                .into(),
        )
        .add_exit_system_set(
            GameState::Title,
            SystemSet::new()
                .with_system(cleanup::<OverlayMarker>)
                .with_system(cleanup::<CharacterMarker>),
        );
    }
}

fn handle_direction_input(
    mut query: Query<(&mut GameButtonData, &mut UiColor)>,
    mut direction_event_reader: EventReader<DirectionEvent>,
) {
    for direction_event in direction_event_reader.iter() {
        if matches!(direction_event.value, Direction::Up | Direction::Down) {
            let up = matches!(direction_event.value, Direction::Up);
            let mut selected_id = None;
            for (button, _) in query.iter() {
                if button.selected {
                    selected_id = match button.id {
                        PLAY_ID => Some(if up { QUIT_ID } else { INSTRUCTIONS_ID }),
                        INSTRUCTIONS_ID => Some(if up { PLAY_ID } else { QUIT_ID }),
                        QUIT_ID => Some(if up { INSTRUCTIONS_ID } else { PLAY_ID }),
                        _ => unreachable!("The button id was not declared"),
                    };
                }
            }
            if let Some(id) = selected_id {
                for (mut button, mut color) in query.iter_mut() {
                    if button.id == id {
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
}

fn handle_action_input(
    mut game_state_event_writer: EventWriter<SceneTransitionEvent>,
    mut query: Query<&mut GameButtonData>,
    mut action_event_reader: EventReader<ActionEvent>,
    mut exit: EventWriter<AppExit>,
) {
    for action_event in action_event_reader.iter() {
        match action_event.value {
            Action::Pick => {
                for button in query.iter_mut() {
                    if button.selected {
                        match button.id {
                            PLAY_ID => {
                                game_state_event_writer.send(SceneTransitionEvent::selection());
                            }
                            INSTRUCTIONS_ID => {
                                game_state_event_writer.send(SceneTransitionEvent::instructions());
                            }
                            QUIT_ID => {
                                exit.send(AppExit);
                            }
                            _ => unreachable!("The button id was not declared"),
                        }
                    }
                }
            }
            Action::Exit => exit.send(AppExit),
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

pub fn update_character_animation(
    time: Res<Time>,
    mut query: Query<&mut TextureAtlasSprite, With<CharacterMarker>>,
    mut character_animation: ResMut<CharacterAnimation>,
) {
    let mut sprite = query.single_mut();

    character_animation.tick(time.delta());

    if character_animation.secondary_timer_finished() {
        character_animation.reset_with_blink_row();
    }

    if character_animation.primary_timer_finished() {
        if sprite.index == BLINK_ROW_LAST_FRAME_INDEX {
            character_animation.reset_with_front_row();
        } else {
            character_animation.next_index();
        }
    }

    sprite.index = character_animation.sprite_index();
}
