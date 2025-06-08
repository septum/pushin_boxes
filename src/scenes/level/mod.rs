mod ui;

use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{
    resources::prelude::*,
    ui::{DynamicTextData, OverlayMarker},
};

const STOPWATCH_COUNTER_ID: usize = 0;
const MOVES_COUNTER_ID: usize = 1;
const UNDOS_COUNTER_ID: usize = 2;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Level),
            (
                self::ui::spawn,
                spawn_level,
                CharacterAnimation::insert_level_character_animation,
            ),
        )
        .add_systems(
            Update,
            (
                handle_direction_input.run_if(on_event::<DirectionInputEvent>()),
                handle_action_input.run_if(on_event::<ActionInputEvent>()),
                update_character_sprite,
                update_character_position,
                update_counters,
                update_map,
                update_level_state,
                check_lever_timer_just_finished,
            )
                .chain()
                .run_if(in_state(GameState::Level)),
        )
        .add_systems(
            OnExit(GameState::Level),
            (
                cleanup::<OverlayMarker>,
                cleanup::<CharacterMarker>,
                cleanup::<MapPosition>,
            ),
        );
    }
}

fn spawn_level(mut commands: Commands, mut level: ResMut<Level>, images: Res<Images>) {
    level.spawn(&mut commands, &images);
}

fn handle_action_input(
    mut game_state_event_writer: EventWriter<SceneTransitionEvent>,
    mut action_event_reader: EventReader<ActionInputEvent>,
    mut level: ResMut<Level>,
    level_handles: Res<LevelHandles>,
    level_states_assets: Res<Assets<LevelState>>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    if level.no_remaining_zones() {
        return;
    };

    for action_event in action_event_reader.read() {
        match &action_event.value {
            ActionInput::Undo => {
                if level.undo() {
                    sfx.play(sounds.sfx_undo_move.clone());
                }
            }
            ActionInput::Reload => {
                if level.reload(&level_handles, &level_states_assets) {
                    sfx.play(sounds.sfx_reload_level.clone());
                }
            }
            ActionInput::Exit => {
                sfx.play(sounds.sfx_push_box.clone());
                game_state_event_writer.send(SceneTransitionEvent::selection(
                    if level.is_stock() {
                        SelectionKind::Stock
                    } else {
                        SelectionKind::Custom
                    },
                ));
            }
            _ => (),
        }
    }
}

fn handle_direction_input(
    mut direction_event_reader: EventReader<DirectionInputEvent>,
    mut level: ResMut<Level>,
    sounds: Res<Sounds>,
    sfx: Res<AudioChannel<Sfx>>,
) {
    if level.no_remaining_zones() {
        return;
    };

    for direction_event in direction_event_reader.read() {
        let direction = &direction_event.value;
        level.set_animation_row_with(direction);

        let mut next_position = level.character_position();
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
                        sfx.play(sounds.sfx_move_character.clone());
                        sfx.play(sounds.sfx_push_box.clone());

                        level.save_snapshot();
                        level.set_entity(&next_position, updated_next_entity);
                        level.set_entity(&adjacent_position, MapEntity::B);
                        level.move_character(next_position);
                        level.increment_moves();

                        if in_zone {
                            level.increment_remaining_zones();
                        }
                    }
                    MapEntity::Z => {
                        sfx.play(sounds.sfx_move_character.clone());
                        sfx.play(sounds.sfx_push_box.clone());
                        sfx.play(sounds.sfx_set_zone.clone());

                        level.save_snapshot();
                        level.set_entity(&next_position, updated_next_entity);
                        level.set_entity(&adjacent_position, MapEntity::P);
                        level.move_character(next_position);
                        level.increment_moves();

                        if !in_zone {
                            level.decrement_remaining_zones();
                        }
                    }
                    _ => (),
                }
            }
            MapEntity::V => {}
            _ => {
                level.save_snapshot();
                level.move_character(next_position);
                level.increment_moves();
                sfx.play(sounds.sfx_move_character.clone());
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
        .character_position()
        .update_translation(&mut transform.translation);
}

fn update_character_sprite(
    time: Res<Time>,
    level: Res<Level>,
    mut character_animation: ResMut<CharacterAnimation>,
    mut query: Query<&mut TextureAtlas, With<CharacterMarker>>,
) {
    let mut sprite = query.single_mut();
    let level_animation_row = level.get_animation_row();

    character_animation.tick(time.delta());

    if level_animation_row == 0 {
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

    if !character_animation.row_is(level_animation_row)
        && !character_animation.secondary_timer_finished()
        && !character_animation.tertiary_timer_finished()
    {
        character_animation.reset_primary_timer();
        character_animation.reset_index();
        character_animation.set_row(level_animation_row);
    }

    if character_animation.primary_timer_just_finished() {
        character_animation.next_index();
    }

    sprite.index = character_animation.sprite_index();
}

fn update_counters(level: Res<Level>, mut texts: Query<(&mut Text, &DynamicTextData)>) {
    for (mut text, data) in texts.iter_mut() {
        text.sections[1].value = match data.id {
            MOVES_COUNTER_ID => level.moves_string(),
            UNDOS_COUNTER_ID => level.undos_string(),
            STOPWATCH_COUNTER_ID => level.stopwatch_string(),
            _ => unreachable!("The counter id does not exists"),
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
        *image = map_entity.to_image(&images);
        position.update_translation(&mut transform.translation);
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

fn check_lever_timer_just_finished(
    mut scene_transition_event_writer: EventWriter<SceneTransitionEvent>,
    level: Res<Level>,
) {
    if level.timer_just_finished() {
        match level.kind {
            LevelKind::Stock(_) | LevelKind::Custom(_) => {
                scene_transition_event_writer.send(SceneTransitionEvent::win());
            }
            LevelKind::Playtest(_) => {
                scene_transition_event_writer.send(SceneTransitionEvent::passed());
            }
            LevelKind::Editable => unreachable!("An editable level cannot trigger the level timer"),
        }
    }
}
