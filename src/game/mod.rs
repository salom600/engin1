//! # Sample Game Content
//!
//! A minimal "starter scene" the user sees when they first open the editor:
//! a few primitives arranged around the origin. This is the **game's**
//! plugin — the editor's own setup (camera, light, ground) lives in
//! [`crate::editor::setup_editor_camera`].
//!
//! When the user opens their own project, they can disable this plugin.

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Sample game plugin: spawns a few demo entities the user can interact with.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sample_scene);
    }
}

/// Spawn a small demo scene: a rotating cube, a sphere, and a torus.
fn spawn_sample_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spinning cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.2, 0.2),
                ..default()
            }),
            transform: Transform::from_xyz(-2.0, 0.5, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
        crate::editor::components::SceneEntity,
        Name::new("Demo Cube"),
    ));

    // Sphere
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.5).mesh().ico(5).unwrap()),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.2, 0.5, 0.8),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(0.5),
        crate::editor::components::SceneEntity,
        Name::new("Demo Sphere"),
    ));

    // Torus
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Torus::new(0.4, 0.8, 16, 32)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.9, 0.8, 0.2),
                ..default()
            }),
            transform: Transform::from_xyz(2.0, 0.5, 0.0),
            ..default()
        },
        crate::editor::components::SceneEntity,
        Name::new("Demo Torus"),
    ));

    info!("Sample scene spawned: cube + sphere + torus");
}
