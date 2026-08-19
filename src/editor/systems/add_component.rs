//! Handle AddComponentRequest events — adds real ECS components to entities.

use crate::editor::components::{
    AddComponentRequest, AIAgent, ColliderShape, RigidBodyType, ScriptComponent,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody};

/// Process all pending AddComponentRequest events.
pub fn handle_add_component_requests(
    mut commands: Commands,
    mut events: EventReader<AddComponentRequest>,
    transforms: Query<&Transform>,
) {
    for req in events.read() {
        match req {
            AddComponentRequest::RigidBody { entity, body_type } => {
                let rb = match body_type {
                    RigidBodyType::Dynamic => RigidBody::Dynamic,
                    RigidBodyType::Static => RigidBody::Fixed,
                    RigidBodyType::Kinematic => RigidBody::KinematicPositionBased,
                };
                commands.entity(*entity).insert(rb);
                info!(
                    "Added RigidBody ({:?}) to entity {:?}",
                    body_type, entity
                );
            }
            AddComponentRequest::Collider { entity, shape } => {
                let collider = match shape {
                    ColliderShape::Box => {
                        // Try to match the entity's scale, default to 0.5 half-extents
                        let half = transforms
                            .get(*entity)
                            .map(|t| t.scale * 0.5)
                            .unwrap_or(Vec3::splat(0.5));
                        Collider::cuboid(half.x, half.y, half.z)
                    }
                    ColliderShape::Sphere => {
                        let radius = transforms
                            .get(*entity)
                            .map(|t| t.scale.x * 0.5)
                            .unwrap_or(0.5);
                        Collider::ball(radius)
                    }
                };
                commands.entity(*entity).insert(collider);
                info!("Added Collider ({:?}) to entity {:?}", shape, entity);
            }
            AddComponentRequest::LuaScript { entity, path } => {
                commands.entity(*entity).insert(ScriptComponent {
                    script_path: path.clone(),
                    enabled: true,
                    hot_reload: true,
                });
                info!("Attached Lua script '{}' to entity {:?}", path, entity);
            }
            AddComponentRequest::RhaiScript { entity, path } => {
                // Rhai scripts use the same ScriptComponent but with .rhai extension
                commands.entity(*entity).insert(ScriptComponent {
                    script_path: path.clone(),
                    enabled: true,
                    hot_reload: true,
                });
                info!("Attached Rhai script '{}' to entity {:?}", path, entity);
            }
            AddComponentRequest::AIAgent { entity, name } => {
                commands.entity(*entity).insert(AIAgent {
                    name: name.clone(),
                    enabled: true,
                });
                info!("Added AI Agent '{}' to entity {:?}", name, entity);
            }
        }
    }
    events.clear();
}
