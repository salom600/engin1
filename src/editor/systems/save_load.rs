//! Scene save / load systems.
//!
//! Uses Bevy's [`DynamicSceneBuilder`] to serialize all `SceneEntity`-tagged
//! entities to a `.scn.ron` file, and to load them back.

use crate::editor::components::SceneEntity;
use crate::editor::state::CurrentScenePath;
use bevy::prelude::*;
use bevy::scene::{DynamicScene, DynamicSceneBuilder};
use std::io::Write;

/// Handle save scene requests — serializes all SceneEntity entities to disk.
pub fn handle_save_requests(
    world: &mut World,
    mut events: EventReader<crate::editor::components::SaveSceneRequest>,
    type_registry: Res<AppTypeRegistry>,
    current_path: Res<CurrentScenePath>,
) {
    if events.read().next().is_none() {
        return;
    }
    events.clear();

    // Collect all SceneEntity entities (the user's content, not editor-only entities)
    let entities: Vec<Entity> = world
        .query_filtered::<Entity, With<SceneEntity>>()
        .iter(world)
        .collect();

    if entities.is_empty() {
        warn!("No entities to save — scene is empty.");
        return;
    }

    // Build the DynamicScene
    let scene = DynamicSceneBuilder::from_world(world)
        .extract_entities(entities.into_iter())
        .build();

    let ron_string = match scene.serialize(&type_registry) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to serialize scene: {e}");
            return;
        }
    };

    let path = current_path.0.clone().unwrap_or_else(|| {
        let dir = std::env::current_dir().unwrap_or_default();
        dir.join("assets/scenes/scene.scn.ron")
    });

    // Ensure the parent directory exists
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    match std::fs::File::create(&path).and_then(|mut f| f.write_all(ron_string.as_bytes())) {
        Ok(_) => info!("Scene saved to {:?}", path),
        Err(e) => error!("Failed to write scene file: {e}"),
    }
}

/// Handle load scene requests — loads a .scn.ron file and spawns its entities.
pub fn handle_load_requests(
    world: &mut World,
    mut events: EventReader<crate::editor::components::LoadSceneRequest>,
    type_registry: Res<AppTypeRegistry>,
    mut current_path: ResMut<CurrentScenePath>,
) {
    for req in events.read() {
        let bytes = match std::fs::read(&req.path) {
            Ok(b) => b,
            Err(e) => {
                error!("Failed to read scene file {:?}: {e}", req.path);
                continue;
            }
        };

        let scene = match DynamicScene::from_bytes(&type_registry, &bytes) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to parse scene RON: {e}");
                continue;
            }
        };

        // Clear existing SceneEntity entities before loading
        let to_despawn: Vec<Entity> = world
            .query_filtered::<Entity, With<SceneEntity>>()
            .iter(world)
            .collect();
        for e in to_despawn {
            world.entity_mut(e).despawn_recursive();
        }

        // Spawn all entities from the scene into the world
        scene.write_to_world(world, &type_registry);
        current_path.0 = Some(req.path.clone());
        info!("Loaded scene from {:?}", req.path);
    }
    events.clear();
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
