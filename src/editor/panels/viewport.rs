//! 3D viewport panel content.
//!
//! The viewport is drawn inside the `CentralPanel` by the master layout
//! system. The actual 3D rendering happens on a Bevy camera tagged with
//! [`EditorCamera`](crate::editor::components::EditorCamera) — egui just
//! draws overlay UI (toolbar, status, hints) on top of the rendered image.

use crate::editor::components::{EditorCamera, ViewportCamera};
use crate::editor::resources::EditorSettings;
use crate::editor::state::Selection;
use bevy::prelude::*;
use bevy_egui::egui;

/// Draw the viewport content inside the central panel.
///
/// This function does NOT create its own `TopBottomPanel` or `CentralPanel` —
/// the caller (layout system) is responsible for creating the `CentralPanel`
/// and passing its `ui` to this function.
pub fn draw_content(
    ui: &mut egui::Ui,
    selection: &Selection,
    settings: &mut EditorSettings,
    camera_query: &Query<&ViewportCamera, With<EditorCamera>>,
) {
    // ---- Mini-toolbar at the top of the viewport ----
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        ui.strong("Perspective");
        ui.separator();
        ui.checkbox(&mut settings.show_grid, "Grid");
        ui.checkbox(&mut settings.show_axes, "Axes");
        ui.checkbox(&mut settings.show_gizmo, "Gizmo");
        ui.checkbox(&mut settings.show_physics_debug, "Physics");

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new("RMB: orbit  |  MMB: pan  |  Wheel: zoom  |  WASD: move")
                    .color(egui::Color32::from_rgb(140, 140, 140))
                    .small(),
            );
        });
    });
    ui.separator();

    // ---- Central rendering area ----
    // Bevy's camera renders to the whole window, so the 3D scene shows through
    // here. We just draw an overlay border and some hints.
    let (rect, _response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());

    // Draw a subtle background to mark the viewport area
    ui.painter().rect_filled(
        rect,
        2.0,
        egui::Color32::from_rgba_unmultiplied(20, 20, 25, 40),
    );

    // Camera info overlay (top-right)
    if let Some(cam) = camera_query.iter().next() {
        let pos = cam.position();
        let info = format!(
            "Camera\nPos: ({:.1}, {:.1}, {:.1})\nTarget: ({:.1}, {:.1}, {:.1})\nDist: {:.1}",
            pos.x, pos.y, pos.z, cam.target.x, cam.target.y, cam.target.z, cam.distance
        );
        ui.painter().text(
            rect.right_top() + egui::vec2(-12.0, 12.0),
            egui::Align2::RIGHT_TOP,
            &info,
            egui::FontId::monospace(11.0),
            egui::Color32::from_rgb(180, 180, 180),
        );
    }

    // Selection info overlay (bottom-left)
    let sel_text = if selection.is_empty() {
        "No entity selected".to_string()
    } else {
        format!("{} entit{} selected", selection.entities.len(), if selection.entities.len() == 1 { "y" } else { "ies" })
    };
    ui.painter().text(
        rect.left_bottom() + egui::vec2(12.0, -12.0),
        egui::Align2::LEFT_BOTTOM,
        &sel_text,
        egui::FontId::proportional(12.0),
        egui::Color32::from_rgb(140, 140, 140),
    );

    // Center hint when nothing selected
    if selection.is_empty() {
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "Click an entity in the Hierarchy to select it",
            egui::FontId::proportional(14.0),
            egui::Color32::from_rgb(100, 100, 100),
        );
    }
}
