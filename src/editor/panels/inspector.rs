//! Component inspector panel content.
//!
//! Shows the components of the currently selected entity and lets the user
//! edit them. Drawn inside a `SidePanel::right` by the master layout system.

use crate::editor::state::Selection;
use bevy::prelude::*;
use bevy_egui::egui;

/// Draw the inspector content inside the given `ui`.
pub fn draw_content(
    ui: &mut egui::Ui,
    selection: &Selection,
    transform_query: &Query<&Transform>,
    visibility_query: &Query<&Visibility>,
    name_query: &Query<&Name>,
) {
    // ---- Header ----
    ui.horizontal(|ui| {
        ui.strong("Inspector");
    });
    ui.separator();

    let Some(primary) = selection.primary else {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.label("No entity selected.");
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new("Select an entity in the\nHierarchy or Viewport to inspect it.")
                    .color(egui::Color32::from_rgb(140, 140, 140))
                    .small(),
            );
        });
        return;
    };

    // ---- Entity header ----
    ui.horizontal(|ui| {
        ui.strong("Entity:");
        let entity_name = name_query
            .get(primary)
            .map(|n| n.as_str().to_string())
            .unwrap_or_else(|_| format!("{:?}", primary));
        ui.label(&entity_name);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(format!("v{}", primary.index()))
                    .color(egui::Color32::from_rgb(140, 140, 140))
                    .small(),
            );
        });
    });
    ui.separator();

    // ---- Transform ----
    egui::CollapsingHeader::new("Transform")
        .default_open(true)
        .show(ui, |ui| {
            if let Ok(transform) = transform_query.get(primary) {
                draw_transform_editor(ui, transform);
            } else {
                ui.label(
                    egui::RichText::new("(no Transform component)")
                        .color(egui::Color32::from_rgb(140, 140, 140)),
                );
                if ui.button("+ Add Transform").clicked() {
                    info!("Add Transform (TODO)");
                }
            }
        });

    // ---- Visibility ----
    egui::CollapsingHeader::new("Visibility")
        .default_open(true)
        .show(ui, |ui| {
            if let Ok(visibility) = visibility_query.get(primary) {
                ui.label(format!("Current: {:?}", visibility));
            } else {
                ui.label(
                    egui::RichText::new("(no Visibility component)")
                        .color(egui::Color32::from_rgb(140, 140, 140)),
                );
            }
        });

    // ---- Mesh / Material ----
    egui::CollapsingHeader::new("Mesh / Material")
        .default_open(false)
        .show(ui, |ui| {
            ui.label("Mesh: (none)");
            ui.label("Material: (none)");
            if ui.button("+ Add Mesh Renderer").clicked() {
                info!("Add Mesh (TODO)");
            }
        });

    // ---- Physics ----
    egui::CollapsingHeader::new("Physics")
        .default_open(false)
        .show(ui, |ui| {
            ui.label("RigidBody: None");
            ui.label("Collider: None");
            if ui.button("+ Add RigidBody").clicked() {
                info!("Add RigidBody (TODO)");
            }
        });

    // ---- Audio ----
    egui::CollapsingHeader::new("Audio")
        .default_open(false)
        .show(ui, |ui| {
            ui.label("Audio Source: None");
            if ui.button("+ Add Audio Source").clicked() {
                info!("Add Audio (TODO)");
            }
        });

    // ---- Scripting ----
    egui::CollapsingHeader::new("Scripting")
        .default_open(false)
        .show(ui, |ui| {
            ui.label("Scripts: (none attached)");
            if ui.button("+ Attach Script").clicked() {
                info!("Attach Script (TODO)");
            }
        });

    // ---- AI ----
    egui::CollapsingHeader::new("AI")
        .default_open(false)
        .show(ui, |ui| {
            ui.label("AI Agent: None");
            if ui.button("+ Add AI Agent").clicked() {
                info!("Add AI Agent (TODO)");
            }
        });

    // ---- Footer ----
    ui.separator();
    ui.horizontal(|ui| {
        if ui.button("Add Component").clicked() {
            info!("Add Component (TODO)");
        }
    });
}

/// Draw a Transform editor with Translation / Rotation / Scale fields.
fn draw_transform_editor(ui: &mut egui::Ui, transform: &Transform) {
    let translation = transform.translation;
    let rotation = transform.rotation.to_euler(bevy::math::EulerRot::XYZ);
    let scale = transform.scale;

    ui.label("Translation");
    draw_vec3_row(ui, translation);
    ui.add_space(2.0);

    ui.label("Rotation (XYZ, radians)");
    draw_vec3_row(ui, Vec3::new(rotation.0, rotation.1, rotation.2));
    ui.add_space(2.0);

    ui.label("Scale");
    draw_vec3_row(ui, scale);
}

/// Draw a 3-component vector editor as three labeled DragValue inputs.
fn draw_vec3_row(ui: &mut egui::Ui, value: Vec3) {
    let mut x = value.x;
    let mut y = value.y;
    let mut z = value.z;
    ui.horizontal(|ui| {
        ui.add(
            egui::DragValue::new(&mut x)
                .speed(0.1)
                .range(-1000.0..=1000.0)
                .prefix("X: ")
                .fixed_decimals(2),
        );
        ui.add(
            egui::DragValue::new(&mut y)
                .speed(0.1)
                .range(-1000.0..=1000.0)
                .prefix("Y: ")
                .fixed_decimals(2),
        );
        ui.add(
            egui::DragValue::new(&mut z)
                .speed(0.1)
                .range(-1000.0..=1000.0)
                .prefix("Z: ")
                .fixed_decimals(2),
        );
    });
}
