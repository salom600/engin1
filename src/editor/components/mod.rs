//! Editor marker components.
//!
//! These are small, mostly marker-style components attached to entities
//! so that the editor's systems can identify them without reflection.

use bevy::prelude::*;

/// Marker attached to the editor's viewport camera.
#[derive(Component, Debug, Clone, Copy, Default, Reflect)]
#[reflect(Component)]
pub struct EditorCamera;

/// Resource holding the current viewport rectangle (in physical pixels).
///
/// The master layout system updates this every frame after egui lays out
/// the CentralPanel. The camera system reads it and sets the camera's
/// `Camera::viewport` field so the 3D scene only renders inside the
/// central panel area — not behind the side/bottom panels.
#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct ViewportRect {
    /// The rectangle in physical pixels where the 3D scene should render.
    pub rect: Option<bevy::math::URect>,
}

/// Per-camera viewport state (orbit angles, distance, target).
#[derive(Component, Debug, Clone, Copy, Default, Reflect)]
#[reflect(Component)]
pub struct ViewportCamera {
    /// Yaw angle (radians, around Y axis).
    pub yaw: f32,
    /// Pitch angle (radians, around X axis).
    pub pitch: f32,
    /// Distance from the orbit target.
    pub distance: f32,
    /// Orbit target point in world space.
    pub target: Vec3,
}

impl ViewportCamera {
    /// Construct a viewport camera looking at the origin from a reasonable distance.
    pub fn new(distance: f32, yaw: f32, pitch: f32) -> Self {
        Self {
            yaw,
            pitch,
            distance,
            target: Vec3::ZERO,
        }
    }

    /// Recompute the camera position from the orbit parameters.
    pub fn position(&self) -> Vec3 {
        let cos_pitch = self.pitch.cos();
        let x = self.target.x + self.distance * cos_pitch * self.yaw.sin();
        let y = self.target.y + self.distance * self.pitch.sin();
        let z = self.target.z + self.distance * cos_pitch * self.yaw.cos();
        Vec3::new(x, y, z)
    }
}

/// Marker attached to entities that are selected in the editor.
#[derive(Component, Debug, Clone, Copy, Default, Reflect)]
#[reflect(Component)]
pub struct Selected;

/// Marker attached to entities that are *locked* (cannot be selected or modified).
#[derive(Component, Debug, Clone, Copy, Default, Reflect)]
#[reflect(Component)]
pub struct Locked;

/// Marker attached to entities that are *hidden* in the viewport.
#[derive(Component, Debug, Clone, Copy, Default, Reflect)]
#[reflect(Component)]
pub struct Hidden;

/// Marker attached to every entity that was spawned by the user (not the editor).
///
/// The scene hierarchy uses this to differentiate editor-only entities (cameras,
/// lights used by the editor) from the user's scene content.
#[derive(Component, Debug, Clone, Copy, Default, Reflect)]
#[reflect(Component)]
pub struct SceneEntity;

/// Marker attached to an entity that should be drawn with a custom debug outline
/// in the viewport (e.g. the selection box).
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct DebugOutline {
    /// Outline color.
    pub color: Color,
    /// Outline width in pixels.
    pub width: f32,
}

/// Marker attached to the editor-only directional light used to illuminate the viewport.
#[derive(Component, Debug, Clone, Copy, Default, Reflect)]
#[reflect(Component)]
pub struct EditorLight;

/// Tag describing what a script component should run as.
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct ScriptComponent {
    /// Path to the script (relative to `assets/scripts/`).
    pub script_path: String,
    /// Whether the script is currently enabled.
    pub enabled: bool,
    /// Optional hot-reload flag.
    pub hot_reload: bool,
}

/// Tag describing an AI agent — driven by the `big-brain` crate.
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct AIAgent {
    /// Name of the agent (for debugging).
    pub name: String,
    /// Whether the AI is currently enabled.
    pub enabled: bool,
}

/// What kind of entity to spawn when the user clicks "Add" in the toolbar.
#[derive(Event, Clone, Debug)]
pub enum SpawnRequest {
    /// Spawn a cube primitive.
    Cube,
    /// Spawn a sphere primitive.
    Sphere,
    /// Spawn a plane primitive.
    Plane,
    /// Spawn a camera.
    Camera,
    /// Spawn a directional light.
    DirectionalLight,
    /// Spawn a point light.
    PointLight,
    /// Spawn an empty entity (no mesh, just Transform + Name).
    Empty,
    /// Spawn a child of the given parent entity.
    ChildOf {
        /// The parent entity to attach the new child to.
        parent: Entity,
    },
}

/// Request to delete an entity (and all its children).
#[derive(Event, Clone, Debug)]
pub struct DeleteEntityRequest {
    /// The entity to delete.
    pub entity: Entity,
}

/// Request to rename an entity.
#[derive(Event, Clone, Debug)]
pub struct RenameEntityRequest {
    /// The entity to rename.
    pub entity: Entity,
    /// The new name.
    pub new_name: String,
}

/// Request to save the current scene to disk.
#[derive(Event, Clone, Debug, Default)]
pub struct SaveSceneRequest;

/// Request to load a scene from disk.
#[derive(Event, Clone, Debug)]
pub struct LoadSceneRequest {
    /// The path to load from.
    pub path: std::path::PathBuf,
}

/// Request to duplicate an entity.
#[derive(Event, Clone, Debug)]
pub struct DuplicateEntityRequest {
    /// The entity to duplicate.
    pub entity: Entity,
}

/// What kind of component to add to an entity.
#[derive(Event, Clone, Debug)]
pub enum AddComponentRequest {
    /// Add a physics RigidBody to the entity.
    RigidBody {
        /// The target entity.
        entity: Entity,
        /// Dynamic (affected by forces), Static (immovable), or Kinematic (moved manually).
        body_type: RigidBodyType,
    },
    /// Add a collider shape to the entity.
    Collider {
        /// The target entity.
        entity: Entity,
        /// The shape of the collider.
        shape: ColliderShape,
    },
    /// Attach a Lua script to the entity.
    LuaScript {
        /// The target entity.
        entity: Entity,
        /// Path to the .lua file (relative to assets/scripts/).
        path: String,
    },
    /// Attach a Rhai script (Rust-like syntax) to the entity.
    RhaiScript {
        /// The target entity.
        entity: Entity,
        /// Path to the .rhai file (relative to assets/scripts/).
        path: String,
    },
    /// Add an AI agent to the entity.
    AIAgent {
        /// The target entity.
        entity: Entity,
        /// Name of the agent (for debugging).
        name: String,
    },
}

/// The type of a physics rigid body.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum RigidBodyType {
    /// Affected by gravity and forces.
    Dynamic,
    /// Immovable; collides but doesn't move.
    Static,
    /// Moved manually via code; not affected by forces.
    Kinematic,
}

/// The shape of a physics collider.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum ColliderShape {
    /// Box-shaped (cuboid) collider.
    Box,
    /// Spherical collider.
    Sphere,
}
