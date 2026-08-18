//! Component inspector panel content.
//!
//! Shows the components of the currently selected entity and lets the user
//! edit them. Transform and Name fields are **mutable** — changes write back
//! to the ECS world immediately.

use crate::editor::state::Selection;
use bevy::prelude::*;
use bevy_egui::egui;

/// Draw the inspector content inside the given `ui`.
pub fn draw_content(
    ui: &mut egui::Ui,
    selection: &Selection,
    transform_query: &mut Query<&mut Transform>,
    visibility_query: &Query<&Visibility>,
    name_query: &mut Query<&mut Name>,
    is_edit_mode: bool,
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
                egui::RichText::new(
                    "Select an entity in the\nHierarchy or Viewport to inspect it.",
                )
                .color(egui::Color32::from_rgb(140, 140, 140))
                .small(),
            );
        });
        return;
    };

    // ---- Entity header + editable name ----
    ui.horizontal(|ui| {
        ui.strong("Entity:");
        ui.label(format!("v{}", primary.index()));
    });

    // Editable name field
    if is_edit_mode {
        if let Ok(mut name) = name_query.get_mut(primary) {
            let mut buf = name.as_str().to_string();
            let resp = ui.add(
                egui::TextEdit::singleline(&mut buf)
                    .desired_width(ui.available_width())
                    .hint_text("Entity name..."),
            );
            if resp.changed() {
                name.set(buf);
            }
        }
    } else {
        if let Ok(name) = name_query.get(primary) {
            ui.label(name.as_str());
        }
    }
    ui.separator();

    // ---- Transform (editable in edit mode, read-only in play mode) ----
    egui::CollapsingHeader::new("Transform")
        .default_open(true)
        .show(ui, |ui| {
            if let Ok(mut transform) = transform_query.get_mut(primary) {
                if is_edit_mode {
                    draw_transform_editor(ui, &mut transform);
                } else {
                    draw_transform_readonly(ui, &transform);
                }
            } else {
                ui.label(
                    egui::RichText::new("(no Transform component)")
                        .color(egui::Color32::from_rgb(140, 140, 140)),
                );
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
            ui.label("Mesh: (edit via Add menu)");
            ui.label("Material: (edit via Add menu)");
        });

    // ---- Physics ----
    egui::CollapsingHeader::new("Physics")
        .default_open(false)
        .show(ui, |ui| {
            ui.label("RigidBody: None");
            ui.label("Collider: None");
        });

    // ---- Audio ----
    egui::CollapsingHeader::new("Audio")
        .default_open(false)
        .show(ui, |ui| {
            ui.label("Audio Source: None");
        });

    // ---- Scripting ----
    egui::CollapsingHeader::new("Scripting")
        .default_open(false)
        .show(ui, |ui| {
            ui.label("Scripts: (none attached)");
        });

    // ---- AI ----
    egui::CollapsingHeader::new("AI")
        .default_open(false)
        .show(ui, |ui| {
            ui.label("AI Agent: None");
        });
}

/// Draw an editable Transform editor — writes back to the ECS world.
fn draw_transform_editor(ui: &mut egui::Ui, transform: &mut Transform) {
    let mut t = transform.translation;
    let euler = transform.rotation.to_euler(bevy::math::EulerRot::XYZ);
    let mut r = Vec3::new(euler.0, euler.1, euler.2);
    let mut s = transform.scale;

    let mut changed = false;

    ui.label("Translation");
    changed |= draw_vec3_row(ui, &mut t);
    ui.add_space(2.0);

    ui.label("Rotation (XYZ, radians)");
    changed |= draw_vec3_row(ui, &mut r);
    ui.add_space(2.0);

    ui.label("Scale");
    changed |= draw_vec3_row(ui, &mut s);

    if changed {
        transform.translation = t;
        transform.rotation = Quat::from_euler(bevy::math::EulerRot::XYZ, r.x, r.y, r.z);
        // Guard against zero-scale degenerate matrices
        transform.scale = s.abs().max(Vec3::splat(0.0001));
    }
}

/// Draw a read-only Transform display (during play mode).
fn draw_transform_readonly(ui: &mut egui::Ui, transform: &Transform) {
    ui.label(format!(
        "Translation: ({:.2}, {:.2}, {:.2})",
        transform.translation.x, transform.translation.y, transform.translation.z
    ));
    let euler = transform.rotation.to_euler(bevy::math::EulerRot::XYZ);
    ui.label(format!(
        "Rotation: ({:.2}, {:.2}, {:.2})",
        euler.0, euler.1, euler.2
    ));
    ui.label(format!(
        "Scale: ({:.2}, {:.2}, {:.2})",
        transform.scale.x, transform.scale.y, transform.scale.z
    ));
}

/// Draw a 3-component vector editor. Returns `true` if any value was changed.
fn draw_vec3_row(ui: &mut egui::Ui, v: &mut Vec3) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        let prefixes = ["X: ", "Y: ", "Z: "];
        let vals = [v.x, v.y, v.z];
        for (i, val) in vals.into_iter().enumerate() {
            let mut local = val;
            let resp = ui.add(
                egui::DragValue::new(&mut local)
                    .speed(0.1)
                    .range(-1000.0..=1000.0)
                    .prefix(prefixes[i])
                    .fixed_decimals(2),
            );
            if resp.changed() {
                match i {
                    0 => v.x = local,
                    1 => v.y = local,
                    _ => v.z = local,
                }
                changed = true;
            }
        }
    });
    changed
}
