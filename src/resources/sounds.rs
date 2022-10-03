use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl, AudioSource};
use iyes_loopless::prelude::*;
use iyes_loopless::state::CurrentState;

use super::prelude::*;

pub struct Sfx;

pub struct Music;

#[derive(AssetCollection)]
pub struct Sounds {
    sfx_volume: f64,
    music_volume: f64,
    #[asset(path = "sounds/sfx/move_player.wav")]
    pub sfx_move_player: Handle<AudioSource>,
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
    pub fn reset_volume(&mut self) {
        self.music_volume = 1.0;
        self.sfx_volume = 1.0;
    }

    pub fn toggle_volume(&mut self) {
        if self.music_volume > 0.0 {
            self.music_volume -= 0.25;
            self.sfx_volume -= 0.25;
        } else {
            self.reset_volume();
        }
    }
}

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_if_resource_exists::<Sounds>()
                .with_system(play_music)
                .with_system(handle_volume_input.run_on_event::<ActionEvent>())
                .with_system(play_sfx.run_on_event::<ActionEvent>())
                .into(),
        );
    }
}

pub fn setup_volume(
    mut sounds: ResMut<Sounds>,
    sfx: ResMut<AudioChannel<Sfx>>,
    music: ResMut<AudioChannel<Music>>,
) {
    sounds.reset_volume();
    music.set_volume(sounds.music_volume);
    sfx.set_volume(sounds.sfx_volume);
}

pub fn handle_volume_input(
    mut action_event_reader: EventReader<ActionEvent>,
    mut sounds: ResMut<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
    music: Res<AudioChannel<Music>>,
) {
    for action_event in action_event_reader.iter() {
        if matches!(action_event.value, Action::Volume) {
            sounds.toggle_volume();
            music.set_volume(sounds.music_volume);
            sfx.set_volume(sounds.sfx_volume);
        }
    }
}

pub fn play_music(
    sounds: Res<Sounds>,
    music: Res<AudioChannel<Music>>,
    game_state: Res<CurrentState<GameState>>,
) {
    if game_state.is_changed() {
        music.stop();
        music
            .play(match game_state.0 {
                GameState::Title | GameState::Instructions => sounds.music_title.clone(),
                GameState::Selection => sounds.music_selection.clone(),
                GameState::Level => sounds.music_level.clone(),
                GameState::Win => sounds.music_win.clone(),
                GameState::Loading => return,
            })
            .looped();
    }
}

pub fn play_sfx(
    mut action_event_reader: EventReader<ActionEvent>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    for action_event in action_event_reader.iter() {
        match action_event.value {
            Action::Selection | Action::Exit => {
                sfx.play(sounds.sfx_push_box.clone());
            }
            Action::Volume => {
                sfx.play(sounds.sfx_toggle_volume.clone());
            }
            Action::Pick => {
                sfx.play(sounds.sfx_set_zone.clone());
            }
            _ => (),
        }
    }
}
