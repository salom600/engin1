//! Script editor panel.
//!
//! A code editor for Lua scripts. Users can create, edit, and save scripts
//! that are attached to entities via the ScriptComponent.

use crate::editor::resources::ProjectResource;
use bevy::prelude::*;
use bevy_egui::egui;
use std::path::PathBuf;

/// Persistent state for the script editor.
#[derive(Resource, Debug, Clone)]
pub struct ScriptEditorState {
    /// Whether the script editor window is open.
    pub open: bool,
    /// The currently edited script's file path.
    pub current_path: Option<PathBuf>,
    /// The text content of the script being edited.
    pub content: String,
    /// The name shown in the title bar.
    pub title: String,
    /// Whether the content has been modified since last save.
    pub dirty: bool,
    /// Buffer for the "New Script" filename input.
    pub new_name: String,
}

impl Default for ScriptEditorState {
    fn default() -> Self {
        Self {
            open: false,
            current_path: None,
            content: String::new(),
            title: "Script Editor".to_string(),
            dirty: false,
            new_name: "new_script.lua".to_string(),
        }
    }
}

impl ScriptEditorState {
    /// Load a script file into the editor.
    pub fn load(&mut self, path: PathBuf) {
        match std::fs::read_to_string(&path) {
            Ok(content) => {
                self.current_path = Some(path.clone());
                self.content = content;
                self.title = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Script Editor")
                    .to_string();
                self.dirty = false;
            }
            Err(e) => {
                error!("Failed to load script {:?}: {e}", path);
            }
        }
    }

    /// Save the current content to disk.
    pub fn save(&mut self) {
        let Some(path) = &self.current_path else {
            return;
        };
        match std::fs::write(path, &self.content) {
            Ok(_) => {
                self.dirty = false;
                info!("Script saved to {:?}", path);
            }
            Err(e) => error!("Failed to save script: {e}"),
        }
    }

    /// Create a new empty script.
    pub fn new_script(&mut self, project: &ProjectResource) {
        let name = if self.new_name.is_empty() {
            "new_script.lua".to_string()
        } else {
            self.new_name.clone()
        };
        let path = project.assets_dir.join("scripts").join(&name);
        self.current_path = Some(path.clone());
        self.content = "-- New Lua script\n-- Attach to an entity via the Inspector\n\nfunction on_start(entity)\n    print(\"Hello from \" .. tostring(entity))\nend\n\nfunction on_update(entity, dt)\n    -- Called every frame while playing\nend\n".to_string();
        self.title = name;
        self.dirty = true;
    }
}

/// Draw the script editor as a floating window.
pub fn draw_window(ctx: &egui::Context, state: &mut ScriptEditorState, project: &ProjectResource) {
    if !state.open {
        return;
    }

    let title = if state.dirty {
        format!("* {} — Script Editor", state.title)
    } else {
        format!("{} — Script Editor", state.title)
    };

    egui::Window::new(title)
        .open(&mut state.open)
        .resizable(true)
        .collapsible(true)
        .default_width(600.0)
        .default_height(480.0)
        .show(ctx, |ui| {
            // ---- Toolbar ----
            ui.horizontal(|ui| {
                if ui.button("📂 Open...").clicked() {
                    if let Some(file) = rfd::FileDialog::new()
                        .add_filter("Lua script", &["lua"])
                        .set_directory(&project.assets_dir.join("scripts"))
                        .pick_file()
                    {
                        state.load(file);
                    }
                }
                if ui.button("💾 Save").clicked() {
                    state.save();
                }
                if ui.button("Save As...").clicked() {
                    if let Some(file) = rfd::FileDialog::new()
                        .add_filter("Lua script", &["lua"])
                        .set_file_name(&state.title)
                        .save_file()
                    {
                        state.current_path = Some(file.clone());
                        state.save();
                    }
                }
                ui.separator();
                ui.label("New:");
                ui.add(
                    egui::TextEdit::singleline(&mut state.new_name)
                        .desired_width(120.0)
                        .hint_text("script_name.lua"),
                );
                if ui.button("➕ New").clicked() {
                    state.new_script(project);
                }
            });
            ui.separator();

            // ---- Code editor (multi-line text edit) ----
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let resp = ui.add(
                        egui::TextEdit::multiline(&mut state.content)
                            .code_editor()
                            .desired_width(f32::MAX)
                            .desired_rows(20),
                    );
                    if resp.changed() {
                        state.dirty = true;
                    }
                });

            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!(
                    "{} chars, {} lines",
                    state.content.len(),
                    state.content.lines().count()
                ));
                if state.dirty {
                    ui.colored_label(egui::Color32::from_rgb(204, 153, 0), "● Unsaved");
                } else if state.current_path.is_some() {
                    ui.colored_label(egui::Color32::from_rgb(40, 180, 80), "● Saved");
                }
            });
        });
}
