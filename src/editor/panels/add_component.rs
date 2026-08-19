//! Add Component dialog.
//!
//! A floating window that lets the user add real ECS components to the
//! selected entity: physics (RigidBody, Collider), scripts, AI agents, etc.
//!
//! All buttons send real `AddComponentRequest` events that are processed
//! by the `handle_add_component_requests` system.

use crate::editor::components::{
    AddComponentRequest, ColliderShape, RigidBodyType,
};
use crate::editor::panels::{PendingActions, ViewportState};
use crate::editor::state::Selection;
use bevy::prelude::*;
use bevy_egui::egui;

/// Draw the Add Component floating window.
pub fn draw_window(
    ctx: &egui::Context,
    state: &mut ViewportState,
    selection: &Selection,
    pending: &mut PendingActions,
) {
    if !state.add_component_open {
        return;
    }

    let Some(entity) = selection.primary else {
        state.add_component_open = false;
        return;
    };

    egui::Window::new("Add Component")
        .resizable(true)
        .collapsible(false)
        .default_width(380.0)
        .default_height(480.0)
        .open(&mut state.add_component_open)
        .show(ctx, |ui| {
            ui.label(format!("Adding component to entity: {:?}", entity));
            ui.separator();
            ui.label("Select a component to add:");
            ui.separator();

            // ---- Physics section ----
            ui.add_space(4.0);
            ui.heading("Physics");
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                if ui
                    .button("RigidBody (Dynamic)")
                    .on_hover_text("Affected by gravity and forces — falls, bounces, etc.")
                    .clicked()
                {
                    pending.add_components.push(AddComponentRequest::RigidBody {
                        entity,
                        body_type: RigidBodyType::Dynamic,
                    });
                    state.add_component_open = false;
                }
                if ui
                    .button("RigidBody (Static)")
                    .on_hover_text("Immovable; collides but doesn't move (walls, floors)")
                    .clicked()
                {
                    pending.add_components.push(AddComponentRequest::RigidBody {
                        entity,
                        body_type: RigidBodyType::Static,
                    });
                    state.add_component_open = false;
                }
                if ui
                    .button("RigidBody (Kinematic)")
                    .on_hover_text("Moved manually via code; not affected by forces")
                    .clicked()
                {
                    pending.add_components.push(AddComponentRequest::RigidBody {
                        entity,
                        body_type: RigidBodyType::Kinematic,
                    });
                    state.add_component_open = false;
                }
            });
            ui.horizontal_wrapped(|ui| {
                if ui
                    .button("Collider (Box)")
                    .on_hover_text("Box-shaped collision volume")
                    .clicked()
                {
                    pending.add_components.push(AddComponentRequest::Collider {
                        entity,
                        shape: ColliderShape::Box,
                    });
                    state.add_component_open = false;
                }
                if ui
                    .button("Collider (Sphere)")
                    .on_hover_text("Spherical collision volume")
                    .clicked()
                {
                    pending.add_components.push(AddComponentRequest::Collider {
                        entity,
                        shape: ColliderShape::Sphere,
                    });
                    state.add_component_open = false;
                }
            });

            ui.add_space(8.0);

            // ---- Scripting section ----
            ui.heading("Scripting");
            ui.separator();
            ui.label("Attach a script to make the entity behave during play mode:");
            ui.horizontal_wrapped(|ui| {
                if ui
                    .button("📄 Lua Script")
                    .on_hover_text("Attach an existing .lua script")
                    .clicked()
                {
                    if let Some(file) = rfd::FileDialog::new()
                        .add_filter("Lua script", &["lua"])
                        .set_directory("assets/scripts")
                        .pick_file()
                    {
                        let path = file.to_string_lossy().to_string();
                        pending.add_components.push(AddComponentRequest::LuaScript {
                            entity,
                            path,
                        });
                        state.add_component_open = false;
                    }
                }
                if ui
                    .button("🦀 Rhai Script")
                    .on_hover_text("Attach a .rhai script (Rust-like syntax)")
                    .clicked()
                {
                    if let Some(file) = rfd::FileDialog::new()
                        .add_filter("Rhai script", &["rhai"])
                        .set_directory("assets/scripts")
                        .pick_file()
                    {
                        let path = file.to_string_lossy().to_string();
                        pending.add_components.push(AddComponentRequest::RhaiScript {
                            entity,
                            path,
                        });
                        state.add_component_open = false;
                    }
                }
            });
            ui.label(
                egui::RichText::new(
                    "Rhai is a Rust-like scripting language — similar syntax to Rust but interpreted at runtime.",
                )
                .color(egui::Color32::from_rgb(140, 140, 140))
                .small(),
            );

            ui.add_space(8.0);

            // ---- AI section ----
            ui.heading("Artificial Intelligence");
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                if ui
                    .button("🤖 AI Agent")
                    .on_hover_text("Utility AI agent (big-brain) — makes decisions")
                    .clicked()
                {
                    pending.add_components.push(AddComponentRequest::AIAgent {
                        entity,
                        name: format!("Agent {:?}", entity),
                    });
                    state.add_component_open = false;
                }
            });

            ui.add_space(8.0);

            // ---- Rendering section ----
            ui.heading("Rendering");
            ui.separator();
            ui.label("(Use the Add menu in the toolbar to spawn entities with meshes)");
            ui.label("Components: Mesh, StandardMaterial, Visibility");

            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Close").clicked() {
                    state.add_component_open = false;
                }
            });
        });
}
