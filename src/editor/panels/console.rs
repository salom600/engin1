//! Console / log panel content.
//!
//! Shows the editor log buffer with severity filter and command input.
//! Drawn inside the bottom tab panel by the master layout system.

use crate::editor::panels::ConsoleState;
use crate::editor::resources::{EditorLog, LogLevel};
use bevy::prelude::*;
use bevy_egui::egui;

/// Draw the console content inside the given `ui`.
pub fn draw_content(ui: &mut egui::Ui, editor_log: &EditorLog, state: &mut ConsoleState) {
    // ---- Filter bar ----
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        ui.checkbox(&mut state.show_trace, "Trace");
        ui.checkbox(&mut state.show_info, "Info");
        ui.checkbox(&mut state.show_warn, "Warn");
        ui.checkbox(&mut state.show_error, "Error");

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(format!("{} entries", editor_log.len()))
                    .color(egui::Color32::from_rgb(140, 140, 140))
                    .small(),
            );
        });
    });
    ui.separator();

    // ---- Log entries ----
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .stick_to_bottom(true)
        .show(ui, |ui| {
            let entries = editor_log.snapshot();
            if entries.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.label(
                        egui::RichText::new("Console is empty.\nLog messages will appear here.")
                            .color(egui::Color32::from_rgb(140, 140, 140))
                            .small(),
                    );
                });
                return;
            }

            for entry in entries.iter().rev() {
                let visible = match entry.level {
                    LogLevel::Trace => state.show_trace,
                    LogLevel::Info => state.show_info,
                    LogLevel::Warn => state.show_warn,
                    LogLevel::Error => state.show_error,
                };
                if !visible {
                    continue;
                }

                let color = match entry.level {
                    LogLevel::Trace => egui::Color32::from_rgb(120, 120, 120),
                    LogLevel::Info => egui::Color32::from_rgb(204, 204, 204),
                    LogLevel::Warn => egui::Color32::from_rgb(218, 165, 32),
                    LogLevel::Error => egui::Color32::from_rgb(220, 80, 80),
                };

                let time = entry
                    .timestamp
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                let target = entry.target.as_deref().unwrap_or("");

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    ui.colored_label(
                        egui::Color32::from_rgb(100, 100, 100),
                        format!("[{:08}", time),
                    );
                    ui.colored_label(color, format!("{:>5}]", entry.level.label()));
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

    // ---- Command input ----
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("$").color(egui::Color32::from_rgb(100, 180, 100)));
        let resp = ui.add(
            egui::TextEdit::singleline(&mut state.command)
                .hint_text("Type a command (help, clear, version, scan, save) and press Enter...")
                .desired_width(ui.available_width()),
        );
        if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            let cmd = state.command.trim().to_string();
            if !cmd.is_empty() {
                editor_log.push(LogLevel::Info, format!("> {}", cmd));
                match cmd.as_str() {
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
                state.command.clear();
                resp.request_focus();
            }
        }
    });
}
