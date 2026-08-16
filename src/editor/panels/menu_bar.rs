//! Top menu bar (File / Edit / View / Asset / Build / Help).

use crate::editor::resources::{CommandHistory, ProjectResource, EditorSettings};
use crate::editor::state::EditorState;
use bevy::prelude::*;
use bevy_egui::egui;

/// The menu bar system. Draws the top-of-window menu bar with all the
/// standard entries (File / Edit / View / Asset / Build / Help).
pub fn draw_system(
    mut ctxs: bevy_egui::EguiContexts,
    panel_visibility: Res<crate::editor::panels::PanelVisibility>,
    mut panel_visibility_mut: ResMut<crate::editor::panels::PanelVisibility>,
    project: Res<ProjectResource>,
    history: Res<CommandHistory>,
    current_state: Res<State<EditorState>>,
    mut next_state: ResMut<NextState<EditorState>>,
    mut settings: ResMut<EditorSettings>,
) {
    let Some(ctx) = ctxs.ctx_mut().into() else {
        return;
    };

    // Clone visibility flags so we can mutate them through a closure.
    let mut vis = panel_visibility.clone();
    let mut state_to_set: Option<EditorState> = None;

    egui::TopBottomPanel::top("menu_bar")
        .exact_height(28.0)
        .show(ctx, |ui| {
            use egui::*;

            // ----- File menu -----
            ui.menu_button("File", |ui| {
                if ui.add(Button::new("New Project...")).clicked() {
                    info!("File → New Project (TODO)");
                    ui.close_menu();
                }
                if ui.add(Button::new("Open Project...")).clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Bevy project", &["toml"])
                        .pick_folder()
                    {
                        info!("Opening project at {:?}", path);
                    }
                    ui.close_menu();
                }
                ui.separator();
                if ui.add(Button::new("Save Scene")).clicked() {
                    info!("File → Save Scene");
                    ui.close_menu();
                }
                if ui.add(Button::new("Save Scene As...")).clicked() {
                    info!("File → Save Scene As");
                    ui.close_menu();
                }
                if ui.add(Button::new("Load Scene...")).clicked() {
                    info!("File → Load Scene");
                    ui.close_menu();
                }
                ui.separator();
                if ui.add(Button::new("Recent Scenes")).clicked() {
                    ui.close_menu();
                }
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
                if ui.add(Button::new("Exit")).clicked() {
                    std::process::exit(0);
                }
            });

            // ----- Edit menu -----
            ui.menu_button("Edit", |ui| {
                let undo_label = history
                    .next_undo_description()
                    .map(|s| format!("Undo {s}"))
                    .unwrap_or_else(|| "Undo".to_string());
                let redo_label = history
                    .next_redo_description()
                    .map(|s| format!("Redo {s}"))
                    .unwrap_or_else(|| "Redo".to_string());
                if ui.add(Button::new(undo_label)).clicked() {
                    info!("Edit → Undo (TODO: route through history)");
                    ui.close_menu();
                }
                if ui.add(Button::new(redo_label)).clicked() {
                    info!("Edit → Redo (TODO)");
                    ui.close_menu();
                }
                ui.separator();
                if ui.add(Button::new("Duplicate Selection")).clicked() {
                    info!("Edit → Duplicate (TODO)");
                    ui.close_menu();
                }
                if ui.add(Button::new("Delete Selection")).clicked() {
                    info!("Edit → Delete (TODO)");
                    ui.close_menu();
                }
                ui.separator();
                if ui.add(Button::new("Preferences...")).clicked() {
                    vis.settings_open = !vis.settings_open;
                    ui.close_menu();
                }
            });

            // ----- View menu -----
            ui.menu_button("View", |ui| {
                ui.checkbox(&mut vis.viewport, "Viewport");
                ui.checkbox(&mut vis.scene_hierarchy, "Scene Hierarchy");
                ui.checkbox(&mut vis.inspector, "Inspector");
                ui.checkbox(&mut vis.asset_browser, "Asset Browser");
                ui.checkbox(&mut vis.console, "Console");
                ui.separator();
                if ui.add(Button::new("Toggle Theme")).clicked() {
                    settings.theme = match settings.theme {
                        crate::editor::resources::ThemeKind::Dark => {
                            crate::editor::resources::ThemeKind::Light
                        }
                        crate::editor::resources::ThemeKind::Light => {
                            crate::editor::resources::ThemeKind::Dark
                        }
                    };
                    ui.close_menu();
                }
            });

            // ----- Asset menu -----
            ui.menu_button("Asset", |ui| {
                if ui.add(Button::new("Import...")).clicked() {
                    info!("Asset → Import (TODO)");
                    ui.close_menu();
                }
                if ui.add(Button::new("Re-scan assets/")).clicked() {
                    info!("Asset → Re-scan");
                    ui.close_menu();
                }
                ui.separator();
                if ui.add(Button::new("Compress Textures")).clicked() {
                    info!("Asset → Compress Textures (TODO)");
                    ui.close_menu();
                }
                if ui.add(Button::new("Compress Audio")).clicked() {
                    info!("Asset → Compress Audio (TODO)");
                    ui.close_menu();
                }
                if ui.add(Button::new("Organize Models")).clicked() {
                    info!("Asset → Organize Models (TODO)");
                    ui.close_menu();
                }
            });

            // ----- Build menu -----
            ui.menu_button("Build", |ui| {
                if ui.add(Button::new("Build Debug")).clicked() {
                    info!("Build → Debug (TODO)");
                    ui.close_menu();
                }
                if ui.add(Button::new("Build Release")).clicked() {
                    info!("Build → Release (TODO)");
                    ui.close_menu();
                }
                if ui.add(Button::new("Run")).clicked() {
                    state_to_set = Some(EditorState::Playing);
                    ui.close_menu();
                }
                ui.separator();
                if ui.add(Button::new("Build for Windows")).clicked() {
                    info!("Build → Windows (TODO)");
                    ui.close_menu();
                }
                if ui.add(Button::new("Build for macOS")).clicked() {
                    info!("Build → macOS (TODO)");
                    ui.close_menu();
                }
                if ui.add(Button::new("Build for Linux")).clicked() {
                    info!("Build → Linux (TODO)");
                    ui.close_menu();
                }
                if ui.add(Button::new("Build for Web")).clicked() {
                    info!("Build → Web (TODO)");
                    ui.close_menu();
                }
            });

            // ----- Help menu -----
            ui.menu_button("Help", |ui| {
                if ui.add(Button::new("About Bevy Editor")).clicked() {
                    vis.about_open = true;
                    ui.close_menu();
                }
                if ui.add(Button::new("Documentation")).clicked() {
                    let _ = open::that("https://bevyengine.org/learn/book/");
                    ui.close_menu();
                }
                if ui.add(Button::new("Bevy Examples")).clicked() {
                    let _ = open::that("https://github.com/bevyengine/bevy/tree/main/examples");
                    ui.close_menu();
                }
                if ui.add(Button::new("Report a Bug")).clicked() {
                    let _ = open::that("https://github.com/salom600/engin1/issues");
                    ui.close_menu();
                }
            });

            // Right-aligned state indicator
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let state_label = match current_state.get() {
                    EditorState::Loading => "Loading",
                    EditorState::Editing => "Editing",
                    EditorState::Playing => "Playing",
                    EditorState::Paused => "Paused",
                };
                let (color, _) = match current_state.get() {
                    EditorState::Loading => (egui::Color32::from_rgb(153, 153, 153), ""),
                    EditorState::Editing => (egui::Color32::from_rgb(40, 180, 80), ""),
                    EditorState::Playing => (egui::Color32::from_rgb(0, 122, 204), ""),
                    EditorState::Paused => (egui::Color32::from_rgb(204, 153, 0), ""),
                };
                let dot = "●";
                ui.colored_label(color, format!("{dot} {state_label}"));
                ui.separator();
                ui.label(format!("{}", env!("CARGO_PKG_VERSION")));
            });
        });

    // Apply visibility changes back to the resource.
    if vis != *panel_visibility {
        *panel_visibility_mut = vis;
    }

    // Apply state changes.
    if let Some(new_state) = state_to_set {
        next_state.set(new_state);
    }
}
