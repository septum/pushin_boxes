mod ui;

use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_kira_audio::{AudioChannel, AudioControl};
use iyes_loopless::prelude::*;

use crate::{resources::prelude::*, ui::OverlayMarker};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Instructions, self::ui::spawn)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Instructions)
                    .with_system(handle_input.run_on_event::<ActionInputEvent>())
                    .with_system(play_sfx.run_on_event::<ActionInputEvent>())
                    .into(),
            )
            .add_exit_system(GameState::Instructions, cleanup::<OverlayMarker>);
    }
}

fn handle_input(
    mut game_state_event_writer: EventWriter<SceneTransitionEvent>,
    mut action_event_reader: EventReader<ActionInputEvent>,
) {
    for action_event in action_event_reader.iter() {
        if matches!(action_event.value, ActionInput::Exit) {
            game_state_event_writer.send(SceneTransitionEvent::title());
        }
    }
}

fn play_sfx(
    mut action_event_reader: EventReader<ActionInputEvent>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    for action_event in action_event_reader.iter() {
        if matches!(action_event.value, ActionInput::Exit) {
            sfx.play(sounds.sfx_push_box.clone());
        }
    }
}
