//! Editor-wide user-configurable settings.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Editor settings resource. Persisted to `editor.toml` in the project root.
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    /// Auto-save interval in seconds (0 = disabled).
    pub autosave_interval_secs: f32,
    /// Whether to show grid lines in the viewport.
    pub show_grid: bool,
    /// Whether to show the world origin axes.
    pub show_axes: bool,
    /// Whether to enable the physics debug renderer.
    pub show_physics_debug: bool,
    /// Whether to enable the transform gizmo.
    pub show_gizmo: bool,
    /// Camera move speed (units / second).
    pub camera_move_speed: f32,
    /// Camera rotation sensitivity.
    pub camera_rotation_sensitivity: f32,
    /// Camera zoom sensitivity.
    pub camera_zoom_sensitivity: f32,
    /// Whether to use a dark or light theme.
    pub theme: ThemeKind,
    /// Maximum undo history depth.
    pub max_undo_history: usize,
    /// Whether to capture screenshots of the viewport on F12.
    pub capture_screenshots: bool,
    /// Path to the user's preferred external text editor (for the "Open in Editor" action).
    pub external_editor: Option<String>,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            autosave_interval_secs: 300.0,
            show_grid: true,
            show_axes: true,
            show_physics_debug: false,
            show_gizmo: true,
            camera_move_speed: 10.0,
            camera_rotation_sensitivity: 0.005,
            camera_zoom_sensitivity: 0.5,
            theme: ThemeKind::Dark,
            max_undo_history: 100,
            capture_screenshots: true,
            external_editor: None,
        }
    }
}

/// Available editor themes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeKind {
    /// Dark theme (default).
    Dark,
    /// Light theme.
    Light,
}
