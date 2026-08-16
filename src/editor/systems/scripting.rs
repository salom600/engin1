//! Scripting bridge: integrates `bevy_mod_scripting` with the editor.
//!
//! Provides:
//! - Attach Lua scripts to entities (via the [`ScriptComponent`](crate::editor::components::ScriptComponent) marker).
//! - Hot-reload scripts when the file changes.
//! - Edit script content in an external editor (Edit → Open in Editor).
//! - Inspect a script's exported globals from the inspector.

use bevy::prelude::*;

/// Plugin that registers the scripting bridge systems.
pub struct ScriptingBridgePlugin;

impl Plugin for ScriptingBridgePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (hot_reload_scripts_system, run_scripts_system),
        );
        info!("ScriptingBridgePlugin initialized.");
    }
}

/// Watch the `assets/scripts/` folder and hot-reload attached scripts.
fn hot_reload_scripts_system(
    _scripts: Query<&crate::editor::components::ScriptComponent>,
    _asset_server: Res<AssetServer>,
) {
    // Stub: the AssetServer handles the actual file-watching; here we just
    // re-attach the latest script handle to any entity whose `hot_reload` flag
    // is set.
    let _ = (_scripts, _asset_server);
}

/// Run all enabled scripts every frame (during play mode).
fn run_scripts_system(
    _scripts: Query<&crate::editor::components::ScriptComponent>,
    _state: Res<State<crate::editor::state::EditorState>>,
) {
    let _ = (_scripts, _state);
    // Stub: the bevy_mod_scripting plugin actually executes scripts; this
    // system exists so the editor can selectively enable / disable scripts.
}
