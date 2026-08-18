//! Top toolbar (Play / Pause / Stop / Save / Load / Add).
//!
//! Exposes a single [`draw`] function that creates a
//! `TopBottomPanel::top("toolbar")`. Called by the master layout system.

use crate::editor::components::{LoadSceneRequest, SaveSceneRequest, SpawnRequest};
use crate::editor::panels::PendingActions;
use crate::editor::resources::ProjectResource;
use crate::editor::state::EditorState;
use bevy::prelude::*;
use bevy_egui::egui;

/// Draw the toolbar as a `TopBottomPanel::top("toolbar")`.
///
/// Must be called AFTER `menu_bar::draw` and BEFORE side/central panels.
pub fn draw(
    ctx: &egui::Context,
    current_state: &State<EditorState>,
    next_state: &mut NextState<EditorState>,
    project: &ProjectResource,
    pending: &mut PendingActions,
) {
    egui::TopBottomPanel::top("toolbar")
        .exact_height(38.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().button_padding = (8.0, 3.0).into();
                ui.spacing_mut().item_spacing.x = 4.0;

                let playing = matches!(current_state.get(), EditorState::Playing);
                let paused = matches!(current_state.get(), EditorState::Paused);
                let editing = matches!(current_state.get(), EditorState::Editing);

                // ---- Play / Pause / Stop group ----
                let play_label = if paused { "▶ Resume" } else { "▶ Play" };
                let play_bg = if playing {
                    egui::Color32::from_rgb(40, 180, 80)
                } else {
                    egui::Color32::from_rgb(45, 130, 60)
                };
                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new(play_label)
                                .color(egui::Color32::WHITE)
                                .strong(),
                        )
                        .fill(play_bg)
                        .min_size(egui::vec2(90.0, 0.0)),
                    )
                    .on_hover_text("Play / Pause (F5)")
                    .clicked()
                {
                    if editing || paused {
                        next_state.set(EditorState::Playing);
                    }
                }

                let pause_bg = if paused {
                    egui::Color32::from_rgb(204, 153, 0)
                } else {
                    egui::Color32::from_rgb(100, 80, 0)
                };
                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("⏸ Pause")
                                .color(egui::Color32::WHITE)
                                .strong(),
                        )
                        .fill(pause_bg)
                        .min_size(egui::vec2(80.0, 0.0)),
                    )
                    .on_hover_text("Pause (F6)")
                    .clicked()
                {
                    if playing {
                        next_state.set(EditorState::Paused);
                    }
                }

                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("⏹ Stop")
                                .color(egui::Color32::WHITE)
                                .strong(),
                        )
                        .fill(egui::Color32::from_rgb(180, 40, 40))
                        .min_size(egui::vec2(70.0, 0.0)),
                    )
                    .on_hover_text("Stop play mode")
                    .clicked()
                {
                    if !editing {
                        next_state.set(EditorState::Editing);
                    }
                }

                ui.separator();

                // ---- Save / Load (real actions!) ----
                if ui
                    .button("💾 Save")
                    .on_hover_text("Save scene (Ctrl+S)")
                    .clicked()
                {
                    pending.save = true;
                }
                if ui.button("📂 Load").on_hover_text("Load scene").clicked() {
                    if let Some(file) = rfd::FileDialog::new()
                        .add_filter("Bevy scene", &["scn.ron", "ron"])
                        .pick_file()
                    {
                        pending.load = Some(file);
                    }
                }

                ui.separator();

                // ---- Add entity menu (real spawning!) ----
                ui.menu_button("➕ Add", |ui| {
                    if ui.button("Cube").clicked() {
                        pending.spawns.push(SpawnRequest::Cube);
                        ui.close_menu();
                    }
                    if ui.button("Sphere").clicked() {
                        pending.spawns.push(SpawnRequest::Sphere);
                        ui.close_menu();
                    }
                    if ui.button("Plane").clicked() {
                        pending.spawns.push(SpawnRequest::Plane);
                        ui.close_menu();
                    }
                    if ui.button("Camera").clicked() {
                        pending.spawns.push(SpawnRequest::Camera);
                        ui.close_menu();
                    }
                    if ui.button("Directional Light").clicked() {
                        pending.spawns.push(SpawnRequest::DirectionalLight);
                        ui.close_menu();
                    }
                    if ui.button("Point Light").clicked() {
                        pending.spawns.push(SpawnRequest::PointLight);
                        ui.close_menu();
                    }
                    if ui.button("Empty Entity").clicked() {
                        pending.spawns.push(SpawnRequest::Empty);
                        ui.close_menu();
                    }
                });

                // Right-aligned project info
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Bevy {}", project.bevy_version));
                    ui.separator();
                    ui.label(format!("📁 {}", project.name));
                });
            });
        });
}
