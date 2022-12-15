mod ui;

use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_kira_audio::{AudioChannel, AudioControl};
use iyes_loopless::prelude::*;

use crate::{
    resources::prelude::*,
    ui::{DynamicTextData, OverlayMarker},
};

const VOLUME_ID: usize = 1;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Options, self::ui::spawn)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Options)
                    .with_system(handle_action_input.run_on_event::<ActionInputEvent>())
                    .with_system(handle_direction_input.run_on_event::<DirectionInputEvent>())
                    .with_system(play_action_sfx.run_on_event::<ActionInputEvent>())
                    .with_system(play_direction_sfx.run_on_event::<DirectionInputEvent>())
                    .with_system(update_dynamic_text)
                    .into(),
            )
            .add_exit_system(GameState::Options, cleanup::<OverlayMarker>);
    }
}

fn handle_action_input(
    mut game_state_event_writer: EventWriter<SceneTransitionEvent>,
    mut action_event_reader: EventReader<ActionInputEvent>,
    save_file: Res<SaveFile>,
) {
    for action_event in action_event_reader.iter() {
        if matches!(action_event.value, ActionInput::Exit) {
            save_file.save();
            game_state_event_writer.send(SceneTransitionEvent::title());
        }
    }
}

fn handle_direction_input(
    mut direction_event_reader: EventReader<DirectionInputEvent>,
    mut sounds: ResMut<Sounds>,
    mut save_file: ResMut<SaveFile>,
) {
    for direction_event in direction_event_reader.iter() {
        match direction_event.value {
            DirectionInput::Left => {
                sounds.decrease_volume();
                save_file.set_volume(sounds.get_volume());
            }
            DirectionInput::Right => {
                sounds.increase_volume();
                save_file.set_volume(sounds.get_volume());
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
    for direction_event in direction_event_reader.iter() {
        match direction_event.value {
            DirectionInput::Left | DirectionInput::Right => {
                sfx.play(sounds.sfx_move_character.clone());
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
    for action_event in action_event_reader.iter() {
        if matches!(action_event.value, ActionInput::Exit) {
            sfx.play(sounds.sfx_push_box.clone());
        }
    }
}

fn update_dynamic_text(sounds: Res<Sounds>, mut texts: Query<(&mut Text, &DynamicTextData)>) {
    for (mut text, data) in texts.iter_mut() {
        text.sections[1].value = match data.id {
            VOLUME_ID => format!("<{:>4.0}%>", sounds.get_volume() * 100.0),
            _ => unreachable!("The text id does not exists"),
        };
    }
}
