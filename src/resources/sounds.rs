use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl, AudioSource};
use iyes_loopless::prelude::*;
use iyes_loopless::state::CurrentState;

use super::prelude::*;

pub const INITIAL_VOLUME: f64 = 0.5;

pub struct Sfx;

pub struct Music;

#[derive(AssetCollection)]
pub struct Sounds {
    volume: f64,
    #[asset(path = "sounds/sfx/move_character.wav")]
    pub sfx_move_character: Handle<AudioSource>,
    #[asset(path = "sounds/sfx/push_box.wav")]
    pub sfx_push_box: Handle<AudioSource>,
    #[asset(path = "sounds/sfx/set_zone.wav")]
    pub sfx_set_zone: Handle<AudioSource>,
    #[asset(path = "sounds/sfx/toggle_volume.wav")]
    pub sfx_toggle_volume: Handle<AudioSource>,
    #[asset(path = "sounds/sfx/undo_move.wav")]
    pub sfx_undo_move: Handle<AudioSource>,
    #[asset(path = "sounds/sfx/reload_level.wav")]
    pub sfx_reload_level: Handle<AudioSource>,
    #[asset(path = "sounds/music/level.wav")]
    pub music_level: Handle<AudioSource>,
    #[asset(path = "sounds/music/selection.wav")]
    pub music_selection: Handle<AudioSource>,
    #[asset(path = "sounds/music/title.wav")]
    pub music_title: Handle<AudioSource>,
    #[asset(path = "sounds/music/win.wav")]
    pub music_win: Handle<AudioSource>,
}

impl Sounds {
    pub fn get_volume(&self) -> f64 {
        self.volume
    }

    pub fn decrease_volume(&mut self) {
        if self.volume > 0.0 {
            self.volume -= 0.25;
        } else {
            self.volume = 1.0;
        }
    }

    pub fn increase_volume(&mut self) {
        if self.volume < 1.0 {
            self.volume += 0.25;
        } else {
            self.volume = 0.0;
        }
    }
}

pub fn setup(
    mut sounds: ResMut<Sounds>,
    save_file: Res<SaveFile>,
    sfx: ResMut<AudioChannel<Sfx>>,
    music: ResMut<AudioChannel<Music>>,
) {
    sounds.volume = save_file.get_volume();
    music.set_volume(sounds.get_volume());
    sfx.set_volume(sounds.get_volume());
}

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_if_resource_exists::<Sounds>()
                .with_system(handle_volume_change)
                .with_system(play_music)
                .with_system(play_sfx.run_on_event::<ActionInputEvent>())
                .into(),
        );
    }
}

fn handle_volume_change(
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
    music: Res<AudioChannel<Music>>,
) {
    if sounds.is_changed() {
        music.set_volume(sounds.volume);
        sfx.set_volume(sounds.volume);
    }
}

fn play_music(
    sounds: Res<Sounds>,
    music: Res<AudioChannel<Music>>,
    game_state: Res<CurrentState<GameState>>,
) {
    if game_state.is_changed() {
        music.stop();
        music
            .play(match game_state.0 {
                GameState::Title | GameState::Instructions => sounds.music_title.clone(),
                GameState::Selection(_) | GameState::Options => sounds.music_selection.clone(),
                GameState::Level | GameState::Editor => sounds.music_level.clone(),
                GameState::Win | GameState::Passed => sounds.music_win.clone(),
                GameState::Loading => return,
            })
            .looped();
    }
}

fn play_sfx(
    mut action_event_reader: EventReader<ActionInputEvent>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    for action_event in action_event_reader.iter() {
        match action_event.value {
            ActionInput::Selection | ActionInput::Exit => {
                sfx.play(sounds.sfx_push_box.clone());
            }
            ActionInput::Pick => {
                sfx.play(sounds.sfx_set_zone.clone());
            }
            _ => (),
        }
    }
}
