mod ui;

use bevy::{app::Plugin as BevyPlugin, prelude::*};
use iyes_loopless::prelude::*;

use crate::{resources::prelude::*, ui::OverlayMarker};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            GameState::Win,
            SystemSet::new()
                .with_system(save)
                .with_system(self::ui::spawn)
                .with_system(CharacterAnimation::insert_happy_character_animation),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Win)
                .with_system(handle_action_input.run_on_event::<ActionInputEvent>())
                .with_system(update_character_animation)
                .into(),
        )
        .add_exit_system(GameState::Win, cleanup::<OverlayMarker>)
        .add_exit_system(GameState::Win, cleanup::<CharacterMarker>);
    }
}

fn save(mut save_file: ResMut<SaveFile>, level: Res<Level>) {
    if matches!(level.kind, LevelKind::Stock(_)) {
        save_file.set_new_record(&level);
        save_file.unlock_new_level(&level);
        save_file.save();
    }
}

fn handle_action_input(
    mut level_instertion_event_writer: EventWriter<LevelInsertionEvent>,
    mut game_state_event_writer: EventWriter<SceneTransitionEvent>,
    mut action_event_reader: EventReader<ActionInputEvent>,
    level: Res<Level>,
) {
    for action_event in action_event_reader.iter() {
        match action_event.value {
            ActionInput::Select => {
                match &level.kind {
                    LevelKind::Stock(index) => {
                        if level.is_last() {
                            game_state_event_writer.send(SceneTransitionEvent::selection(false));
                        } else {
                            level_instertion_event_writer
                                .send(LevelInsertionEvent::new(LevelKind::Stock(index + 1)));
                        }
                    }
                    LevelKind::Custom(_) => {
                        game_state_event_writer.send(SceneTransitionEvent::selection(true));
                    }
                    LevelKind::Playtest(_) => {
                        unreachable!("A playtest level cannot be won");
                    }
                    LevelKind::Editable => {
                        unreachable!("An editable level cannot be won");
                    }
                };
            }
            ActionInput::Exit => {
                game_state_event_writer.send(SceneTransitionEvent::title());
            }
            _ => {}
        }
    }
}

fn update_character_animation(
    time: Res<Time>,
    mut character_animation: ResMut<CharacterAnimation>,
    mut query: Query<&mut TextureAtlasSprite, With<CharacterMarker>>,
) {
    character_animation.tick(time.delta());
    if character_animation.primary_timer_just_finished() {
        let mut sprite = query.single_mut();
        character_animation.next_index();
        sprite.index = character_animation.sprite_index();
    }
}
