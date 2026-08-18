//! Asset browser panel content.
//!
//! Lists every file in the project's `assets/` folder, organized by kind.
//! Drawn inside the bottom tab panel by the master layout system.

use crate::editor::panels::AssetBrowserState;
use crate::editor::resources::{AssetDatabase, AssetKind, ProjectResource};
use bevy::prelude::*;
use bevy_egui::egui;

/// Draw the asset browser content inside the given `ui`.
pub fn draw_content(
    ui: &mut egui::Ui,
    asset_db: &AssetDatabase,
    project: &ProjectResource,
    state: &mut AssetBrowserState,
) {
    // ---- Summary chips ----
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        let counts = [
            (AssetKind::Texture, "Textures"),
            (AssetKind::Model, "Models"),
            (AssetKind::Audio, "Audio"),
            (AssetKind::Scene, "Scenes"),
            (AssetKind::Script, "Scripts"),
            (AssetKind::Config, "Config"),
            (AssetKind::Other, "Other"),
        ];
        for (kind, label) in counts {
            let count = asset_db.count_of(kind);
            if count > 0 {
                ui.label(
                    egui::RichText::new(format!("{}: {}", label, count))
                        .color(egui::Color32::from_rgb(180, 180, 180))
                        .small(),
                );
                ui.separator();
            }
        }
        ui.label(
            egui::RichText::new(format!(
                "Total: {:.1} MB",
                asset_db.total_size_bytes() as f64 / (1024.0 * 1024.0)
            ))
            .color(egui::Color32::from_rgb(140, 140, 140))
            .small(),
        );
    });
    ui.separator();

    // ---- Filter ----
    ui.horizontal(|ui| {
        ui.add(
            egui::TextEdit::singleline(&mut state.filter)
                .hint_text("🔍 Filter files...")
                .desired_width(200.0),
        );
        ui.separator();
        ui.label(
            egui::RichText::new(format!("📁 {}", project.assets_dir.display()))
                .color(egui::Color32::from_rgb(140, 140, 140))
                .small(),
        );
    });
    ui.separator();

    // ---- Asset list ----
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if asset_db.entries.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.label("No assets found.");
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new(format!(
                            "Place files in {} to see them here.",
                            project.assets_dir.display()
                        ))
                        .color(egui::Color32::from_rgb(140, 140, 140))
                        .small(),
                    );
                });
            } else {
                // Table header
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Type").strong().small());
                    ui.separator();
                    ui.label(egui::RichText::new("Name").strong().small());
                    ui.separator();
                    ui.label(egui::RichText::new("Path").strong().small());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new("Size").strong().small());
                    });
                });
                ui.separator();

                for entry in &asset_db.entries {
                    if !state.filter.is_empty()
                        && !entry
                            .relative_path
                            .to_lowercase()
                            .contains(&state.filter.to_lowercase())
                    {
                        continue;
                    }
                    ui.horizontal(|ui| {
                        ui.label(entry.kind.icon());
                        ui.separator();
                        ui.label(&entry.name);
                        ui.separator();
                        ui.label(
                            egui::RichText::new(&entry.relative_path)
                                .color(egui::Color32::from_rgb(140, 140, 140))
                                .small(),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(format!(
                                    "{:.1} KB",
                                    entry.size_bytes as f64 / 1024.0
                                ))
                                .color(egui::Color32::from_rgb(140, 140, 140))
                                .small(),
                            );
                        });
                    });
                }
            }
        });
}
