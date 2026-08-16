//! Component inspector panel.
//!
//! Shows the components of the currently selected entity and lets the user
//! edit them. For known component types (Transform, Visibility, Material,
//! etc.) we provide hand-written editors; for everything else we fall back
//! to a generic reflection-based editor.

use crate::editor::resources::EditorSettings;
use crate::editor::state::Selection;
use bevy::prelude::*;
use bevy_egui::egui;

/// Inspector draw system.
pub fn draw_system(
    mut ctxs: bevy_egui::EguiContexts,
    selection: Res<Selection>,
    _settings: Res<EditorSettings>,
    transform_query: Query<&Transform>,
    visibility_query: Query<&Visibility>,
    name_query: Query<&Name>,
) {
    let Some(ctx) = ctxs.ctx_mut().into() else {
        return;
    };

    egui::SidePanel::right("inspector")
        .default_width(320.0)
        .width_range(220.0..=520.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.strong("Inspector");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("Add ▾");
                });
            });
            ui.separator();

            let Some(primary) = selection.primary else {
                ui.label("No entity selected.");
                ui.label("Select an entity in the viewport or hierarchy to inspect it.");
                return;
            };

            // Header: entity ID + name
            ui.horizontal(|ui| {
                ui.strong(format!("Entity"));
                ui.label(format!("{:?}", primary));
            });
            if let Ok(name) = name_query.get(primary) {
                ui.label(format!("Name: {}", name.as_str()));
            } else {
                ui.label("Name: (unnamed)");
            }
            ui.separator();

            // Transform editor
            egui::CollapsingHeader::new("Transform")
                .default_open(true)
                .show(ui, |ui| {
                    if let Ok(transform) = transform_query.get(primary) {
                        draw_transform_editor(ui, transform);
                    } else {
                        ui.label("(no Transform component)");
                    }
                });

            // Visibility editor
            egui::CollapsingHeader::new("Visibility")
                .default_open(true)
                .show(ui, |ui| {
                    if let Ok(visibility) = visibility_query.get(primary) {
                        draw_visibility_editor(ui, visibility);
                    } else {
                        ui.label("(no Visibility component)");
                    }
                });

            // Other known components
            egui::CollapsingHeader::new("Mesh / Material")
                .default_open(false)
                .show(ui, |ui| {
                    ui.label("Mesh: (no mesh renderer)");
                    ui.label("Material: (no material)");
                    if ui.button("Add Mesh").clicked() {
                        info!("Add Mesh (TODO)");
                    }
                });

            egui::CollapsingHeader::new("Physics")
                .default_open(false)
                .show(ui, |ui| {
                    ui.label("RigidBody: None");
                    ui.label("Collider: None");
                    if ui.button("Add RigidBody").clicked() {
                        info!("Add RigidBody (TODO)");
                    }
                });

            egui::CollapsingHeader::new("Audio")
                .default_open(false)
                .show(ui, |ui| {
                    ui.label("Audio Source: None");
                    if ui.button("Add Audio Source").clicked() {
                        info!("Add Audio (TODO)");
                    }
                });

            egui::CollapsingHeader::new("Scripting")
                .default_open(false)
                .show(ui, |ui| {
                    ui.label("Scripts: (none attached)");
                    if ui.button("Attach Script").clicked() {
                        info!("Attach Script (TODO)");
                    }
                });

            egui::CollapsingHeader::new("AI")
                .default_open(false)
                .show(ui, |ui| {
                    ui.label("AI Agent: None");
                    if ui.button("Add AI Agent").clicked() {
                        info!("Add AI Agent (TODO)");
                    }
                });

            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Add Component").clicked() {
                    info!("Add Component (TODO)");
                }
                if ui.button("Reset").clicked() {
                    info!("Reset entity (TODO)");
                }
            });
        });
}

/// Draw a Transform editor with Translation / Rotation / Scale fields.
fn draw_transform_editor(ui: &mut egui::Ui, transform: &Transform) {
    let translation = transform.translation;
    let rotation = transform.rotation.to_euler(bevy::math::EulerRot::XYZ);
    let scale = transform.scale;

    ui.label("Translation");
    draw_vec3_row(ui, "T", translation);
    ui.label("Rotation (radians, XYZ)");
    draw_vec3_row(ui, "R", Vec3::new(rotation.0, rotation.1, rotation.2));
    ui.label("Scale");
    draw_vec3_row(ui, "S", scale);
}

/// Draw a 3-component vector editor as three float inputs side by side.
fn draw_vec3_row(ui: &mut egui::Ui, label: &str, value: Vec3) {
    let mut x = value.x;
    let mut y = value.y;
    let mut z = value.z;
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(
            egui::DragValue::new(&mut x)
                .speed(0.1)
                .range(-1000.0..=1000.0)
                .prefix("x: "),
        );
        ui.add(
            egui::DragValue::new(&mut y)
                .speed(0.1)
                .range(-1000.0..=1000.0)
                .prefix("y: "),
        );
        ui.add(
            egui::DragValue::new(&mut z)
                .speed(0.1)
                .range(-1000.0..=1000.0)
                .prefix("z: "),
        );
    });
}

/// Draw a Visibility editor with show / hide / hidden buttons.
fn draw_visibility_editor(ui: &mut egui::Ui, visibility: &Visibility) {
    ui.label(format!("Current: {:?}", visibility));
    ui.horizontal(|ui| {
        if ui.button("Visible").clicked() {
            info!("Set Visible (TODO)");
        }
        if ui.button("Hidden").clicked() {
            info!("Set Hidden (TODO)");
        }
    });
}
