use bevy::{app::Plugin as BevyPlugin, prelude::*};
use uuid::Uuid;

use crate::resources::prelude::*;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, insert_level.run_if(on_event::<LevelInsertionEvent>));
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
    if let Some(level_insertion_event) = level_insertion_event_reader.read().next() {
        match &level_insertion_event.kind {
            LevelKind::Stock(index) => {
                let state = *level_states_assets
                    .get(level_handles.get_stock(index))
                    .unwrap();
                let record = save_file.get_record(&level_insertion_event.kind);
                let level = Level::new(level_insertion_event.kind.clone(), state, record);

                commands.insert_resource(level);
                scene_transition_event_writer.write(SceneTransitionEvent::level());
            }
            LevelKind::Custom(payload) => {
                let parsed_payload: Vec<&str> = payload.split('$').collect();
                let uuid = Uuid::parse_str(parsed_payload[1]).expect("Cannot parse uuid");
                let state = *level_states_assets
                    .get(level_handles.get_custom(&uuid).unwrap())
                    .unwrap();
                let record = save_file.get_record(&level_insertion_event.kind);
                let level = Level::new(level_insertion_event.kind.clone(), state, record);

                commands.insert_resource(level);
                scene_transition_event_writer.write(SceneTransitionEvent::level());
            }
            LevelKind::Playtest(state) => {
                let level = Level::new(
                    level_insertion_event.kind.clone(),
                    *state,
                    LevelRecord::default(),
                );

                commands.insert_resource(level);
                scene_transition_event_writer.write(SceneTransitionEvent::level());
            }
            LevelKind::Editable => {
                unreachable!("An editable should not be inserted with this event")
            }
        }
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
