use std::time::Duration;

use bevy::prelude::*;

use crate::{assets::prelude::Images, level::LevelResource};

use super::Character;

const BLINK_ROW: usize = 4;
const SLEEP_ROW: usize = 5;
const FRONT_ROW: usize = 0;
const BLINK_ROW_LAST_FRAME_INDEX: usize = 19;
const WIN_CHARACTER_TRANSFORM: Transform =
    Transform::from_translation(Vec3::new(222.0, -16.0, 1.0));

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
        let transform: Transform;

        #[cfg(not(target_family = "wasm"))]
        {
            transform = Transform::from_translation(Vec3::new(0.0, 54.0, 1.0));
        }

        #[cfg(target_family = "wasm")]
        {
            transform = Transform::from_translation(Vec3::new(0.0, 2.0, 1.0));
        }

        commands.insert_resource(character_animation);
        commands.spawn((sprite, transform)).insert(Character);
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
            .insert(Character);
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

    pub fn update_character_happy_animation(
        time: Res<Time>,
        mut character_animation: ResMut<CharacterAnimation>,
        mut query: Query<&mut Sprite, With<Character>>,
    ) {
        character_animation.tick(time.delta());
        if character_animation.primary_timer_just_finished() {
            let mut sprite = query.single_mut().unwrap();
            character_animation.next_index();
            sprite.texture_atlas.as_mut().unwrap().index = character_animation.sprite_index();
        }
    }

    pub fn update_blinking_character_animation(
        time: Res<Time>,
        mut query: Query<&mut Sprite, With<Character>>,
        mut character_animation: ResMut<CharacterAnimation>,
    ) {
        let mut sprite = query.single_mut().unwrap();

        character_animation.tick(time.delta());

        if character_animation.secondary_timer_just_finished() {
            character_animation.set_blink_row();
            character_animation.reset_primary_timer();
            character_animation.reset_secondary_timer();
        }

        if character_animation.primary_timer_just_finished() {
            if sprite.texture_atlas.as_mut().unwrap().index == BLINK_ROW_LAST_FRAME_INDEX {
                character_animation.set_front_row();
                character_animation.reset_primary_timer();
                character_animation.reset_secondary_timer();
            } else {
                character_animation.next_index();
            }
        }

        sprite.texture_atlas.as_mut().unwrap().index = character_animation.sprite_index();
    }

    pub fn update_level_character_animation(
        time: Res<Time>,
        level: Res<LevelResource>,
        mut character_animation: ResMut<CharacterAnimation>,
        mut query: Query<&mut Sprite, With<Character>>,
    ) {
        let mut sprite = query.single_mut().unwrap();
        let level_character_facing_direction = level.character_facing_direction();

        character_animation.tick(time.delta());

        if level_character_facing_direction == 0 {
            if character_animation.secondary_timer_just_finished() {
                character_animation.set_blink_row();
                character_animation.reset_primary_timer();
            }

            if character_animation.tertiary_timer_just_finished() {
                character_animation.set_sleep_row();
                character_animation.reset_primary_timer();
            }
        } else {
            character_animation.reset_secondary_timer();
            character_animation.reset_tertiary_timer();
        }

        if !character_animation.row_is(level_character_facing_direction)
            && !character_animation.secondary_timer_finished()
            && !character_animation.tertiary_timer_finished()
        {
            character_animation.reset_primary_timer();
            character_animation.reset_index();
            character_animation.set_row(level_character_facing_direction);
        }

        if character_animation.primary_timer_just_finished() {
            character_animation.next_index();
        }

        sprite.texture_atlas.as_mut().unwrap().index = character_animation.sprite_index();
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
