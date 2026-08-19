//! 3D viewport panel content.
//!
//! The viewport is the central workspace where the user sees their 3D scene.
//! It provides:
//! - Transform tool toolbar (Move / Rotate / Scale)
//! - Click-to-select on entities
//! - Visual gizmo for the selected entity
//! - Camera info overlay
//! - Grid / axes / physics debug toggles

use crate::editor::components::{EditorCamera, ViewportCamera};
use crate::editor::resources::EditorSettings;
use crate::editor::state::Selection;
use bevy::prelude::*;
use bevy_egui::egui;

/// The current transform tool mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TransformMode {
    /// Select entities (no gizmo).
    #[default]
    Select,
    /// Move entities (translation gizmo).
    Move,
    /// Rotate entities.
    Rotate,
    /// Scale entities.
    Scale,
}

/// Draw the viewport content inside the central panel.
pub fn draw_content(
    ui: &mut egui::Ui,
    selection: &Selection,
    settings: &mut EditorSettings,
    camera_query: &Query<&ViewportCamera, With<EditorCamera>>,
    transform_mode: &mut TransformMode,
) {
    // ---- Compact tool toolbar (top of viewport) ----
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;

        // Transform tools
        let modes = [
            (TransformMode::Select, "👆 Select"),
            (TransformMode::Move, "✥ Move"),
            (TransformMode::Rotate, "⟳ Rotate"),
            (TransformMode::Scale, "⤢ Scale"),
        ];
        for (mode, label) in modes {
            let selected = *transform_mode == mode;
            let bg = if selected {
                egui::Color32::from_rgb(0, 122, 204)
            } else {
                egui::Color32::from_rgb(51, 51, 51)
            };
            let text_color = if selected {
                egui::Color32::WHITE
            } else {
                egui::Color32::from_rgb(180, 180, 180)
            };
            if ui
                .add(
                    egui::Button::new(egui::RichText::new(label).color(text_color))
                        .fill(bg)
                        .frame(false),
                )
                .clicked()
            {
                *transform_mode = mode;
            }
        }

        ui.separator();

        // View toggles
        ui.checkbox(&mut settings.show_grid, "Grid");
        ui.checkbox(&mut settings.show_axes, "Axes");
        ui.checkbox(&mut settings.show_gizmo, "Gizmo");
        ui.checkbox(&mut settings.show_physics_debug, "Physics");

        // Right-aligned help
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new("RMB: orbit | MMB: pan | Wheel: zoom | WASD: move | F: focus")
                    .color(egui::Color32::from_rgb(140, 140, 140))
                    .small(),
            );
        });
    });
    ui.separator();

    // ---- Central 3D rendering area ----
    // Bevy's camera renders to the whole window, so the 3D scene shows through.
    let available = ui.available_size();
    let (rect, _response) = ui.allocate_exact_size(available, egui::Sense::click_and_drag());

    // Subtle background tint to mark the viewport area
    ui.painter().rect_filled(
        rect,
        0.0,
        egui::Color32::from_rgba_unmultiplied(15, 15, 20, 30),
    );

    // ---- Camera info overlay (top-right) ----
    if let Some(cam) = camera_query.iter().next() {
        let pos = cam.position();
        let info = format!(
            "Camera  Pos: ({:.1}, {:.1}, {:.1})  Target: ({:.1}, {:.1}, {:.1})  Dist: {:.1}",
            pos.x, pos.y, pos.z, cam.target.x, cam.target.y, cam.target.z, cam.distance
        );
        ui.painter().text(
            rect.right_top() + egui::vec2(-8.0, 8.0),
            egui::Align2::RIGHT_TOP,
            &info,
            egui::FontId::monospace(10.0),
            egui::Color32::from_rgb(160, 160, 160),
        );
    }

    // ---- Transform mode indicator (bottom-left) ----
    let mode_label = match transform_mode {
        TransformMode::Select => "Mode: Select",
        TransformMode::Move => "Mode: Move (W)",
        TransformMode::Rotate => "Mode: Rotate (E)",
        TransformMode::Scale => "Mode: Scale (R)",
    };
    ui.painter().text(
        rect.left_top() + egui::vec2(8.0, 30.0),
        egui::Align2::LEFT_TOP,
        mode_label,
        egui::FontId::proportional(11.0),
        egui::Color32::from_rgb(180, 180, 180),
    );

    // ---- Selection info overlay (bottom-left) ----
    let sel_text = if selection.is_empty() {
        "No entity selected".to_string()
    } else if selection.entities.len() == 1 {
        "1 entity selected".to_string()
    } else {
        format!("{} entities selected", selection.entities.len())
    };
    ui.painter().text(
        rect.left_bottom() + egui::vec2(8.0, -8.0),
        egui::Align2::LEFT_BOTTOM,
        &sel_text,
        egui::FontId::proportional(11.0),
        egui::Color32::from_rgb(140, 140, 140),
    );

    // ---- Center hint when nothing selected ----
    if selection.is_empty() {
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "Use the Hierarchy panel or Add menu to create entities\nClick an entity in the Hierarchy to select it",
            egui::FontId::proportional(13.0),
            egui::Color32::from_rgb(100, 100, 100),
        );
    } else if let Some(primary) = selection.primary {
        // Draw a selection indicator box around the center of the viewport
        // (the actual 3D gizmo would be rendered by bevy_transform_gizmo)
        let center = rect.center();
        let gizmo_size = 40.0;
        ui.painter().rect_stroke(
            egui::Rect::from_center_size(center, egui::vec2(gizmo_size, gizmo_size)),
            2.0,
            egui::Stroke::new(1.5, egui::Color32::from_rgb(0, 180, 255)),
        );
        ui.painter().text(
            center + egui::vec2(0.0, gizmo_size / 2.0 + 8.0),
            egui::Align2::CENTER_TOP,
            format!("Entity {:?}", primary),
            egui::FontId::proportional(10.0),
            egui::Color32::from_rgb(0, 180, 255),
        );
    }
}
