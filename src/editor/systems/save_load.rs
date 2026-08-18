//! Scene save / load systems.
//!
//! Uses Bevy's [`DynamicSceneBuilder`] to serialize all `SceneEntity`-tagged
//! entities to a `.scn.ron` file, and to load them back.

use crate::editor::components::SceneEntity;
use crate::editor::state::CurrentScenePath;
use bevy::ecs::entity::EntityHashMap;
use bevy::prelude::*;
use bevy::reflect::serde::SceneDeserializer;
use bevy::scene::{DynamicScene, DynamicSceneBuilder};
use ron::de::Deserializer;
use std::io::Write;

/// Handle save scene requests — serializes all SceneEntity entities to disk.
/// This is an exclusive system (takes &mut World) so it must be registered alone.
pub fn handle_save_requests(world: &mut World) {
    // Check for save events using resource_scope
    let should_save =
        world.resource_scope(
            |_world,
             mut events: Mut<
                bevy::ecs::event::Events<crate::editor::components::SaveSceneRequest>,
            >| {
                let has_events = !events.is_empty();
                events.clear();
                has_events
            },
        );

    if !should_save {
        return;
    }

    // Collect all SceneEntity entities
    let entities: Vec<Entity> = world
        .query_filtered::<Entity, With<SceneEntity>>()
        .iter(world)
        .collect();

    if entities.is_empty() {
        warn!("No entities to save — scene is empty.");
        return;
    }

    let entity_count = entities.len();
    let scene = DynamicSceneBuilder::from_world(world)
        .extract_entities(entities.into_iter())
        .build();

    let (ron_string, current_path) =
        world.resource_scope(|_world, type_registry: Mut<AppTypeRegistry>| {
            let registry = type_registry.read();
            let ron = scene.serialize(&registry);
            let path = _world.resource::<CurrentScenePath>().0.clone();
            (ron, path)
        });

    let ron_string = match ron_string {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to serialize scene: {e}");
            return;
        }
    };

    let path = current_path.unwrap_or_else(|| {
        let dir = std::env::current_dir().unwrap_or_default();
        dir.join("assets/scenes/scene.scn.ron")
    });

    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    match std::fs::File::create(&path).and_then(|mut f| f.write_all(ron_string.as_bytes())) {
        Ok(_) => info!("Scene saved to {:?} ({} entities)", path, entity_count),
        Err(e) => error!("Failed to write scene file: {e}"),
    }
}

/// Handle load scene requests — loads a .scn.ron file and spawns its entities.
/// This is an exclusive system.
pub fn handle_load_requests(world: &mut World) {
    let load_paths: Vec<std::path::PathBuf> =
        world.resource_scope(
            |_world,
             mut events: Mut<
                bevy::ecs::event::Events<crate::editor::components::LoadSceneRequest>,
            >| {
                let paths: Vec<_> = events.drain().map(|e| e.path).collect();
                paths
            },
        );

    if load_paths.is_empty() {
        return;
    }

    for path in load_paths {
        let ron_string = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to read scene file {:?}: {e}", path);
                continue;
            }
        };

        let scene_result = world.resource_scope(|_world, type_registry: Mut<AppTypeRegistry>| {
            let registry = type_registry.read();
            let mut deserializer =
                Deserializer::from_str(&ron_string).map_err(|e| e.to_string())?;
            SceneDeserializer {
                type_registry: &registry,
            }
            .deserialize(&mut deserializer)
            .map_err(|e| e.to_string())
        });

        let scene = match scene_result {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to parse scene RON: {e}");
                continue;
            }
        };

        // Clear existing SceneEntity entities
        let to_despawn: Vec<Entity> = world
            .query_filtered::<Entity, With<SceneEntity>>()
            .iter(world)
            .collect();
        for e in to_despawn {
            world.entity_mut(e).despawn_recursive();
        }

        // Spawn all entities from the scene into the world
        let mut entity_map = EntityHashMap::new();
        scene.write_to_world(world, &mut entity_map);
        world.resource_mut::<CurrentScenePath>().0 = Some(path.clone());
        info!("Loaded scene from {:?}", path);
    }
}

/// Autosave the current scene on a configurable interval.
pub fn autosave_system(
    time: Res<Time>,
    settings: Res<crate::editor::resources::EditorSettings>,
    mut last_save: Local<f32>,
    mut save_events: EventWriter<crate::editor::components::SaveSceneRequest>,
) {
    if settings.autosave_interval_secs <= 0.0 {
        return;
    }
    *last_save += time.delta_seconds();
    if *last_save >= settings.autosave_interval_secs {
        save_events.send(crate::editor::components::SaveSceneRequest);
        *last_save = 0.0;
    }
}
