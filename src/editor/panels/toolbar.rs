//! Top toolbar (Play / Pause / Stop / Step / Save / Load / Build).

use crate::editor::resources::ProjectResource;
use crate::editor::state::EditorState;
use bevy::prelude::*;
use bevy_egui::egui;

/// Toolbar draw system.
pub fn draw_system(
    mut ctxs: bevy_egui::EguiContexts,
    current_state: Res<State<EditorState>>,
    mut next_state: ResMut<NextState<EditorState>>,
    _project: Res<ProjectResource>,
) {
    let Some(ctx) = ctxs.try_ctx_mut() else {
        return;
    };

    egui::TopBottomPanel::top("toolbar")
        .exact_height(40.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().button_padding = (10.0, 4.0).into();

                // Play / Pause / Stop group
                let playing = matches!(current_state.get(), EditorState::Playing);
                let paused = matches!(current_state.get(), EditorState::Paused);
                let editing = matches!(current_state.get(), EditorState::Editing);

                let play_label = if paused { "Resume" } else { "Play" };
                let play_color = if playing {
                    egui::Color32::from_rgb(40, 180, 80)
                } else {
                    egui::Color32::from_rgb(40, 140, 60)
                };
                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new(format!("▶ {play_label}"))
                                .color(egui::Color32::WHITE),
                        )
                        .fill(play_color),
                    )
                    .clicked()
                {
                    if editing || paused {
                        next_state.set(EditorState::Playing);
                    }
                }

                let pause_color = if paused {
                    egui::Color32::from_rgb(204, 153, 0)
                } else {
                    egui::Color32::from_rgb(160, 120, 0)
                };
                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("⏸ Pause").color(egui::Color32::WHITE),
                        )
                        .fill(pause_color),
                    )
                    .clicked()
                {
                    if playing {
                        next_state.set(EditorState::Paused);
                    }
                }

                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("⏹ Stop").color(egui::Color32::WHITE),
                        )
                        .fill(egui::Color32::from_rgb(204, 0, 0)),
                    )
                    .clicked()
                {
                    if !editing {
                        next_state.set(EditorState::Editing);
                    }
                }

                if ui.button("⏭ Step").clicked() {
                    info!("Toolbar → Step (TODO: advance one frame)");
                }

                ui.separator();

                // Save / Load group
                if ui.button("💾 Save").clicked() {
                    info!("Toolbar → Save (TODO)");
                }
                if ui.button("📂 Load").clicked() {
                    info!("Toolbar → Load (TODO)");
                }

                ui.separator();

                // Edit tools
                if ui.button("↩ Undo").clicked() {
                    info!("Toolbar → Undo (TODO)");
                }
                if ui.button("↪ Redo").clicked() {
                    info!("Toolbar → Redo (TODO)");
                }

                ui.separator();

                // Build
                if ui.button("🔨 Build Debug").clicked() {
                    info!("Toolbar → Build Debug (TODO)");
                }
                if ui.button("🚀 Build Release").clicked() {
                    info!("Toolbar → Build Release (TODO)");
                }

                ui.separator();

                // Quick-add primitives
                ui.menu_button("+ Add", |ui| {
                    if ui.button("Cube").clicked() {
                        info!("Add → Cube (TODO)");
                    }
                    if ui.button("Sphere").clicked() {
                        info!("Add → Sphere (TODO)");
                    }
                    if ui.button("Plane").clicked() {
                        info!("Add → Plane (TODO)");
                    }
                    if ui.button("Camera").clicked() {
                        info!("Add → Camera (TODO)");
                    }
                    if ui.button("Light").clicked() {
                        info!("Add → Light (TODO)");
                    }
                    if ui.button("Empty Entity").clicked() {
                        info!("Add → Empty (TODO)");
                    }
                });

                // Right-aligned project info
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Project: {}", _project.name));
                    ui.separator();
                    ui.label(format!("Bevy {}", _project.bevy_version));
                });
            });
        });
}
