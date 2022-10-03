mod ui;

use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_kira_audio::{AudioChannel, AudioControl};
use iyes_loopless::prelude::*;

use crate::{
    resources::{level::BOX_ENTITY_OFFSET, prelude::*},
    ui::{DynamicTextMarker, OverlayMarker},
};

const STOPWATCH_COUNTER_NAME: &str = "stopwatch";
const MOVES_COUNTER_NAME: &str = "moves";
const UNDOS_COUNTER_NAME: &str = "undos";

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            GameState::Level,
            SystemSet::new()
                .with_system(self::ui::spawn)
                .with_system(spawn_level),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Level)
                .with_system(handle_direction_input.run_on_event::<DirectionEvent>())
                .with_system(handle_action_input.run_on_event::<ActionEvent>())
                .with_system(update_character_position)
                .with_system(update_character_sprite)
                .with_system(update_counters)
                .with_system(update_map)
                .with_system(update_level_state)
                .with_system(check_lever_timer_finished)
                .into(),
        )
        .add_exit_system_set(
            GameState::Level,
            SystemSet::new()
                .with_system(cleanup::<OverlayMarker>)
                .with_system(cleanup::<CharacterMarker>)
                .with_system(cleanup::<MapPosition>),
        );
    }
}

fn spawn_level(mut commands: Commands, mut level: ResMut<Level>, images: Res<Images>) {
    level.spawn(&mut commands, &images);
}

fn handle_action_input(
    mut game_state_event_writer: EventWriter<SceneTransitionEvent>,
    mut action_event_reader: EventReader<ActionEvent>,
    mut level: ResMut<Level>,
    levels: Res<LevelHandles>,
    level_states: Res<Assets<LevelState>>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    for action_event in action_event_reader.iter() {
        match &action_event.value {
            Action::Undo => {
                if level.undo() {
                    sfx.play(sounds.sfx_undo_move.clone());
                }
            }
            Action::Reload => {
                if level.reload(&levels, &level_states) {
                    sfx.play(sounds.sfx_reload_level.clone());
                }
            }
            Action::Selection | Action::Exit => {
                game_state_event_writer.send(SceneTransitionEvent::selection());
            }
            _ => (),
        }
    }
}

fn handle_direction_input(
    mut direction_event_reader: EventReader<DirectionEvent>,
    mut level: ResMut<Level>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    for direction_event in direction_event_reader.iter() {
        let direction = &direction_event.value;
        level.set_sprite_index_with(direction);

        let mut next_position = level.state.player_position;
        next_position.update_position(direction);

        let next_entity = level.get_entity(&next_position);
        match next_entity {
            MapEntity::B | MapEntity::P => {
                let in_zone = matches!(next_entity, MapEntity::P);
                let updated_next_entity = if in_zone { MapEntity::Z } else { MapEntity::F };

                let mut adjacent_position = next_position;
                adjacent_position.update_position(direction);

                let adjacent_entity = level.get_entity(&adjacent_position);
                match adjacent_entity {
                    MapEntity::F => {
                        sfx.play(sounds.sfx_move_player.clone());
                        sfx.play(sounds.sfx_push_box.clone());

                        level.save_snapshot();
                        level.set_entity(&next_position, updated_next_entity);
                        level.set_entity(&adjacent_position, MapEntity::B);
                        level.move_player(next_position);
                        level.increment_moves();

                        if in_zone {
                            level.increment_remaining_zones();
                        }
                    }
                    MapEntity::Z => {
                        sfx.play(sounds.sfx_move_player.clone());
                        sfx.play(sounds.sfx_push_box.clone());
                        sfx.play(sounds.sfx_set_zone.clone());

                        level.save_snapshot();
                        level.set_entity(&next_position, updated_next_entity);
                        level.set_entity(&adjacent_position, MapEntity::P);
                        level.move_player(next_position);
                        level.increment_moves();

                        if !in_zone {
                            level.decrement_remaining_zones();
                        }
                    }
                    _ => (),
                }
            }
            MapEntity::W => {}
            _ => {
                level.save_snapshot();
                level.move_player(next_position);
                level.increment_moves();
                sfx.play(sounds.sfx_move_player.clone());
            }
        }
    }
}

fn update_character_position(
    level: Res<Level>,
    mut query: Query<&mut Transform, With<CharacterMarker>>,
) {
    let mut transform = query.single_mut();
    level
        .state
        .player_position
        .update_player_translation(&mut transform.translation);
}

fn update_character_sprite(
    time: Res<Time>,
    mut level: ResMut<Level>,
    mut query: Query<&mut TextureAtlasSprite, With<CharacterMarker>>,
) {
    let mut sprite = query.single_mut();
    let level_sprite_index = level.get_sprite_index();
    sprite.index = level
        .animation
        .update_sprite_index(time.delta(), level_sprite_index);
}

fn update_counters(
    level: Res<Level>,
    mut texts: Query<(&mut Text, &DynamicTextMarker), With<DynamicTextMarker>>,
) {
    for (mut text, marker) in texts.iter_mut() {
        text.sections[1].value = match marker.name.as_str() {
            MOVES_COUNTER_NAME => level.moves.to_string(),
            UNDOS_COUNTER_NAME => level.undos.to_string(),
            STOPWATCH_COUNTER_NAME => level.stopwatch_string(),
            _ => unreachable!("The marker name does not exists"),
        };
    }
}

fn update_map(
    level: Res<Level>,
    images: Res<Images>,
    mut query: Query<(&mut Handle<Image>, &mut Transform, &MapPosition)>,
) {
    for (mut image, mut transform, position) in query.iter_mut() {
        let map_entity = level.get_entity(position);

        if let Some(entity_image) = map_entity.to_image(&images) {
            *image = entity_image;
        }

        position.update_entity_translation(&mut transform.translation);

        if matches!(map_entity, MapEntity::B | MapEntity::P) {
            transform.translation.y += BOX_ENTITY_OFFSET as f32;
            transform.translation.z += 1.0;
        }
    }
}

fn update_level_state(time: Res<Time>, mut level: ResMut<Level>) {
    let delta = time.delta();
    if level.no_remaining_zones() {
        level.tick_timer(delta);
    } else {
        level.tick_stopwatch(delta);
    }
}

fn check_lever_timer_finished(
    mut scene_transition_event_writer: EventWriter<SceneTransitionEvent>,
    level: Res<Level>,
) {
    if level.timer_finished() {
        scene_transition_event_writer.send(SceneTransitionEvent::win());
    }
}
