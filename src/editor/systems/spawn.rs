//! Entity spawning system.
//!
//! Listens for [`SpawnRequest`] events and creates real Bevy entities with
//! proper components (mesh, material, transform, name, `SceneEntity` marker).

use crate::editor::components::{SceneEntity, SpawnRequest};
use bevy::math::primitives::{Cuboid, Plane3d, Sphere};
use bevy::prelude::*;

/// Handle all pending spawn requests.
pub fn handle_spawn_requests(
    mut commands: Commands,
    mut events: EventReader<SpawnRequest>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for req in events.read() {
        match req {
            SpawnRequest::Cube => {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                        material: materials.add(StandardMaterial {
                            base_color: Color::srgb(0.8, 0.3, 0.3),
                            ..default()
                        }),
                        transform: Transform::from_xyz(0.0, 0.5, 0.0),
                        ..default()
                    },
                    SceneEntity,
                    Name::new("Cube"),
                ));
            }
            SpawnRequest::Sphere => {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Sphere::new(0.5).mesh().ico(5).unwrap()),
                        material: materials.add(StandardMaterial {
                            base_color: Color::srgb(0.3, 0.5, 0.9),
                            ..default()
                        }),
                        transform: Transform::from_xyz(0.0, 0.5, 0.0),
                        ..default()
                    },
                    SceneEntity,
                    Name::new("Sphere"),
                ));
            }
            SpawnRequest::Plane => {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Plane3d::default().mesh().size(2.0, 2.0)),
                        material: materials.add(StandardMaterial {
                            base_color: Color::srgb(0.5, 0.5, 0.5),
                            ..default()
                        }),
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..default()
                    },
                    SceneEntity,
                    Name::new("Plane"),
                ));
            }
            SpawnRequest::Camera => {
                commands.spawn((
                    Camera3dBundle {
                        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
                        ..default()
                    },
                    SceneEntity,
                    Name::new("Camera"),
                ));
            }
            SpawnRequest::DirectionalLight => {
                commands.spawn((
                    DirectionalLightBundle {
                        directional_light: DirectionalLight {
                            illuminance: 10_000.0,
                            shadows_enabled: true,
                            ..default()
                        },
                        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
                        ..default()
                    },
                    SceneEntity,
                    Name::new("Directional Light"),
                ));
            }
            SpawnRequest::PointLight => {
                commands.spawn((
                    PointLightBundle {
                        point_light: PointLight {
                            intensity: 1_500.0,
                            range: 20.0,
                            shadows_enabled: true,
                            ..default()
                        },
                        transform: Transform::from_xyz(0.0, 4.0, 0.0),
                        ..default()
                    },
                    SceneEntity,
                    Name::new("Point Light"),
                ));
            }
            SpawnRequest::Empty => {
                commands.spawn((
                    SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..default()
                    },
                    SceneEntity,
                    Name::new("Empty Entity"),
                ));
            }
            SpawnRequest::ChildOf { parent } => {
                commands.entity(*parent).with_children(|p| {
                    p.spawn((
                        PbrBundle {
                            mesh: meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
                            material: materials.add(StandardMaterial {
                                base_color: Color::srgb(0.6, 0.6, 0.2),
                                ..default()
                            }),
                            transform: Transform::from_xyz(0.0, 1.0, 0.0),
                            ..default()
                        },
                        SceneEntity,
                        Name::new("Child"),
                    ));
                });
            }
        }
    }
    events.clear();
}
