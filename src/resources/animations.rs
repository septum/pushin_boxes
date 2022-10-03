use std::time::Duration;

use bevy::prelude::*;

use super::prelude::*;

const BLINK_ROW: usize = 4;
const FRONT_ROW: usize = 0;

pub const BLINK_ROW_LAST_FRAME_INDEX: usize = 19;

const TITLE_CHARACTER_TRANSFORM: Transform = Transform::from_translation(Vec3::new(0.0, 24.0, 1.0));
const WIN_CHARACTER_TRANSFORM: Transform = Transform::from_translation(Vec3::new(220.0, 24.0, 1.0));

pub struct CharacterAnimation {
    primary_timer: Timer,
    secondary_timer: Timer,
    row: usize,
    index: usize,
}

impl CharacterAnimation {
    pub fn insert_title_character_animation(mut commands: Commands, images: Res<Images>) {
        let bundle = SpriteSheetBundle {
            texture_atlas: images.player_atlas.clone(),
            transform: TITLE_CHARACTER_TRANSFORM,
            ..default()
        };
        let character_animation = CharacterAnimation {
            primary_timer: Timer::from_seconds(0.25, true),
            secondary_timer: Timer::from_seconds(3.0, false),
            row: 0,
            index: 0,
        };
        commands.insert_resource(character_animation);
        commands.spawn_bundle(bundle).insert(CharacterMarker);
    }

    pub fn insert_win_character_animation(mut commands: Commands, images: Res<Images>) {
        let bundle = SpriteSheetBundle {
            texture_atlas: images.player_atlas.clone(),
            transform: WIN_CHARACTER_TRANSFORM,
            ..default()
        };
        let character_animation = CharacterAnimation {
            primary_timer: Timer::from_seconds(0.125, true),
            secondary_timer: Timer::default(),
            row: 6,
            index: 0,
        };
        commands.insert_resource(character_animation);
        commands.spawn_bundle(bundle).insert(CharacterMarker);
    }

    pub fn tick(&mut self, delta: Duration) {
        self.primary_timer.tick(delta);
        self.secondary_timer.tick(delta);
    }

    pub fn reset_with_blink_row(&mut self) {
        self.primary_timer.reset();
        self.secondary_timer.reset();
        self.index = 0;
        self.row = BLINK_ROW;
    }

    pub fn reset_with_front_row(&mut self) {
        self.primary_timer.reset();
        self.secondary_timer.reset();
        self.index = 0;
        self.row = FRONT_ROW;
    }

    pub fn tick_primary(&mut self, delta: Duration) -> bool {
        self.primary_timer.tick(delta).just_finished()
    }

    pub fn primary_timer_finished(&self) -> bool {
        self.primary_timer.just_finished()
    }

    pub fn secondary_timer_finished(&self) -> bool {
        self.secondary_timer.just_finished()
    }

    pub fn next_index(&mut self) {
        self.index = (self.index + 1) % 4;
    }

    pub fn sprite_index(&self) -> usize {
        self.index + (4 * self.row)
    }

    pub fn next_sprite_index(&mut self) -> usize {
        self.next_index();
        self.index + (4 * self.row)
    }
}
