mod ui;

use bevy::{app::Plugin as BevyPlugin, prelude::*};

use crate::{
    level::{LevelInsertionEvent, LevelKind, LevelResource},
    resources::prelude::*,
};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Win),
            (
                save,
                self::ui::spawn,
                CharacterAnimation::insert_happy_character_animation,
            ),
        )
        .add_systems(
            Update,
            (
                handle_action_input.run_if(on_event::<ActionInputEvent>),
                update_character_animation,
            )
                .run_if(in_state(GameState::Win)),
        )
        .add_systems(
            OnExit(GameState::Win),
            (
                cleanup::<game_ui::OverlayMarker>,
                cleanup::<CharacterMarker>,
            )
                .chain(),
        );
    }
}

fn save(mut save_file: ResMut<SaveFile>, level: Res<LevelResource>) {
    save_file.set_new_record(&level);
    save_file.unlock_new_level(&level);
    save_file.save();
}

fn handle_action_input(
    mut level_instertion_event_writer: EventWriter<LevelInsertionEvent>,
    mut game_state_event_writer: EventWriter<SceneTransitionEvent>,
    mut action_event_reader: EventReader<ActionInputEvent>,
    level: Res<LevelResource>,
) {
    for action_event in action_event_reader.read() {
        match action_event.value {
            ActionInput::Select => match level.kind() {
                LevelKind::Stock(index) => {
                    if level.is_last() {
                        game_state_event_writer
                            .write(SceneTransitionEvent::selection(SelectionKind::Stock));
                    } else {
                        level_instertion_event_writer
                            .write(LevelInsertionEvent::new(LevelKind::Stock(index + 1)));
                    }
                }
                LevelKind::Custom(_) => {
                    game_state_event_writer
                        .write(SceneTransitionEvent::selection(SelectionKind::Custom));
                }
                LevelKind::Playtest(_) => {
                    unreachable!("A playtest level cannot be won");
                }
                LevelKind::Editable => {
                    unreachable!("An editable level cannot be won");
                }
            },
            ActionInput::Exit => {
                game_state_event_writer.write(SceneTransitionEvent::title());
            }
            _ => {}
        }
    }
}

fn update_character_animation(
    time: Res<Time>,
    mut character_animation: ResMut<CharacterAnimation>,
    mut query: Query<&mut Sprite, With<CharacterMarker>>,
) {
    character_animation.tick(time.delta());
    if character_animation.primary_timer_just_finished() {
        let mut sprite = query.single_mut().unwrap();
        character_animation.next_index();
        sprite.texture_atlas.as_mut().unwrap().index = character_animation.sprite_index();
    }
}
