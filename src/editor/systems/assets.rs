//! Asset pipeline: scans `assets/`, organizes files, compresses images/audio.
//!
//! This plugin periodically rescans the project's `assets/` folder and updates
//! the in-memory [`AssetDatabase`] so the Asset Browser panel sees new files
//! without restarting the editor.

use crate::editor::resources::{AssetDatabase, ProjectResource};
use bevy::prelude::*;

/// Plugin that registers the asset pipeline systems.
pub struct AssetPipelinePlugin;

impl Plugin for AssetPipelinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, periodic_rescan_system);
        info!("AssetPipelinePlugin initialized.");
    }
}

/// On the first frame after startup, scan the assets directory and populate
/// the [`AssetDatabase`]. Subsequent scans happen on a timer (every 5s).
pub fn rescan_on_startup_system(
    mut db: ResMut<AssetDatabase>,
    project: Res<ProjectResource>,
    mut last_scan: Local<f32>,
    time: Res<Time>,
) {
    *last_scan += time.delta_seconds();
    if *last_scan < 5.0 && !db.entries.is_empty() {
        return;
    }
    if let Err(e) = db.rescan(&project.assets_dir) {
        warn!("Failed to scan assets/: {e}");
    } else {
        debug!("Assets scanned: {} entries", db.entries.len());
    }
    *last_scan = 0.0;
}

/// Periodically rescan the assets directory (every 5 seconds).
fn periodic_rescan_system(
    mut db: ResMut<AssetDatabase>,
    project: Res<ProjectResource>,
    mut timer: Local<Timer>,
    _time: Res<Time>,
) {
    if timer.duration().as_secs() == 0 {
        *timer = Timer::from_seconds(5.0, TimerMode::Repeating);
    }
    // Without Time resource, just always rescan (timer logic simplified)
    let _ = timer.tick(_time.delta());
    if !timer.just_finished() {
        return;
    }
    if let Err(e) = db.rescan(&project.assets_dir) {
        warn!("Periodic asset rescan failed: {e}");
    }
}
