//! Add Component dialog.
//!
//! A floating window that lets the user add real ECS components to the
//! selected entity: physics (RigidBody, Collider), scripts, AI agents, etc.

use crate::editor::components::{AIAgent, ScriptComponent};
use crate::editor::panels::ViewportState;
use crate::editor::state::Selection;
use bevy::prelude::*;
use bevy_egui::egui;
use bevy_rapier3d::prelude::{Collider, RigidBody};

/// The list of addable component categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentCategory {
    /// Physics components (RigidBody, Collider).
    Physics,
    /// Scripting (Lua script attachment).
    Scripting,
    /// AI agent.
    AI,
    /// Rendering (mesh, material — already added via Add menu, listed for reference).
    Rendering,
}

/// Draw the Add Component floating window.
pub fn draw_window(
    ctx: &egui::Context,
    state: &mut ViewportState,
    selection: &Selection,
    pending: &mut crate::editor::panels::PendingActions,
) {
    if !state.add_component_open {
        return;
    }

    let Some(_entity) = selection.primary else {
        state.add_component_open = false;
        return;
    };

    egui::Window::new("Add Component")
        .resizable(true)
        .collapsible(false)
        .default_width(360.0)
        .default_height(420.0)
        .show(ctx, |ui| {
            ui.label("Select a component to add to the selected entity:");
            ui.separator();

            // ---- Physics section ----
            ui.add_space(4.0);
            ui.heading("Physics");
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                if ui
                    .button("RigidBody (Dynamic)")
                    .on_hover_text("Affected by gravity and forces")
                    .clicked()
                {
                    info!("Add RigidBody (Dynamic) — TODO: queue via pending actions");
                    state.add_component_open = false;
                }
                if ui
                    .button("RigidBody (Static)")
                    .on_hover_text("Doesn't move, but collides")
                    .clicked()
                {
                    info!("Add RigidBody (Static) — TODO");
                    state.add_component_open = false;
                }
                if ui
                    .button("RigidBody (Kinematic)")
                    .on_hover_text("Moved manually, no forces")
                    .clicked()
                {
                    info!("Add RigidBody (Kinematic) — TODO");
                    state.add_component_open = false;
                }
                if ui
                    .button("Collider (Box)")
                    .on_hover_text("Box-shaped collision volume")
                    .clicked()
                {
                    info!("Add Collider (Box) — TODO");
                    state.add_component_open = false;
                }
                if ui
                    .button("Collider (Sphere)")
                    .on_hover_text("Spherical collision volume")
                    .clicked()
                {
                    info!("Add Collider (Sphere) — TODO");
                    state.add_component_open = false;
                }
            });

            ui.add_space(8.0);

            // ---- Scripting section ----
            ui.heading("Scripting");
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                if ui
                    .button("Lua Script")
                    .on_hover_text("Attach a Lua script to this entity")
                    .clicked()
                {
                    info!("Add Lua Script — TODO: open script picker");
                    state.add_component_open = false;
                }
                if ui
                    .button("New Script")
                    .on_hover_text("Create a new Lua script file")
                    .clicked()
                {
                    info!("Create new script — TODO: open script editor");
                    state.add_component_open = false;
                }
            });

            ui.add_space(8.0);

            // ---- AI section ----
            ui.heading("Artificial Intelligence");
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                if ui
                    .button("AI Agent")
                    .on_hover_text("Utility AI agent (big-brain)")
                    .clicked()
                {
                    info!("Add AI Agent — TODO");
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
