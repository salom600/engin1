//! Entity editing systems: delete, rename, duplicate.
//!
//! These listen for editor events and perform real ECS mutations.

use crate::editor::components::{
    DeleteEntityRequest, DuplicateEntityRequest, RenameEntityRequest, SceneEntity,
};
use crate::editor::state::Selection;
use bevy::prelude::*;

/// Handle entity deletion requests — despawns the entity and all its children.
pub fn handle_delete_requests(
    mut commands: Commands,
    mut events: EventReader<DeleteEntityRequest>,
    mut selection: ResMut<Selection>,
) {
    for req in events.read() {
        let entity = req.entity;
        commands.entity(entity).despawn_recursive();
        // Clear from selection
        selection.entities.retain(|&e| e != entity);
        if selection.primary == Some(entity) {
            selection.primary = None;
        }
        info!("Deleted entity {:?}", entity);
    }
    events.clear();
}

/// Handle entity rename requests.
pub fn handle_rename_requests(
    mut events: EventReader<RenameEntityRequest>,
    mut names: Query<&mut Name>,
) {
    for req in events.read() {
        if let Ok(mut name) = names.get_mut(req.entity) {
            name.set(req.new_name.clone());
            info!("Renamed entity {:?} to '{}'", req.entity, req.new_name);
        }
    }
    events.clear();
}

/// Handle entity duplication requests.
pub fn handle_duplicate_requests(
    mut commands: Commands,
    mut events: EventReader<DuplicateEntityRequest>,
    names: Query<&Name>,
    transforms: Query<&Transform>,
) {
    for req in events.read() {
        let entity = req.entity;
        // Simple duplication: spawn a new entity with the same name + transform
        // (full component cloning requires reflection — TODO for later)
        let name = names
            .get(entity)
            .map(|n| format!("{} (copy)", n.as_str()))
            .unwrap_or_else(|_| "Entity (copy)".to_string());
        let transform = transforms.get(entity).copied().unwrap_or_default();

        commands.spawn((
            SpatialBundle {
                transform: Transform::from_translation(
                    transform.translation + Vec3::new(1.0, 0.0, 1.0),
                ),
                ..default()
            },
            SceneEntity,
            Name::new(name),
        ));
        info!("Duplicated entity {:?}", entity);
    }
    events.clear();
}

/// Clean up selection when entities are despawned.
pub fn cleanup_selection_after_despawn(
    mut selection: ResMut<Selection>,
    mut removed: RemovedComponents<SceneEntity>,
) {
    for entity in removed.read() {
        selection.entities.retain(|&e| e != entity);
        if selection.primary == Some(entity) {
            selection.primary = None;
        }
    }
}
