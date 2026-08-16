//! 3D viewport panel.
//!
//! The viewport is the central canvas where the user sees their scene rendered.
//! In the current architecture, the actual rendering happens on a Bevy camera
//! (tagged with [`EditorCamera`](crate::editor::components::EditorCamera)) and
//! this panel just provides the toolbar / status overlay on top of the rendered
//! image.
//!
//! Future improvements:
//! - Render the camera output to a texture and embed it in this panel.
//! - Support multiple viewports (top / front / side / perspective like Blender).
//! - 2D / 3D toggle.

use crate::editor::components::{EditorCamera, ViewportCamera};
use crate::editor::resources::EditorSettings;
use crate::editor::state::Selection;
use bevy::prelude::*;
use bevy_egui::egui;

/// Viewport panel system.
pub fn draw_system(
    mut ctxs: bevy_egui::EguiContexts,
    selection: Res<Selection>,
    mut settings: ResMut<EditorSettings>,
    camera_query: Query<&ViewportCamera, With<EditorCamera>>,
) {
    let Some(ctx) = ctxs.ctx_mut() else {
        return;
    };

    // Take a snapshot of the settings so we can mutate them through egui closures.
    let mut show_grid = settings.show_grid;
    let mut show_axes = settings.show_axes;
    let mut show_gizmo = settings.show_gizmo;
    let mut show_physics_debug = settings.show_physics_debug;

    // The "viewport" toolbar — runs along the top of the editor area.
    egui::TopBottomPanel::top("viewport_toolbar")
        .exact_height(28.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.strong("Perspective");
                ui.separator();
                ui.checkbox(&mut show_grid, "Grid");
                ui.checkbox(&mut show_axes, "Axes");
                ui.checkbox(&mut show_gizmo, "Gizmo");
                ui.checkbox(&mut show_physics_debug, "Physics Debug");

                ui.separator();
                if let Some(cam) = camera_query.iter().next() {
                    ui.label(format!(
                        "Cam: target=({:.1}, {:.1}, {:.1}) dist={:.1}",
                        cam.target.x, cam.target.y, cam.target.z, cam.distance
                    ));
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("WASD: move  |  QE: up/down  |  RMB+drag: orbit  |  Scroll: zoom");
                });
            });
        });

    // Bottom status bar — shows selection info + camera info.
    egui::TopBottomPanel::bottom("viewport_status")
        .exact_height(22.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Selected: {} entities", selection.entities.len()));
                ui.separator();
                if let Some(primary) = selection.primary {
                    ui.label(format!("Primary: {:?}", primary));
                } else {
                    ui.label("Nothing selected");
                }
                ui.separator();
                if let Some(cam) = camera_query.iter().next() {
                    ui.label(format!(
                        "Pos: ({:.1}, {:.1}, {:.1})",
                        cam.position().x,
                        cam.position().y,
                        cam.position().z
                    ));
                }
            });
        });

    // The central area: Bevy's rendering output shows through here.
    egui::CentralPanel::default().show(ctx, |ui| {
        // Draw a faint border / background to mark the viewport boundary.
        let (rect, _) = ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
        let painter = ui.painter_at(rect);
        painter.rect_filled(
            rect,
            4.0,
            egui::Color32::from_rgba_unmultiplied(15, 15, 15, 60),
        );

        // Show a hint when nothing is selected
        if selection.is_empty() {
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Click an entity to select it",
                egui::FontId::proportional(14.0),
                egui::Color32::from_rgb(120, 120, 120),
            );
        }
    });

    // Write back any changed settings.
    if settings.show_grid != show_grid
        || settings.show_axes != show_axes
        || settings.show_gizmo != show_gizmo
        || settings.show_physics_debug != show_physics_debug
    {
        settings.show_grid = show_grid;
        settings.show_axes = show_axes;
        settings.show_gizmo = show_gizmo;
        settings.show_physics_debug = show_physics_debug;
    }
}
