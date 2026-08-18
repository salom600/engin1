//! About dialog + Settings dialog.
//!
//! These are floating `egui::Window` objects drawn after all panels.

use crate::editor::panels::PanelVisibility;
use crate::editor::resources::{EditorSettings, ThemeKind};
use bevy::prelude::*;
use bevy_egui::egui;

/// Draw the About and Settings floating windows (if open).
pub fn draw_window(
    ctx: &egui::Context,
    panel_visibility: &mut PanelVisibility,
    settings: &mut EditorSettings,
) {
    if panel_visibility.about_open {
        egui::Window::new("About Bevy Editor")
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);
                    ui.heading("🦋 Bevy Engine Editor");
                    ui.label(
                        egui::RichText::new(format!("Version {}", env!("CARGO_PKG_VERSION")))
                            .color(egui::Color32::from_rgb(140, 140, 140)),
                    );
                    ui.add_space(8.0);
                    ui.label("A complete, integrated game engine editor");
                    ui.label("built on the Bevy engine with bevy_egui.");
                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(4.0);

                    ui.label(egui::RichText::new("Features").strong());
                    ui.label("• 3D viewport with orbit camera + gizmo");
                    ui.label("• Scene hierarchy with parent/child display");
                    ui.label("• Reflection-based component inspector");
                    ui.label("• Asset browser with import / organize");
                    ui.label("• Built-in console with severity filter");
                    ui.label("• Play / Pause / Stop / Step toolbar");
                    ui.label("• Physics (Rapier3D) integration");
                    ui.label("• Audio (bevy_audio) integration");
                    ui.label("• Scripting (Lua) integration");
                    ui.label("• AI (big-brain) integration");
                    ui.add_space(8.0);
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Repository:");
                        ui.hyperlink_to(
                            "github.com/salom600/engin1",
                            "https://github.com/salom600/engin1",
                        );
                    });
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.button("Close").clicked() {
                            panel_visibility.about_open = false;
                        }
                        if ui.button("Open Documentation").clicked() {
                            let _ = open::that("https://bevyengine.org/learn/book/");
                        }
                    });
                });
            });
    }

    if panel_visibility.settings_open {
        egui::Window::new("Settings")
            .resizable(true)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .default_width(420.0)
            .show(ctx, |ui| {
                draw_settings_content(ui, settings, panel_visibility);
            });
    }
}

/// Draw the settings content (used inside the Settings window).
fn draw_settings_content(
    ui: &mut egui::Ui,
    settings: &mut EditorSettings,
    panel_visibility: &mut PanelVisibility,
) {
    ui.label(egui::RichText::new("Editor Settings").strong());
    ui.separator();

    ui.horizontal(|ui| {
        ui.label("Autosave (seconds):");
        ui.add(
            egui::DragValue::new(&mut settings.autosave_interval_secs)
                .range(0.0..=3600.0)
                .speed(1.0),
        );
    });

    ui.checkbox(&mut settings.show_grid, "Show grid");
    ui.checkbox(&mut settings.show_axes, "Show world axes");
    ui.checkbox(&mut settings.show_physics_debug, "Show physics debug");
    ui.checkbox(&mut settings.show_gizmo, "Show transform gizmo");
    ui.checkbox(&mut settings.capture_screenshots, "Enable F12 screenshots");

    ui.separator();

    ui.horizontal(|ui| {
        ui.label("Theme:");
        egui::ComboBox::from_label("")
            .selected_text(match settings.theme {
                ThemeKind::Dark => "Dark",
                ThemeKind::Light => "Light",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut settings.theme, ThemeKind::Dark, "Dark");
                ui.selectable_value(&mut settings.theme, ThemeKind::Light, "Light");
            });
    });

    ui.separator();

    ui.label(egui::RichText::new("Camera").strong());
    ui.horizontal(|ui| {
        ui.label("Move speed:");
        ui.add(
            egui::DragValue::new(&mut settings.camera_move_speed)
                .range(1.0..=100.0)
                .speed(0.5),
        );
    });
    ui.horizontal(|ui| {
        ui.label("Rotation sensitivity:");
        ui.add(
            egui::DragValue::new(&mut settings.camera_rotation_sensitivity)
                .range(0.001..=0.1)
                .speed(0.001),
        );
    });
    ui.horizontal(|ui| {
        ui.label("Zoom sensitivity:");
        ui.add(
            egui::DragValue::new(&mut settings.camera_zoom_sensitivity)
                .range(0.05..=2.0)
                .speed(0.05),
        );
    });

    ui.separator();

    ui.horizontal(|ui| {
        ui.label("Max undo history:");
        ui.add(
            egui::DragValue::new(&mut settings.max_undo_history)
                .range(10..=10000)
                .speed(10.0),
        );
    });

    ui.separator();
    ui.horizontal(|ui| {
        if ui.button("Close").clicked() {
            panel_visibility.settings_open = false;
        }
    });
}
