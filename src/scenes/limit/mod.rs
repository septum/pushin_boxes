mod ui;

use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{resources::prelude::*, ui::OverlayMarker};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Limit), self::ui::spawn)
            .add_systems(
                Update,
                (
                    handle_input.run_if(on_event::<ActionInputEvent>),
                    play_sfx.run_if(on_event::<ActionInputEvent>),
                )
                    .run_if(in_state(GameState::Limit)),
            )
            .add_systems(OnExit(GameState::Limit), cleanup::<OverlayMarker>);
    }
}

fn handle_input(
    mut game_state_event_writer: EventWriter<SceneTransitionEvent>,
    mut action_event_reader: EventReader<ActionInputEvent>,
) {
    for action_event in action_event_reader.read() {
        if matches!(action_event.value, ActionInput::Select) {
            game_state_event_writer.send(SceneTransitionEvent::selection(SelectionKind::Custom));
        }
    }
}

fn play_sfx(
    mut action_event_reader: EventReader<ActionInputEvent>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    for action_event in action_event_reader.read() {
        if matches!(action_event.value, ActionInput::Select) {
            sfx.play(sounds.sfx_push_box.clone());
        }
    }
}
