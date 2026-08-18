//! # Master Editor UI Layout
//!
//! This module contains the single [`draw_editor_ui`] system that draws ALL
//! editor panels in the correct egui order.
//!
//! ## Why a single system?
//!
//! egui requires panels to be drawn in a strict order:
//!
//! 1. **Top panels** (`TopBottomPanel::top`) — drawn first, top-to-bottom
//! 2. **Bottom panels** (`TopBottomPanel::bottom`) — drawn next, bottom-to-top
//! 3. **Side panels** (`SidePanel::left` / `SidePanel::right`) — drawn next
//! 4. **Central panel** (`CentralPanel`) — drawn LAST, fills remaining space
//!
//! If any panel creates a `CentralPanel` before another panel is drawn, the
//! second panel will either overlap or be invisible. This was the root cause
//! of the "jumbled, disorganized, fragmented" UI.
//!
//! By putting all panel drawing in a single system, we guarantee the correct
//! order and avoid Bevy's `B0002` resource conflict panics.
//!
//! ## Layout
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │ Menu Bar (File / Edit / View / Asset / Build / Help)        │  top
//! ├─────────────────────────────────────────────────────────────┤
//! │ Toolbar (▶ Play  ⏸ Pause  ⏹ Stop  💾 Save  ➕ Add)          │  top
//! ├──┬─────────────────────────────────────────────────┬────────┤
//! │  │  Viewport mini-toolbar (Grid / Axes / Gizmo)     │        │
//! │ H│ ─────────────────────────────────────────────── │ Inspect│
//! │ i│                                                 │ or     │
//! │ e│            3D Scene Rendering                   │        │
//! │ r│            (Bevy camera output)                 │        │
//! │ a│                                                 │        │
//! │ r│                                                 │        │
//! ├──┴─────────────────────────────────────────────────┴────────┤
//! │ [Console] [Assets] [Output]    | filter / actions           │  bottom
//! │                                                               │
//! │  Log entries / Asset list / ...                               │
//! ├─────────────────────────────────────────────────────────────┤
//! │ FPS: 60 | Entities: 12 | ● Editing | v0.1.0                  │  bottom (status)
//! └─────────────────────────────────────────────────────────────┘
//! ```

use crate::editor::components::{EditorCamera, Hidden, Locked, SceneEntity, ViewportCamera};
use crate::editor::panels::{
    about, asset_browser, console, inspector, menu_bar, scene_hierarchy, toolbar, viewport,
    AssetBrowserState, BottomTab, ConsoleState, HierarchyState, PanelVisibility,
};
use crate::editor::resources::{
    AssetDatabase, CommandHistory, EditorLog, EditorSettings, ProjectResource,
};
use crate::editor::state::{EditorState, Selection};
use crate::editor::theme::EditorTheme;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::hierarchy::{Children, Parent};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

/// The master layout system. Draws ALL editor UI in the correct egui panel order.
///
/// This is the ONLY system that should draw egui panels. Individual panel
/// modules expose `draw(ctx, ...)` or `draw_content(ui, ...)` functions that
/// are called from here.
#[allow(clippy::too_many_arguments)]
pub fn draw_editor_ui(
    mut ctxs: EguiContexts,
    theme: Res<EditorTheme>,
    mut panel_visibility: ResMut<PanelVisibility>,
    mut bottom_tab: ResMut<BottomTab>,
    mut console_state: ResMut<ConsoleState>,
    mut hierarchy_state: ResMut<HierarchyState>,
    mut asset_browser_state: ResMut<AssetBrowserState>,
    project: Res<ProjectResource>,
    asset_db: Res<AssetDatabase>,
    editor_log: Res<EditorLog>,
    mut selection: ResMut<Selection>,
    mut settings: ResMut<EditorSettings>,
    history: Res<CommandHistory>,
    current_state: Res<State<EditorState>>,
    mut next_state: ResMut<NextState<EditorState>>,
    camera_query: Query<&ViewportCamera, With<EditorCamera>>,
    transform_query: Query<&Transform>,
    visibility_query: Query<&Visibility>,
    name_query: Query<&Name>,
    parents: Query<&Parent>,
    children: Query<&Children>,
    scene_entities: Query<Entity, With<SceneEntity>>,
    hidden: Query<&Hidden>,
    locked: Query<&Locked>,
    mut commands: Commands,
    diagnostics: Res<DiagnosticsStore>,
) {
    let Some(ctx) = ctxs.try_ctx_mut() else {
        return;
    };

    // Apply the editor theme for this frame.
    theme.apply(ctx);

    // Handle global keyboard shortcuts.
    handle_keyboard_shortcuts(ctx, &current_state, &mut next_state);

    // ═══════════════════════════════════════════════════════════════
    // 1. TOP PANELS — drawn first, in visual top-to-bottom order.
    //    menu_bar is on top, toolbar below it.
    // ═══════════════════════════════════════════════════════════════

    menu_bar::draw(
        ctx,
        &mut *panel_visibility,
        &project,
        &history,
        &current_state,
        &mut next_state,
        &mut settings,
    );

    toolbar::draw(ctx, &current_state, &mut next_state, &project);

    // ═══════════════════════════════════════════════════════════════
    // 2. BOTTOM PANELS — drawn next, in visual bottom-to-top order.
    //    status_bar is bottommost (called first), bottom_panel above it.
    // ═══════════════════════════════════════════════════════════════

    // --- Status bar (bottommost) ---
    egui::TopBottomPanel::bottom("status_bar")
        .exact_height(24.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;

                // FPS
                let fps = diagnostics
                    .get(FrameTimeDiagnosticsPlugin::FPS)
                    .and_then(|d| d.smoothed())
                    .unwrap_or(0.0);
                let fps_color = if fps > 55.0 {
                    egui::Color32::from_rgb(40, 180, 80)
                } else if fps > 30.0 {
                    egui::Color32::from_rgb(204, 153, 0)
                } else {
                    egui::Color32::from_rgb(204, 0, 0)
                };
                ui.colored_label(fps_color, format!("FPS: {:.0}", fps));
                ui.separator();

                // Entity count
                ui.label(format!("Entities: {}", scene_entities.iter().count()));
                ui.separator();

                // Selection
                if let Some(primary) = selection.primary {
                    let name = name_query
                        .get(primary)
                        .map(|n| n.as_str().to_string())
                        .unwrap_or_else(|_| format!("{:?}", primary));
                    ui.label(format!("Selected: {}", name));
                } else {
                    ui.label(
                        egui::RichText::new("No selection")
                            .color(egui::Color32::from_rgb(140, 140, 140)),
                    );
                }
                ui.separator();

                // Editor state
                let (state_label, state_color) = match current_state.get() {
                    EditorState::Loading => ("Loading", egui::Color32::from_rgb(153, 153, 153)),
                    EditorState::Editing => ("Editing", egui::Color32::from_rgb(40, 180, 80)),
                    EditorState::Playing => ("Playing", egui::Color32::from_rgb(0, 122, 204)),
                    EditorState::Paused => ("Paused", egui::Color32::from_rgb(204, 153, 0)),
                };
                ui.colored_label(state_color, format!("● {}", state_label));

                // Right-aligned info
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(format!("Bevy Editor v{}", env!("CARGO_PKG_VERSION")))
                            .color(egui::Color32::from_rgb(140, 140, 140)),
                    );
                });
            });
        });

    // --- Tabbed bottom panel (Console / Assets / Output) ---
    egui::TopBottomPanel::bottom("bottom_panel")
        .default_height(200.0)
        .height_range(80.0..=500.0)
        .resizable(true)
        .show(ctx, |ui| {
            // Tab bar
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;

                let tabs = [
                    (BottomTab::Console, "📋 Console"),
                    (BottomTab::Assets, "📦 Assets"),
                    (BottomTab::Output, "📤 Output"),
                ];
                for (tab, label) in tabs {
                    let selected = *bottom_tab == tab;
                    let bg = if selected {
                        egui::Color32::from_rgb(51, 51, 51)
                    } else {
                        egui::Color32::from_rgb(30, 30, 30)
                    };
                    let text_color = if selected {
                        egui::Color32::WHITE
                    } else {
                        egui::Color32::from_rgb(140, 140, 140)
                    };
                    let resp = ui.add(
                        egui::Button::new(egui::RichText::new(label).color(text_color))
                            .fill(bg)
                            .frame(false)
                            .min_size(egui::vec2(100.0, 0.0)),
                    );
                    if resp.clicked() {
                        *bottom_tab = tab;
                    }
                }

                ui.separator();

                // Right-aligned per-tab actions
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    match *bottom_tab {
                        BottomTab::Console => {
                            if ui.button("Clear").clicked() {
                                editor_log.clear();
                            }
                        }
                        BottomTab::Assets => {
                            if ui.button("➕ Import...").clicked() {
                                if let Some(file) = rfd::FileDialog::new().pick_file() {
                                    info!("Import asset: {:?}", file);
                                }
                            }
                            if ui.button("⟳ Re-scan").clicked() {
                                info!("Re-scan assets (TODO)");
                            }
                            if ui.button("📂 Open Folder").clicked() {
                                let _ = open::that(&project.assets_dir);
                            }
                        }
                        BottomTab::Output => {}
                    }
                });
            });
            ui.separator();

            // Tab content (fills remaining space in the bottom panel)
            match *bottom_tab {
                BottomTab::Console => {
                    console::draw_content(ui, &editor_log, &mut *console_state);
                }
                BottomTab::Assets => {
                    asset_browser::draw_content(ui, &asset_db, &project, &mut *asset_browser_state);
                }
                BottomTab::Output => {
                    ui.vertical_centered(|ui| {
                        ui.add_space(30.0);
                        ui.label("Output panel");
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new(
                                "Build output and compilation errors will appear here.",
                            )
                            .color(egui::Color32::from_rgb(140, 140, 140)),
                        );
                    });
                }
            }
        });

    // ═══════════════════════════════════════════════════════════════
    // 3. SIDE PANELS — hierarchy (left) and inspector (right).
    //    Drawn after top/bottom panels, before the central panel.
    // ═══════════════════════════════════════════════════════════════

    if panel_visibility.scene_hierarchy {
        egui::SidePanel::left("hierarchy")
            .default_width(280.0)
            .width_range(180.0..=480.0)
            .resizable(true)
            .show(ctx, |ui| {
                scene_hierarchy::draw_content(
                    ui,
                    &mut *selection,
                    &parents,
                    &children,
                    &name_query,
                    &scene_entities,
                    &hidden,
                    &locked,
                    &mut commands,
                    &mut *hierarchy_state,
                );
            });
    }

    if panel_visibility.inspector {
        egui::SidePanel::right("inspector")
            .default_width(320.0)
            .width_range(220.0..=520.0)
            .resizable(true)
            .show(ctx, |ui| {
                inspector::draw_content(
                    ui,
                    &*selection,
                    &transform_query,
                    &visibility_query,
                    &name_query,
                );
            });
    }

    // ═══════════════════════════════════════════════════════════════
    // 4. CENTRAL PANEL (viewport) — MUST BE DRAWN LAST.
    //    It fills whatever space remains after all other panels.
    // ═══════════════════════════════════════════════════════════════

    egui::CentralPanel::default().show(ctx, |ui| {
        viewport::draw_content(ui, &*selection, &mut *settings, &camera_query);
    });

    // ═══════════════════════════════════════════════════════════════
    // 5. FLOATING WINDOWS — drawn after all panels.
    //    These appear on top of everything else.
    // ═══════════════════════════════════════════════════════════════

    about::draw_window(ctx, &mut *panel_visibility, &mut *settings);
}

/// Handle global keyboard shortcuts.
fn handle_keyboard_shortcuts(
    ctx: &egui::Context,
    current_state: &State<EditorState>,
    next_state: &mut NextState<EditorState>,
) {
    // F5 — toggle Play / Stop
    if ctx.input(|i| i.key_pressed(egui::Key::F5)) {
        match current_state.get() {
            EditorState::Editing => next_state.set(EditorState::Playing),
            EditorState::Playing | EditorState::Paused => next_state.set(EditorState::Editing),
            _ => {}
        }
    }

    // F6 — Pause / Resume
    if ctx.input(|i| i.key_pressed(egui::Key::F6)) {
        match current_state.get() {
            EditorState::Playing => next_state.set(EditorState::Paused),
            EditorState::Paused => next_state.set(EditorState::Playing),
            _ => {}
        }
    }
}
