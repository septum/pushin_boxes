mod ui;

use bevy::{
    app::{AppExit, Plugin as BevyPlugin},
    prelude::*,
};
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{
    resources::prelude::*,
    ui::{GameButtonData, OverlayMarker},
};

const PLAY_ID: usize = 0;
const INSTRUCTIONS_ID: usize = 1;
const EDITOR_ID: usize = 2;
const OPTIONS_ID: usize = 3;
const QUIT_ID: usize = 4;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Title),
            (
                self::ui::spawn,
                CharacterAnimation::insert_blinking_character_animation,
            ),
        )
        .add_systems(
            Update,
            (
                update_character_animation,
                handle_action_input.run_if(on_event::<ActionInputEvent>),
                handle_direction_input.run_if(on_event::<DirectionInputEvent>),
                play_action_sfx.run_if(on_event::<ActionInputEvent>),
                play_direction_sfx.run_if(on_event::<DirectionInputEvent>),
            )
                .run_if(in_state(GameState::Title)),
        )
        .add_systems(
            OnExit(GameState::Title),
            (cleanup::<OverlayMarker>, cleanup::<CharacterMarker>),
        );
    }
}

fn handle_direction_input(
    mut query: Query<(&mut GameButtonData, &mut BackgroundColor)>,
    mut direction_event_reader: EventReader<DirectionInputEvent>,
) {
    for direction_event in direction_event_reader.read() {
        if matches!(
            direction_event.value,
            DirectionInput::Up | DirectionInput::Down
        ) {
            let up = matches!(direction_event.value, DirectionInput::Up);
            let mut selected_id = None;
            for (button, _) in query.iter() {
                if button.selected {
                    selected_id = match button.id {
                        PLAY_ID => Some(if up { QUIT_ID } else { INSTRUCTIONS_ID }),
                        INSTRUCTIONS_ID => Some(if up { PLAY_ID } else { EDITOR_ID }),
                        EDITOR_ID => Some(if up { INSTRUCTIONS_ID } else { OPTIONS_ID }),
                        OPTIONS_ID => Some(if up { EDITOR_ID } else { QUIT_ID }),
                        QUIT_ID => Some(if up { OPTIONS_ID } else { PLAY_ID }),
                        _ => unreachable!("The button id was not declared"),
                    };
                }
            }
            if let Some(id) = selected_id {
                for (mut button, mut color) in &mut query {
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
    mut action_event_reader: EventReader<ActionInputEvent>,
    mut exit: EventWriter<AppExit>,
) {
    for action_event in action_event_reader.read() {
        match action_event.value {
            ActionInput::Select => {
                for button in &mut query {
                    if button.selected {
                        match button.id {
                            PLAY_ID => {
                                game_state_event_writer
                                    .write(SceneTransitionEvent::selection(SelectionKind::Stock));
                            }
                            INSTRUCTIONS_ID => {
                                game_state_event_writer.write(SceneTransitionEvent::instructions());
                            }
                            EDITOR_ID => {
                                game_state_event_writer.write(SceneTransitionEvent::editor());
                            }
                            OPTIONS_ID => {
                                game_state_event_writer.write(SceneTransitionEvent::options());
                            }
                            QUIT_ID => {
                                exit.write(AppExit::Success);
                            }
                            _ => unreachable!("The button id was not declared"),
                        }
                    }
                }
            }
            ActionInput::Exit => {
                exit.write(AppExit::Success);
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
            ActionInput::Exit => {
                sfx.play(sounds.sfx_push_box.clone());
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

pub fn update_character_animation(
    time: Res<Time>,
    mut query: Query<&mut Sprite, With<CharacterMarker>>,
    mut character_animation: ResMut<CharacterAnimation>,
) {
    let mut sprite = query.single_mut().unwrap();

    character_animation.tick(time.delta());

    if character_animation.secondary_timer_just_finished() {
        character_animation.set_blink_row();
        character_animation.reset_primary_timer();
        character_animation.reset_secondary_timer();
    }

    if character_animation.primary_timer_just_finished() {
        if sprite.texture_atlas.as_mut().unwrap().index == BLINK_ROW_LAST_FRAME_INDEX {
            character_animation.set_front_row();
            character_animation.reset_primary_timer();
            character_animation.reset_secondary_timer();
        } else {
            character_animation.next_index();
        }
    }

    sprite.texture_atlas.as_mut().unwrap().index = character_animation.sprite_index();
}
