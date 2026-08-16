//! Save / Load + autosave.
//!
//! Persists the current scene to disk as a `.scn.ron` file and reloads it.
//! Autosave fires every N seconds (configurable via [`EditorSettings`]).

use crate::editor::resources::EditorSettings;
use bevy::prelude::*;

/// Autosave the current scene on a configurable interval.
pub fn autosave_system(
    time: Res<Time>,
    settings: Res<EditorSettings>,
    mut last_save: Local<f32>,
) {
    if settings.autosave_interval_secs <= 0.0 {
        return;
    }
    *last_save += time.delta_seconds();
    if *last_save >= settings.autosave_interval_secs {
        info!("Autosave triggered (interval: {}s)", settings.autosave_interval_secs);
        // The real implementation would:
        // 1. Build a DynamicScene from the current world.
        // 2. Write it to `assets/scenes/autosave.scn.ron`.
        *last_save = 0.0;
    }
}
