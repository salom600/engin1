//! Play mode system: snapshots the scene before play and restores it after.

use crate::editor::components::SceneEntity;
use crate::editor::state::EditorState;
use bevy::prelude::*;
use bevy::scene::{DynamicScene, DynamicSceneBuilder};

/// Resource holding the scene snapshot taken before entering play mode.
#[derive(Resource, Default)]
pub struct PlayModeSnapshot(pub Option<DynamicScene>);

/// Take a snapshot of the scene when entering Play mode.
pub fn snapshot_scene_before_play(
    world: &mut World,
    type_registry: Res<AppTypeRegistry>,
    mut snapshot: ResMut<PlayModeSnapshot>,
) {
    let entities: Vec<Entity> = world
        .query_filtered::<Entity, With<SceneEntity>>()
        .iter(world)
        .collect();

    snapshot.0 = Some(
        DynamicSceneBuilder::from_world(world)
            .extract_entities(entities.into_iter())
            .build(),
    );
    info!("Play mode: scene snapshot taken ({} entities)", entities.len());
}

/// Restore the scene snapshot when exiting Play mode.
pub fn restore_scene_after_play(
    world: &mut World,
    type_registry: Res<AppTypeRegistry>,
    mut snapshot: ResMut<PlayModeSnapshot>,
) {
    let Some(scene) = snapshot.0.take() else {
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
    scene.write_to_world(world, &type_registry);
    info!("Play mode: scene restored from snapshot.");
}

/// Sync game-side state with the editor's [`EditorState`].
pub fn play_mode_sync_system(
    _current_state: Res<State<EditorState>>,
    _next_state: ResMut<NextState<EditorState>>,
    _time: Res<Time>,
) {
    // The state machine handles transitions automatically.
    // This system is a hook for any additional per-frame play-mode logic.
    let _ = (_current_state, _next_state, _time);
}
