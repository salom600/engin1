//! About dialog + Settings dialog.

use crate::editor::resources::EditorSettings;
use crate::editor::resources::ThemeKind;
use bevy::prelude::*;
use bevy_egui::egui;

/// About panel system — shows the about dialog when `panel_visibility.about_open` is true.
pub fn draw_system(
    mut ctxs: bevy_egui::EguiContexts,
    mut panel_visibility: ResMut<crate::editor::panels::PanelVisibility>,
) {
    let Some(ctx) = ctxs.try_ctx_mut() else {
        return;
    };

    if panel_visibility.about_open {
        egui::Window::new("About Bevy Editor")
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);
                    ui.heading("Bevy Engine Editor");
                    ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
                    ui.add_space(8.0);
                    ui.label("A complete, integrated game engine editor");
                    ui.label("built on the Bevy engine with bevy_egui.");
                    ui.add_space(8.0);
                    ui.label("Features:");
                    ui.label("• 3D viewport with orbit camera");
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
                draw_settings(ui, &mut panel_visibility);
            });
    }
}

/// Standalone settings window (used both standalone and inside the About dialog).
pub fn draw_settings(
    ui: &mut egui::Ui,
    panel_visibility: &mut crate::editor::panels::PanelVisibility,
) {
    let _ = panel_visibility;
    // We can't mutate EditorSettings from here (no resource access in this helper).
    // The actual settings mutation happens in the menu_bar's preferences sub-menu.
    ui.label("(Settings are configured via Edit → Preferences in the menu bar.)");
    ui.separator();
    ui.label("Quick reference:");
    ui.label("• View menu — toggle individual panels");
    ui.label("• Edit → Toggle Theme — switch dark / light");
    ui.label("• Asset menu — import, compress, organize");
    ui.label("• Build menu — build for Windows / macOS / Linux / Web");
}

/// Helper for the menu bar's preferences sub-menu (mutates settings directly).
pub fn draw_preferences_popup(ui: &mut egui::Ui, settings: &mut EditorSettings) {
    ui.label("Editor Settings");
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
        let theme_label = match settings.theme {
            ThemeKind::Dark => "Dark",
            ThemeKind::Light => "Light",
        };
        egui::ComboBox::from_label("")
            .selected_text(theme_label)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut settings.theme, ThemeKind::Dark, "Dark");
                ui.selectable_value(&mut settings.theme, ThemeKind::Light, "Light");
            });
    });
    ui.separator();
    ui.horizontal(|ui| {
        ui.label("Camera move speed:");
        ui.add(
            egui::DragValue::new(&mut settings.camera_move_speed)
                .range(1.0..=100.0)
                .speed(0.5),
        );
    });
    ui.horizontal(|ui| {
        ui.label("Camera rotation sensitivity:");
        ui.add(
            egui::DragValue::new(&mut settings.camera_rotation_sensitivity)
                .range(0.001..=0.1)
                .speed(0.001),
        );
    });
    ui.horizontal(|ui| {
        ui.label("Camera zoom sensitivity:");
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
}
