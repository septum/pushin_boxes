mod handles;
mod map;
mod snapshots;
mod state;
mod tag;

pub mod prelude {
    pub use super::handles::LevelHandles;
    pub use super::map::{MapEntity, MapPosition};
    pub use super::state::LevelState;
    pub use super::tag::LevelTag;
    pub use super::{CharacterMarker, Level, LevelAnimation, LevelInsertionEvent};
}

use std::time::Duration;

use bevy::{
    app::Plugin as BevyPlugin,
    prelude::*,
    time::{Stopwatch, Timer},
};
use iyes_loopless::prelude::*;
use serde::{Deserialize, Serialize};

use snapshots::{LevelSnapshots, MAX_SNAPSHOTS};

use super::prelude::*;

pub const TOTAL_STOCK_LEVELS: usize = 16;

pub const SPRITE_SIZE: usize = 64;
pub const SPRITE_OFFSET: usize = 32;

pub const ENTITY_SURFACE: usize = 36;
pub const ENTITY_EDGE: usize = 28;
pub const ENTITY_SURFACE_OFFSET: usize = 18;
pub const ENTITY_ON_TOP_OFFSET: usize = 28;
pub const BOX_ENTITY_OFFSET: usize = 14;

pub const MAP_WIDTH: f32 = 640.0;
pub const MAP_HEIGHT: f32 = 388.0;
pub const MAP_COLS: usize = 10;
pub const MAP_ROWS: usize = 10;

#[derive(Component)]
pub struct CharacterMarker;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(insert_level.run_on_event::<LevelInsertionEvent>());
    }
}

pub fn insert_level(
    mut commands: Commands,
    mut level_insertion_event_reader: EventReader<LevelInsertionEvent>,
    mut scene_transition_event_writer: EventWriter<SceneTransitionEvent>,
    save_file: Res<SaveFile>,
    level_handles: Res<LevelHandles>,
    level_states_assets: Res<Assets<LevelState>>,
) {
    if let Some(level_insertion_event) = level_insertion_event_reader.iter().next() {
        let LevelTag::Stock(index) = &level_insertion_event.tag;
        let state = *level_states_assets
            .get(&level_handles.stock[*index])
            .unwrap();
        let record = save_file.get_record(&level_insertion_event.tag);
        let level = Level::new(level_insertion_event.tag.clone(), state, record);

        commands.insert_resource(level);
        scene_transition_event_writer.send(SceneTransitionEvent::level());
    }
}

pub struct LevelInsertionEvent {
    pub tag: LevelTag,
}

impl LevelInsertionEvent {
    pub fn new(tag: LevelTag) -> LevelInsertionEvent {
        LevelInsertionEvent { tag }
    }
}

pub struct LevelDone {
    pub timer: Timer,
    pub flag: bool,
}

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct LevelRecord {
    pub moves: usize,
    pub time: f32,
}

impl LevelRecord {
    pub fn new(moves: usize, time: f32) -> LevelRecord {
        LevelRecord { moves, time }
    }

    pub fn is_set(&self) -> bool {
        self.moves > 0
    }

    pub fn time_string(&self) -> String {
        let duration = Duration::from_secs_f32(self.time);
        let milliseconds = duration.subsec_millis();
        let seconds = duration.as_secs() % 60;
        let minutes = (duration.as_secs() / 60) % 60;
        format!("{minutes:02}:{seconds:02}:{milliseconds:03}")
    }

    pub fn moves_string(&self) -> String {
        self.moves.to_string()
    }

    pub fn moves_in_time(&self, separator: &str) -> String {
        let moves = self.moves_string();
        let time = self.time_string();
        format!("{moves} moves{separator}in {time}")
    }

    pub fn is_better_than(&self, other: &LevelRecord) -> bool {
        self.moves == 0
            || self.moves > other.moves
            || self.moves >= other.moves && self.time > other.time
    }
}

pub struct LevelAnimation {
    pub animation_timer: Timer,
    pub idle_timer: Timer,
    pub long_idle_timer: Timer,
    pub row: usize,
    pub index: usize,
}

impl LevelAnimation {
    pub fn reset(&mut self) {
        self.animation_timer.reset();
        self.idle_timer.reset();
        self.long_idle_timer.reset();
    }

    fn tick(&mut self, delta: Duration) {
        self.animation_timer.tick(delta);
        self.idle_timer.tick(delta);
        self.long_idle_timer.tick(delta);
    }

    pub fn update_sprite_index(&mut self, delta: Duration, level_sprite_index: usize) -> usize {
        self.tick(delta);

        if level_sprite_index == 0 {
            if self.idle_timer.just_finished() {
                self.row = 4;
                self.index = 0;
                self.animation_timer.reset();
            }

            if self.long_idle_timer.just_finished() {
                self.row = 5;
                self.index = 0;
                self.animation_timer.reset();
            }
        } else {
            self.idle_timer.reset();
            self.long_idle_timer.reset();
        }

        if self.row != level_sprite_index
            && !self.idle_timer.finished()
            && !self.long_idle_timer.finished()
        {
            self.row = level_sprite_index;
            self.index = 0;
            self.animation_timer.reset();
        }

        if self.animation_timer.just_finished() {
            self.index = (self.index + 1) % 4;
        }

        self.index + (4 * self.row)
    }
}

pub struct Level {
    pub tag: LevelTag,
    pub state: LevelState,
    pub record: LevelRecord,
    pub snapshots: LevelSnapshots,
    pub undos: usize,
    pub moves: usize,
    pub timer: Timer,
    pub stopwatch: Stopwatch,
    pub animation: LevelAnimation,
}

impl Default for Level {
    fn default() -> Level {
        Level::new(LevelTag::Stock(0), default(), default())
    }
}

impl Level {
    pub fn new(tag: LevelTag, state: LevelState, record: LevelRecord) -> Level {
        Level {
            tag,
            state,
            record,
            snapshots: [None; MAX_SNAPSHOTS],
            undos: 4,
            moves: 0,
            timer: Timer::from_seconds(0.25, false),
            stopwatch: Stopwatch::new(),
            animation: LevelAnimation {
                animation_timer: Timer::from_seconds(0.25, true),
                idle_timer: Timer::from_seconds(7.0, false),
                long_idle_timer: Timer::from_seconds(10.0, false),
                row: 0,
                index: 0,
            },
        }
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

    pub fn player_in(&mut self, position: &MapPosition) -> bool {
        self.state.player_position.equals(position)
    }

    pub fn get_player_position(&self) -> &MapPosition {
        &self.state.player_position
    }

    pub fn get_sprite_index(&self) -> usize {
        self.state.sprite_index
    }

    // TODO: This is the sprite _row_
    pub fn set_sprite_index_with(&mut self, direction: &DirectionInput) {
        match direction {
            DirectionInput::Down => self.state.sprite_index = 0,
            DirectionInput::Up => self.state.sprite_index = 1,
            DirectionInput::Left => self.state.sprite_index = 2,
            DirectionInput::Right => self.state.sprite_index = 3,
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

    pub fn move_player(&mut self, position: MapPosition) {
        self.state.player_position = position;
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

    pub fn is_record_set(&self) -> bool {
        self.record.moves > 0
    }

    pub fn new_record(&self) -> bool {
        self.record.moves == 0
            || self.moves < self.record.moves
            || self.moves <= self.record.moves
                && self.stopwatch.elapsed().as_secs_f32() < self.record.time
    }

    pub fn get_record_set(&self) -> LevelRecord {
        LevelRecord::new(self.moves, self.stopwatch.elapsed().as_secs_f32())
    }

    pub fn is_stock(&self) -> bool {
        matches!(self.tag, LevelTag::Stock(_))
    }

    pub fn name(&self) -> String {
        match self.tag {
            LevelTag::Stock(index) => (index + 1).to_string(),
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
        self.timer.tick(delta);
    }

    pub fn tick_stopwatch(&mut self, delta: Duration) {
        self.stopwatch.tick(delta);
    }

    pub fn stopwatch_elapsed(&self) -> Duration {
        self.stopwatch.elapsed()
    }

    pub fn timer_finished(&self) -> bool {
        self.timer.finished()
    }

    pub fn stopwatch_string(&self) -> String {
        let duration = self.stopwatch_elapsed();
        let milliseconds = duration.subsec_millis();
        let seconds = duration.as_secs() % 60;
        let minutes = (duration.as_secs() / 60) % 60;
        format!("{minutes:02}:{seconds:02}:{milliseconds:03}")
    }

    pub fn moves_in_time(&self, separator: &str) -> String {
        let moves = self.moves.to_string();
        let time = self.stopwatch_string();
        format!("{moves} moves{separator}in {time}")
    }

    pub fn reload(
        &mut self,
        levels: &LevelHandles,
        level_states_assets: &Assets<LevelState>,
    ) -> bool {
        if self.moves != 0 || self.undos < 4 {
            self.set_state(match self.tag {
                LevelTag::Stock(index) => *level_states_assets.get(&levels.stock[index]).unwrap(),
            });
            true
        } else {
            false
        }
    }

    pub fn spawn(&mut self, commands: &mut Commands, images: &Images) {
        self.animation.reset();

        let position = self.get_player_position();
        let index = self.get_sprite_index();

        position.spawn_player(commands, images.player_atlas.clone(), index);

        self.loop_over_entity_and_position(|entity, position| {
            if let Some(texture) = entity.to_image(images) {
                let on_top = matches!(entity, &MapEntity::B | &MapEntity::P);
                position.spawn_entity(commands, texture, on_top);
            }
        });
    }

    pub fn is_last(&self) -> bool {
        match self.tag {
            LevelTag::Stock(index) => index + 1 == TOTAL_STOCK_LEVELS,
        }
    }
}
