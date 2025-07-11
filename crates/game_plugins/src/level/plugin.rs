use bevy::{app::Plugin as BevyPlugin, prelude::*};
use bevy_common_assets::ron::RonAssetPlugin;
use game_core::level::LevelKind;
use uuid::Uuid;

use crate::level::handles::{LevelHandles, LevelStateAsset};
use crate::level::insertion::LevelInsertionEvent;
use crate::level::resource::LevelResource;
use crate::state::GameStateTransitionEvent;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RonAssetPlugin::<LevelStateAsset>::new(&["lvl"]),))
            .add_systems(Update, insert_level.run_if(on_event::<LevelInsertionEvent>));
    }
}

fn insert_level(
    mut commands: Commands,
    mut level_insertion_event_reader: EventReader<LevelInsertionEvent>,
    mut scene_transition_event_writer: EventWriter<GameStateTransitionEvent>,
    level_handles: Res<LevelHandles>,
    level_states_assets: Res<Assets<LevelStateAsset>>,
) {
    if let Some(level_insertion_event) = level_insertion_event_reader.read().next() {
        match level_insertion_event.kind() {
            LevelKind::Stock(index) => {
                let state = **level_states_assets
                    .get(level_handles.get_stock(*index))
                    .unwrap();
                let level = LevelResource::new(level_insertion_event.kind().clone(), state);

                commands.insert_resource(level);
                scene_transition_event_writer.write(GameStateTransitionEvent::level());
            }
            LevelKind::Custom(payload) => {
                let parsed_payload: Vec<&str> = payload.split('$').collect();
                let uuid = Uuid::parse_str(parsed_payload[1]).expect("Cannot parse uuid");
                let state = **level_states_assets
                    .get(level_handles.get_custom(&uuid).unwrap())
                    .unwrap();
                let level = LevelResource::new(level_insertion_event.kind().clone(), state);

                commands.insert_resource(level);
                scene_transition_event_writer.write(GameStateTransitionEvent::level());
            }
            LevelKind::Editable(state) => {
                let level = LevelResource::new(level_insertion_event.kind().clone(), *state);

                commands.insert_resource(level);
                scene_transition_event_writer.write(GameStateTransitionEvent::level());
            }
        }
    }
}
