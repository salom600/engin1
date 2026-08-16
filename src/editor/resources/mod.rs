//! Editor resources: project info, asset DB, log, settings, undo/redo history, camera state.

pub mod asset_db;
pub mod editor_log;
pub mod history;
pub mod project;
pub mod settings;

pub use asset_db::{AssetDatabase, AssetEntry, AssetKind};
pub use editor_log::EditorLog;
pub use history::{Command, CommandHistory};
pub use project::ProjectResource;
pub use settings::{EditorSettings, ThemeKind};

use bevy::prelude::*;

/// Per-camera editor state (mostly used for persistence of user preferences).
#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct EditorCameraState {
    /// Last-known camera position.
    pub last_position: bevy::prelude::Vec3,
    /// Last-known camera target.
    pub last_target: bevy::prelude::Vec3,
    /// Rotation sensitivity (read from settings; cached here for the camera system).
    pub rotation_sensitivity: f32,
    /// Zoom sensitivity (read from settings; cached here for the camera system).
    pub zoom_sensitivity: f32,
    /// Move speed (read from settings; cached here for the camera system).
    pub move_speed: f32,
}
