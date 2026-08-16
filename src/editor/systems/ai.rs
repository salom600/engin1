//! AI bridge: integrates `big-brain` with the editor.
//!
//! Provides:
//! - Attach AI agents (utility AI / behavior trees) to entities.
//! - Inspect / debug the agent's current state.
//! - Visualize the agent's decision-making in the viewport.

use bevy::prelude::*;

/// Plugin that registers the AI bridge systems.
pub struct AIBridgePlugin;

impl Plugin for AIBridgePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_ai_agents_system);
        info!("AIBridgePlugin initialized.");
    }
}

/// Log the state of all AI agents every second (for debugging).
fn debug_ai_agents_system(
    _agents: Query<&crate::editor::components::AIAgent>,
    _time: Res<Time>,
) {
    // Stub: the big-brain plugin handles the actual scoring & action selection;
    // here we would log the agent's current action to the EditorLog for the
    // Console panel.
    let _ = (_agents, _time);
}
