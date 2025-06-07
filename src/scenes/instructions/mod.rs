mod ui;

use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{resources::prelude::*, ui::OverlayMarker};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(self::ui::spawn.in_schedule(OnEnter(GameState::Instructions)))
            .add_systems(
                (
                    handle_input.run_if(on_event::<ActionInputEvent>()),
                    play_sfx.run_if(on_event::<ActionInputEvent>()),
                )
                    .in_set(OnUpdate(GameState::Instructions)),
            )
            .add_system(cleanup::<OverlayMarker>.in_schedule(OnExit(GameState::Instructions)));
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
