//! Top menu bar (File / Edit / View / Asset / Build / Help).
//!
//! This module exposes a single [`draw`] function that creates a
//! `TopBottomPanel::top` on the given egui context. It is called by the
//! master [`crate::editor::layout::draw_editor_ui`] system.

use crate::editor::panels::PanelVisibility;
use crate::editor::resources::{CommandHistory, EditorSettings, ProjectResource, ThemeKind};
use crate::editor::state::EditorState;
use bevy::prelude::*;
use bevy_egui::egui;

/// Draw the menu bar as a `TopBottomPanel::top("menu_bar")`.
///
/// Must be called BEFORE any side panels or the central panel.
#[allow(clippy::too_many_arguments)]
pub fn draw(
    ctx: &egui::Context,
    panel_visibility: &mut PanelVisibility,
    project: &ProjectResource,
    history: &CommandHistory,
    current_state: &State<EditorState>,
    next_state: &mut NextState<EditorState>,
    settings: &mut EditorSettings,
) {
    let mut vis = panel_visibility.clone();
    let mut state_to_set: Option<EditorState> = None;

    egui::TopBottomPanel::top("menu_bar")
        .exact_height(26.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;

                // ----- File menu -----
                ui.menu_button("File", |ui| {
                    if ui.button("New Project...").clicked() {
                        info!("File → New Project (TODO)");
                        ui.close_menu();
                    }
                    if ui.button("Open Project...").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Bevy project", &["toml"])
                            .pick_folder()
                        {
                            info!("Opening project at {:?}", path);
                        }
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Save Scene\tCtrl+S").clicked() {
                        info!("File → Save Scene");
                        ui.close_menu();
                    }
                    if ui.button("Save Scene As...").clicked() {
                        info!("File → Save Scene As");
                        ui.close_menu();
                    }
                    if ui.button("Load Scene...").clicked() {
                        info!("File → Load Scene");
                        ui.close_menu();
                    }
                    ui.separator();
                    ui.menu_button("Recent Scenes", |ui| {
                        if project.recent_scenes.is_empty() {
                            ui.label("(none)");
                        } else {
                            for scene in &project.recent_scenes {
                                if ui.button(scene.display().to_string()).clicked() {
                                    info!("Loading recent scene {:?}", scene);
                                    ui.close_menu();
                                }
                            }
                        }
                    });
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
                    }
                });

                // ----- Edit menu -----
                ui.menu_button("Edit", |ui| {
                    let undo_label = history
                        .next_undo_description()
                        .map(|s| format!("Undo {s}\tCtrl+Z"))
                        .unwrap_or_else(|| "Undo\tCtrl+Z".to_string());
                    let redo_label = history
                        .next_redo_description()
                        .map(|s| format!("Redo {s}\tCtrl+Y"))
                        .unwrap_or_else(|| "Redo\tCtrl+Y".to_string());
                    if ui.add_enabled(false, egui::Button::new(undo_label)).clicked() {
                        info!("Edit → Undo (TODO)");
                        ui.close_menu();
                    }
                    if ui.add_enabled(false, egui::Button::new(redo_label)).clicked() {
                        info!("Edit → Redo (TODO)");
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Duplicate Selection\tCtrl+D").clicked() {
                        info!("Edit → Duplicate (TODO)");
                        ui.close_menu();
                    }
                    if ui.button("Delete Selection\tDel").clicked() {
                        info!("Edit → Delete (TODO)");
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Preferences...").clicked() {
                        vis.settings_open = !vis.settings_open;
                        ui.close_menu();
                    }
                });

                // ----- View menu -----
                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut vis.viewport, "Viewport");
                    ui.checkbox(&mut vis.scene_hierarchy, "Scene Hierarchy");
                    ui.checkbox(&mut vis.inspector, "Inspector");
                    ui.separator();
                    ui.label("Theme:");
                    ui.horizontal(|ui| {
                        if ui
                            .radio_value(&mut settings.theme, ThemeKind::Dark, "Dark")
                            .clicked()
                        {}
                        if ui
                            .radio_value(&mut settings.theme, ThemeKind::Light, "Light")
                            .clicked()
                        {}
                    });
                });

                // ----- Asset menu -----
                ui.menu_button("Asset", |ui| {
                    if ui.button("Import...").clicked() {
                        info!("Asset → Import (TODO)");
                        ui.close_menu();
                    }
                    if ui.button("Re-scan assets/").clicked() {
                        info!("Asset → Re-scan");
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Compress Textures").clicked() {
                        info!("Asset → Compress Textures (TODO)");
                        ui.close_menu();
                    }
                    if ui.button("Compress Audio").clicked() {
                        info!("Asset → Compress Audio (TODO)");
                        ui.close_menu();
                    }
                    if ui.button("Organize Models").clicked() {
                        info!("Asset → Organize Models (TODO)");
                        ui.close_menu();
                    }
                });

                // ----- Build menu -----
                ui.menu_button("Build", |ui| {
                    if ui.button("Build Debug").clicked() {
                        info!("Build → Debug (TODO)");
                        ui.close_menu();
                    }
                    if ui.button("Build Release").clicked() {
                        info!("Build → Release (TODO)");
                        ui.close_menu();
                    }
                    if ui.button("Run\tF5").clicked() {
                        state_to_set = Some(EditorState::Playing);
                        ui.close_menu();
                    }
                    ui.separator();
                    ui.menu_button("Build for Platform", |ui| {
                        if ui.button("Windows").clicked() {
                            info!("Build → Windows (TODO)");
                            ui.close_menu();
                        }
                        if ui.button("macOS").clicked() {
                            info!("Build → macOS (TODO)");
                            ui.close_menu();
                        }
                        if ui.button("Linux").clicked() {
                            info!("Build → Linux (TODO)");
                            ui.close_menu();
                        }
                        if ui.button("Web").clicked() {
                            info!("Build → Web (TODO)");
                            ui.close_menu();
                        }
                    });
                });

                // ----- Help menu -----
                ui.menu_button("Help", |ui| {
                    if ui.button("About Bevy Editor").clicked() {
                        vis.about_open = true;
                        ui.close_menu();
                    }
                    if ui.button("Documentation").clicked() {
                        let _ = open::that("https://bevyengine.org/learn/book/");
                        ui.close_menu();
                    }
                    if ui.button("Bevy Examples").clicked() {
                        let _ = open::that("https://github.com/bevyengine/bevy/tree/main/examples");
                        ui.close_menu();
                    }
                    if ui.button("Report a Bug").clicked() {
                        let _ = open::that("https://github.com/salom600/engin1/issues");
                        ui.close_menu();
                    }
                });

                // Right-aligned state indicator
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (state_label, color) = match current_state.get() {
                        EditorState::Loading => ("Loading", egui::Color32::from_rgb(153, 153, 153)),
                        EditorState::Editing => ("Editing", egui::Color32::from_rgb(40, 180, 80)),
                        EditorState::Playing => ("Playing", egui::Color32::from_rgb(0, 122, 204)),
                        EditorState::Paused => ("Paused", egui::Color32::from_rgb(204, 153, 0)),
                    };
                    ui.colored_label(color, format!("● {state_label}"));
                });
            });
        });

    // Apply visibility changes back.
    if vis != *panel_visibility {
        *panel_visibility = vis;
    }
    // Apply state changes.
    if let Some(new_state) = state_to_set {
        next_state.set(new_state);
    }
}
