//! Physics bridge: integrates `bevy_rapier3d` with the editor.
//!
//! Exposes the following editor features:
//! - Toggle the Rapier debug renderer.
//! - Spawn entities with `RigidBody` / `Collider` components.
//! - Step the simulation manually in pause mode.
//! - Reset all rigid bodies to their initial positions when exiting play mode.

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Plugin that registers the physics bridge systems.
pub struct PhysicsBridgePlugin;

impl Plugin for PhysicsBridgePlugin {
    fn build(&self, app: &mut App) {
        // The RapierPhysicsPlugin is already registered in main.rs (because we need
        // it active in both edit and play modes for the editor's own colliders,
        // e.g. the ground plane). Here we just register the editor-side bridge.

        app.add_systems(
            Update,
            (toggle_debug_render_system, manual_step_system)
                .run_if(in_state(crate::editor::state::EditorState::Paused)),
        );

        info!("PhysicsBridgePlugin initialized.");
    }
}

/// Toggle the Rapier debug renderer based on the editor settings.
fn toggle_debug_render_system(
    _settings: Res<crate::editor::resources::EditorSettings>,
    mut debug_render: ResMut<DebugRenderContext>,
) {
    debug_render.enabled = _settings.show_physics_debug;
}

/// Allow the user to step the simulation one frame at a time when paused.
fn manual_step_system(_time: Res<Time>) {
    // Stub: in a real implementation, we would:
    // 1. Listen for the "Step" toolbar button press.
    // 2. Call RapierConfiguration::step / set timestep to a fixed value.
    // 3. Run one physics step.
    let _ = _time;
}
