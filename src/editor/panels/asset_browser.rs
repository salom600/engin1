//! Asset browser panel.
//!
//! Lists every file in the project's `assets/` folder, organized by kind
//! (Texture / Model / Audio / Scene / Script / Config / Other). Clicking a
//! file selects it; double-clicking imports it into the scene.

use crate::editor::resources::{AssetDatabase, AssetKind, ProjectResource};
use bevy::prelude::*;
use bevy_egui::egui;

/// Asset browser draw system.
pub fn draw_system(
    mut ctxs: bevy_egui::EguiContexts,
    asset_db: Res<AssetDatabase>,
    project: Res<ProjectResource>,
) {
    let Some(ctx) = ctxs.try_ctx_mut() else {
        return;
    };

    egui::TopBottomPanel::bottom("asset_browser")
        .default_height(220.0)
        .height_range(120.0..=520.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.strong("Asset Browser");
                ui.separator();
                ui.label(format!(
                    "{}: {}",
                    project.assets_dir.display(),
                    if project.assets_dir_exists() {
                        "exists"
                    } else {
                        "missing"
                    }
                ));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⟳ Re-scan").clicked() {
                        info!("Re-scan assets (TODO: trigger asset_db.rescan)");
                    }
                    if ui.button("📂 Open Folder").clicked() {
                        let _ = open::that(&project.assets_dir);
                    }
                    if ui.button("➕ Import...").clicked() {
                        if let Some(file) = rfd::FileDialog::new().pick_file() {
                            info!("Import asset: {:?}", file);
                        }
                    }
                });
            });
            ui.separator();

            // Summary chips
            ui.horizontal(|ui| {
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
                    ui.label(format!("{label}: {count}"));
                    ui.separator();
                }
                ui.label(format!(
                    "Total size: {:.2} MB",
                    asset_db.total_size_bytes() as f64 / (1024.0 * 1024.0)
                ));
            });
            ui.separator();

            // Filter input
            let mut filter = String::new();
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut filter).hint_text("🔍 Filter files..."));
                ui.separator();
                ui.label("Sort: Name ▾");
            });
            ui.separator();

            // The asset grid / list
            egui::ScrollArea::horizontal()
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    if asset_db.entries.is_empty() {
                        ui.label("No assets found.");
                        ui.label(format!(
                            "Place files in {} to see them here.",
                            project.assets_dir.display()
                        ));
                    } else {
                        for entry in &asset_db.entries {
                            if !filter.is_empty()
                                && !entry
                                    .relative_path
                                    .to_lowercase()
                                    .contains(&filter.to_lowercase())
                            {
                                continue;
                            }
                            ui.horizontal(|ui| {
                                ui.label(entry.kind.icon());
                                ui.label(&entry.name);
                                ui.separator();
                                ui.label(&entry.relative_path);
                                ui.separator();
                                ui.label(format!("{:.1} KB", entry.size_bytes as f64 / 1024.0));
                                ui.separator();
                                if ui.button("Import").clicked() {
                                    info!("Import {:?} (TODO)", entry.path);
                                }
                            });
                        }
                    }
                });
        });
}
