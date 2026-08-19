//! Viewport scissor system.
//!
//! This system reads the [`ViewportRect`](crate::editor::components::ViewportRect)
//! resource (updated by the master layout system every frame) and applies it
//! to the editor camera's `Camera::viewport` field.
//!
//! This ensures the 3D scene ONLY renders inside the central panel area —
//! not behind the side panels, bottom panels, or toolbar. Without this,
//! Bevy's camera renders to the entire window and egui panels draw on top
//! of it, causing the "3D objects hidden behind panels" problem.

use crate::editor::components::{EditorCamera, ViewportRect};
use bevy::prelude::*;
use bevy::render::camera::Viewport;

/// Apply the viewport rect to the editor camera.
///
/// This system runs in `PostUpdate` (after egui has laid out all panels)
/// so the `ViewportRect` resource is up-to-date.
pub fn apply_viewport_to_camera(
    viewport_rect: Res<ViewportRect>,
    mut camera_query: Query<&mut Camera, With<EditorCamera>>,
) {
    let Some(rect) = viewport_rect.rect else {
        return;
    };

    for mut camera in camera_query.iter_mut() {
        camera.viewport = Some(Viewport {
            physical_position: rect.min,
            physical_size: rect.size(),
            depth: 0.0..1.0,
        });
    }
}
