//! Play mode system: snapshots the scene before play and restores it after.

use crate::editor::components::SceneEntity;
use crate::editor::state::EditorState;
use bevy::prelude::*;
use bevy::scene::{DynamicScene, DynamicSceneBuilder};
use bevy::utils::HashMap;

/// Resource holding the scene snapshot taken before entering play mode.
#[derive(Resource, Default)]
pub struct PlayModeSnapshot(pub Option<DynamicScene>);

/// Take a snapshot of the scene when entering Play mode.
pub fn snapshot_scene_before_play(world: &mut World) {
    let entities: Vec<Entity> = world
        .query_filtered::<Entity, With<SceneEntity>>()
        .iter(world)
        .collect();

    let entity_count = entities.len();
    let scene = DynamicSceneBuilder::from_world(world)
        .extract_entities(entities.into_iter())
        .build();

    world.resource_scope(|_world, mut snapshot: Mut<PlayModeSnapshot>| {
        snapshot.0 = Some(scene);
    });

    info!(
        "Play mode: scene snapshot taken ({} entities)",
        entity_count
    );
}

/// Restore the scene snapshot when exiting Play mode.
pub fn restore_scene_after_play(world: &mut World) {
    let scene =
        world.resource_scope(|_world, mut snapshot: Mut<PlayModeSnapshot>| snapshot.0.take());

    let Some(scene) = scene else {
        return;
    };

    // Clear current SceneEntity entities
    let to_despawn: Vec<Entity> = world
        .query_filtered::<Entity, With<SceneEntity>>()
        .iter(world)
        .collect();
    for e in to_despawn {
        world.entity_mut(e).despawn_recursive();
    }

    // Restore from snapshot
    let mut entity_map = HashMap::new();
    scene.write_to_world(world, &mut entity_map);
    info!("Play mode: scene restored from snapshot.");
}

/// Sync game-side state with the editor's [`EditorState`].
pub fn play_mode_sync_system(
    _current_state: Res<State<EditorState>>,
    _next_state: ResMut<NextState<EditorState>>,
    _time: Res<Time>,
) {
    let _ = (_current_state, _next_state, _time);
}
