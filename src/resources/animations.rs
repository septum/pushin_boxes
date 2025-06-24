use std::time::Duration;

use bevy::prelude::*;

use super::prelude::*;

#[derive(Component)]
pub struct CharacterMarker;

const BLINK_ROW: usize = 4;
const SLEEP_ROW: usize = 5;
const FRONT_ROW: usize = 0;

pub const BLINK_ROW_LAST_FRAME_INDEX: usize = 19;

const TITLE_CHARACTER_TRANSFORM: Transform = Transform::from_translation(Vec3::new(0.0, 22.0, 1.0));
const WIN_CHARACTER_TRANSFORM: Transform = Transform::from_translation(Vec3::new(222.0, 12.0, 1.0));

#[derive(Default, Resource)]
pub struct CharacterAnimation {
    primary_timer: Timer,
    secondary_timer: Timer,
    tertiary_timer: Timer,
    row: usize,
    index: usize,
}

impl CharacterAnimation {
    pub fn insert_blinking_character_animation(mut commands: Commands, images: Res<Images>) {
        let atlas = TextureAtlas {
            layout: images.character_layout.clone(),
            ..default()
        };
        let sprite = Sprite {
            image: images.character.clone(),
            texture_atlas: Some(atlas),
            ..default()
        };
        let character_animation = CharacterAnimation {
            primary_timer: Timer::from_seconds(0.25, TimerMode::Repeating),
            secondary_timer: Timer::from_seconds(3.0, TimerMode::Once),
            ..default()
        };

        commands.insert_resource(character_animation);
        commands
            .spawn((sprite, TITLE_CHARACTER_TRANSFORM))
            .insert(CharacterMarker);
    }

    pub fn insert_happy_character_animation(mut commands: Commands, images: Res<Images>) {
        let atlas = TextureAtlas {
            layout: images.character_layout.clone(),
            ..default()
        };
        let sprite = Sprite {
            image: images.character.clone(),
            texture_atlas: Some(atlas),
            ..default()
        };
        let character_animation = CharacterAnimation {
            primary_timer: Timer::from_seconds(0.125, TimerMode::Repeating),
            row: 6,
            ..default()
        };

        commands.insert_resource(character_animation);
        commands
            .spawn((sprite, WIN_CHARACTER_TRANSFORM))
            .insert(CharacterMarker);
    }

    pub fn insert_level_character_animation(mut commands: Commands) {
        let character_animation = CharacterAnimation {
            primary_timer: Timer::from_seconds(0.25, TimerMode::Repeating),
            secondary_timer: Timer::from_seconds(7.0, TimerMode::Once),
            tertiary_timer: Timer::from_seconds(10.0, TimerMode::Once),
            row: FRONT_ROW,
            index: 0,
        };
        commands.insert_resource(character_animation);
    }

    pub fn tick(&mut self, delta: Duration) {
        self.primary_timer.tick(delta);
        self.secondary_timer.tick(delta);
        self.tertiary_timer.tick(delta);
    }

    pub fn set_blink_row(&mut self) {
        self.reset_index();
        self.set_row(BLINK_ROW);
    }

    pub fn set_front_row(&mut self) {
        self.reset_index();
        self.set_row(FRONT_ROW);
    }

    pub fn set_sleep_row(&mut self) {
        self.reset_index();
        self.set_row(SLEEP_ROW);
    }

    pub fn row_is(&self, row: usize) -> bool {
        self.row == row
    }

    pub fn set_row(&mut self, row: usize) {
        self.row = row;
    }

    pub fn reset_secondary_timer(&mut self) {
        self.secondary_timer.reset();
    }

    pub fn reset_tertiary_timer(&mut self) {
        self.tertiary_timer.reset();
    }

    pub fn reset_primary_timer(&mut self) {
        self.primary_timer.reset();
    }

    pub fn reset_index(&mut self) {
        self.index = 0;
    }

    pub fn primary_timer_just_finished(&self) -> bool {
        self.primary_timer.just_finished()
    }

    pub fn secondary_timer_just_finished(&self) -> bool {
        self.secondary_timer.just_finished()
    }

    pub fn tertiary_timer_just_finished(&self) -> bool {
        self.tertiary_timer.just_finished()
    }

    pub fn primary_timer_finished(&self) -> bool {
        self.primary_timer.finished()
    }

    pub fn secondary_timer_finished(&self) -> bool {
        self.secondary_timer.finished()
    }

    pub fn tertiary_timer_finished(&self) -> bool {
        self.tertiary_timer.finished()
    }

    pub fn next_index(&mut self) {
        self.index = (self.index + 1) % 4;
    }

    pub fn sprite_index(&self) -> usize {
        self.index + (4 * self.row)
    }
}
