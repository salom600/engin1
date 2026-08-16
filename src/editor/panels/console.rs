//! Console / log panel.
//!
//! Shows the editor log buffer with severity filter, search, and a command
//! input at the bottom.

use crate::editor::resources::{EditorLog, LogLevel};
use bevy::prelude::*;
use bevy_egui::egui;

/// Console draw system.
pub fn draw_system(mut ctxs: bevy_egui::EguiContexts, editor_log: Res<EditorLog>) {
    let Some(ctx) = ctxs.ctx_mut().into() else {
        return;
    };

    egui::SidePanel::bottom("console")
        .default_height(180.0)
        .height_range(80.0..=520.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.strong("Console");
                ui.separator();
                let mut show_trace = true;
                let mut show_info = true;
                let mut show_warn = true;
                let mut show_error = true;
                ui.checkbox(&mut show_trace, "Trace");
                ui.checkbox(&mut show_info, "Info");
                ui.checkbox(&mut show_warn, "Warn");
                ui.checkbox(&mut show_error, "Error");
                ui.separator();
                if ui.button("Clear").clicked() {
                    editor_log.clear();
                }
                if ui.button("Save As...").clicked() {
                    info!("Save log (TODO)");
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{} entries", editor_log.len()));
                });
            });
            ui.separator();

            // Log entries
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    let entries = editor_log.snapshot();
                    for entry in entries.iter().rev() {
                        let visible = match entry.level {
                            LogLevel::Trace => show_trace,
                            LogLevel::Info => show_info,
                            LogLevel::Warn => show_warn,
                            LogLevel::Error => show_error,
                        };
                        if !visible {
                            continue;
                        }

                        let color = match entry.level {
                            LogLevel::Trace => egui::Color32::from_rgb(120, 120, 120),
                            LogLevel::Info => egui::Color32::from_rgb(220, 220, 220),
                            LogLevel::Warn => egui::Color32::from_rgb(204, 153, 0),
                            LogLevel::Error => egui::Color32::from_rgb(204, 0, 0),
                        };

                        let time = entry
                            .timestamp
                            .duration_since(std::time::UNIX_EPOCH)
                            .map(|d| d.as_secs())
                            .unwrap_or(0);
                        let target = entry.target.as_deref().unwrap_or("");
                        ui.horizontal(|ui| {
                            ui.colored_label(
                                egui::Color32::from_rgb(120, 120, 120),
                                format!("[{:08}", time),
                            );
                            ui.colored_label(color, format!("{}]", entry.level.label()));
                            if !target.is_empty() {
                                ui.colored_label(
                                    egui::Color32::from_rgb(80, 140, 200),
                                    format!("[{}] ", target),
                                );
                            }
                            ui.colored_label(color, &entry.message);
                        });
                    }
                });

            ui.separator();
            // Command input
            ui.horizontal(|ui| {
                ui.label("$");
                let mut cmd = String::new();
                let resp = ui.text_edit_singleline(&mut cmd, "Type a command and press Enter...");
                if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if !cmd.trim().is_empty() {
                        editor_log.push(LogLevel::Info, format!("> {}", cmd));
                        // Simple built-in commands
                        match cmd.trim() {
                            "help" => editor_log.push(
                                LogLevel::Info,
                                "Available commands: help, clear, version, scan, save",
                            ),
                            "clear" => editor_log.clear(),
                            "version" => editor_log.push(
                                LogLevel::Info,
                                format!("Bevy Editor v{}", env!("CARGO_PKG_VERSION")),
                            ),
                            "scan" => editor_log.push(LogLevel::Info, "Re-scanning assets..."),
                            "save" => editor_log.push(LogLevel::Info, "Saving scene..."),
                            _ => editor_log.push(LogLevel::Warn, format!("Unknown command: {cmd}")),
                        }
                    }
                }
            });
        });
}
