mod event;
mod handles;
mod kind;
mod map;
mod record;
mod snapshots;
mod state;

pub mod prelude {
    pub use super::event::LevelInsertionEvent;
    pub use super::handles::LevelHandles;
    pub use super::kind::LevelKind;
    pub use super::map::{MapEntity, MapPosition};
    pub use super::record::LevelRecord;
    pub use super::state::LevelState;
    pub use super::{Level, TOTAL_CUSTOM_LEVELS, TOTAL_STOCK_LEVELS};
}

use std::time::Duration;

use bevy::{
    app::Plugin as BevyPlugin,
    prelude::*,
    time::{Stopwatch, Timer},
};
use iyes_loopless::prelude::*;
use uuid::Uuid;

use self::{
    map::{MAP_COLS, MAP_ROWS},
    snapshots::{LevelSnapshots, MAX_SNAPSHOTS},
};

use super::prelude::*;

pub const TOTAL_STOCK_LEVELS: usize = 16;
pub const TOTAL_CUSTOM_LEVELS: usize = 16;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(insert_level.run_on_event::<LevelInsertionEvent>());
    }
}

pub fn insert_custom_level_handles(
    save_file: Res<SaveFile>,
    mut level_handles: ResMut<LevelHandles>,
    asset_server: Res<AssetServer>,
) {
    for (_, (key, _)) in save_file.ordered_custom_records() {
        let split_key: Vec<&str> = key.split('$').collect();
        let uuid = Uuid::parse_str(split_key[1]).expect("Cannot parse uuid");
        let path = format!("levels/custom/{}.lvl", &split_key[1]);
        level_handles.insert_custom(uuid, asset_server.load(path));
    }
}

fn insert_level(
    mut commands: Commands,
    mut level_insertion_event_reader: EventReader<LevelInsertionEvent>,
    mut scene_transition_event_writer: EventWriter<SceneTransitionEvent>,
    save_file: Res<SaveFile>,
    level_handles: Res<LevelHandles>,
    level_states_assets: Res<Assets<LevelState>>,
) {
    if let Some(level_insertion_event) = level_insertion_event_reader.iter().next() {
        match &level_insertion_event.kind {
            LevelKind::Stock(index) => {
                let state = *level_states_assets
                    .get(level_handles.get_stock(index))
                    .unwrap();
                let record = save_file.get_record(&level_insertion_event.kind);
                let level = Level::new(level_insertion_event.kind.clone(), state, record);

                commands.insert_resource(level);
                scene_transition_event_writer.send(SceneTransitionEvent::level());
            }
            LevelKind::Custom(payload) => {
                let parsed_payload: Vec<&str> = payload.split('$').collect();
                let uuid = Uuid::parse_str(parsed_payload[1]).expect("Cannot parse uuid");
                let state = *level_states_assets
                    .get(level_handles.get_custom(&uuid))
                    .unwrap();
                let record = save_file.get_record(&level_insertion_event.kind);
                let level = Level::new(level_insertion_event.kind.clone(), state, record);

                commands.insert_resource(level);
                scene_transition_event_writer.send(SceneTransitionEvent::level());
            }
            LevelKind::Playtest(state) => {
                let level = Level::new(
                    level_insertion_event.kind.clone(),
                    *state,
                    LevelRecord::default(),
                );

                commands.insert_resource(level);
                scene_transition_event_writer.send(SceneTransitionEvent::level());
            }
            LevelKind::Editable => {
                unreachable!("An editable should not be inserted with this event")
            }
        }
    }
}

#[derive(Default)]
pub struct LevelDone {
    pub timer: Timer,
    pub value: bool,
}

#[derive(Default, Resource)]
pub struct Level {
    pub kind: LevelKind,
    state: LevelState,
    record: LevelRecord,
    snapshots: LevelSnapshots,
    undos: usize,
    moves: usize,
    stopwatch: Stopwatch,
    done: LevelDone,
}

impl Level {
    pub fn new(kind: LevelKind, state: LevelState, record: LevelRecord) -> Level {
        Level {
            kind,
            state,
            record,
            snapshots: [None; MAX_SNAPSHOTS],
            undos: 4,
            moves: 0,
            stopwatch: Stopwatch::new(),
            done: LevelDone {
                timer: Timer::from_seconds(0.25, TimerMode::Once),
                value: false,
            },
        }
    }

    pub fn editable() -> Level {
        let state = LevelState::editor();
        let kind = LevelKind::Editable;
        let record = LevelRecord::default();
        Level::new(kind, state, record)
    }

    pub fn clone_state(&self) -> LevelState {
        self.state
    }

    pub fn new_record(&self) -> bool {
        self.get_set_record().is_better_than(&self.record)
    }

    pub fn set_state(&mut self, state: LevelState) {
        self.snapshots = [None; MAX_SNAPSHOTS];
        self.state = state;
        self.undos = 4;
        self.moves = 0;
    }

    pub fn loop_over_entity_and_position<F>(&self, mut f: F)
    where
        F: FnMut(&MapEntity, MapPosition),
    {
        for column in 0..MAP_COLS {
            for row in 0..MAP_ROWS {
                let position = MapPosition::new(column, row);
                let entity = self.get_entity(&position);
                f(entity, position);
            }
        }
    }

    pub fn get_entity(&self, position: &MapPosition) -> &MapEntity {
        &self.state.map[position.y][position.x]
    }

    pub fn set_entity(&mut self, position: &MapPosition, entity: MapEntity) {
        self.state.map[position.y][position.x] = entity;
    }

    pub fn character_in(&mut self, position: &MapPosition) -> bool {
        self.state.character_position.equals(position)
    }

    pub fn get_character_position(&self) -> &MapPosition {
        &self.state.character_position
    }

    pub fn get_animation_row(&self) -> usize {
        self.state.animation_row
    }

    pub fn set_animation_row_with(&mut self, direction: &DirectionInput) {
        match direction {
            DirectionInput::Down => self.state.animation_row = 0,
            DirectionInput::Up => self.state.animation_row = 1,
            DirectionInput::Left => self.state.animation_row = 2,
            DirectionInput::Right => self.state.animation_row = 3,
        }
    }

    pub fn increment_moves(&mut self) {
        self.moves += 1;
    }

    pub fn decrement_moves(&mut self) {
        self.moves = self.moves.saturating_sub(1);
    }

    pub fn decrement_undos(&mut self) {
        self.undos = self.undos.saturating_sub(1);
    }

    pub fn move_character(&mut self, position: MapPosition) {
        self.state.character_position = position;
    }

    pub fn increment_remaining_zones(&mut self) {
        self.state.remaining_zones += 1;
    }

    pub fn decrement_remaining_zones(&mut self) {
        self.state.remaining_zones -= 1;
    }

    pub fn no_remaining_zones(&self) -> bool {
        self.state.remaining_zones == 0
    }

    pub fn get_current_record(&self) -> LevelRecord {
        self.record
    }

    pub fn get_set_record(&self) -> LevelRecord {
        LevelRecord::new(self.moves, self.stopwatch.elapsed().as_secs_f32())
    }

    pub fn is_stock(&self) -> bool {
        matches!(self.kind, LevelKind::Stock(_))
    }

    pub fn name(&self) -> String {
        match &self.kind {
            LevelKind::Stock(index) => (index + 1).to_string(),
            LevelKind::Custom(key) => {
                let parsed_key: Vec<&str> = key.split('$').collect();
                parsed_key[0].to_string()
            }
            LevelKind::Playtest(_) => "Playtest".to_string(),
            LevelKind::Editable => unreachable!("An editable level does not have a name"),
        }
    }

    pub fn save_snapshot(&mut self) {
        self.snapshots.rotate_right(1);
        self.snapshots[0] = Some(self.state);
    }

    pub fn undo(&mut self) -> bool {
        if self.undos > 0 {
            if let Some(state) = self.snapshots[0] {
                self.state = state;
                self.snapshots.rotate_left(1);
                self.snapshots[MAX_SNAPSHOTS - 1] = None;
                self.decrement_undos();
                self.decrement_moves();
                return true;
            }
            return false;
        }
        false
    }

    pub fn tick_timer(&mut self, delta: Duration) {
        self.done.timer.tick(delta);
    }

    pub fn tick_stopwatch(&mut self, delta: Duration) {
        self.stopwatch.tick(delta);
    }

    pub fn stopwatch_elapsed(&self) -> Duration {
        self.stopwatch.elapsed()
    }

    pub fn timer_just_finished(&self) -> bool {
        self.done.timer.finished()
    }

    pub fn stopwatch_string(&self) -> String {
        let duration = self.stopwatch_elapsed();
        let milliseconds = duration.subsec_millis();
        let seconds = duration.as_secs() % 60;
        let minutes = (duration.as_secs() / 60) % 60;
        format!("{minutes:02}:{seconds:02}:{milliseconds:03}")
    }

    pub fn moves_string(&self) -> String {
        self.moves.to_string()
    }

    pub fn character_position(&self) -> MapPosition {
        self.state.character_position
    }

    pub fn undos_string(&self) -> String {
        self.undos.to_string()
    }

    pub fn moves_in_time(&self, separator: &str) -> String {
        let moves = self.moves.to_string();
        let time = self.stopwatch_string();
        format!("{moves} moves{separator}in {time}")
    }

    pub fn reload(
        &mut self,
        level_handles: &LevelHandles,
        level_states_assets: &Assets<LevelState>,
    ) -> bool {
        if self.moves != 0 || self.undos < 4 {
            match &self.kind {
                LevelKind::Stock(index) => {
                    self.set_state(
                        *level_states_assets
                            .get(level_handles.get_stock(index))
                            .unwrap(),
                    );
                }
                LevelKind::Custom(key) => {
                    let parsed_key: Vec<&str> = key.split('$').collect();
                    let uuid = Uuid::parse_str(parsed_key[1]).expect("Cannot parse uuid");
                    self.set_state(
                        *level_states_assets
                            .get(level_handles.get_custom(&uuid))
                            .unwrap(),
                    );
                }
                LevelKind::Playtest(state) => {
                    self.set_state(*state);
                }
                LevelKind::Editable => {
                    unreachable!("An editable level can't be reloaded")
                }
            }
            true
        } else {
            false
        }
    }

    pub fn spawn(&mut self, commands: &mut Commands, images: &Images) {
        let position = self.get_character_position();
        let level_animation_row = self.get_animation_row();

        position.spawn_character(
            commands,
            images.character_atlas.clone(),
            level_animation_row,
        );

        self.loop_over_entity_and_position(|entity, position| {
            let texture = entity.to_image(images);
            position.spawn_entity(commands, texture);
        });
    }

    pub fn is_last(&self) -> bool {
        match self.kind {
            LevelKind::Stock(index) => index + 1 == TOTAL_STOCK_LEVELS,
            _ => unreachable!("There is no last level in other level kinds"),
        }
    }
}
