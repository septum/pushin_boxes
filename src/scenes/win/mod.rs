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
                .with_system(CharacterAnimation::insert_win_character_animation),
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
    save_file.set_if_new_record(&level);
    save_file.unlock_if_new_level(&level);
    save_file.save();
}

fn handle_action_input(
    mut level_instertion_event_writer: EventWriter<LevelInsertionEvent>,
    mut game_state_event_writer: EventWriter<SceneTransitionEvent>,
    mut action_event_reader: EventReader<ActionInputEvent>,
    level: Res<Level>,
) {
    for action_event in action_event_reader.iter() {
        match action_event.value {
            ActionInput::Pick => {
                match &level.tag {
                    LevelTag::Stock(index) => {
                        if level.is_last() {
                            game_state_event_writer.send(SceneTransitionEvent::selection());
                        } else {
                            level_instertion_event_writer
                                .send(LevelInsertionEvent::new(LevelTag::Stock(index + 1)));
                        }
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
    if character_animation.tick_primary(time.delta()) {
        let mut sprite = query.single_mut();
        sprite.index = character_animation.next_sprite_index();
    }
}
