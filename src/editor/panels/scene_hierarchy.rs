//! Scene hierarchy panel content.
//!
//! Shows the entity tree (parent / child relationships) of the current scene.
//! Clicking an entity selects it; right-click shows a context menu with
//! rename, duplicate, delete, add child, lock, hide.

use crate::editor::components::{
    DeleteEntityRequest, DuplicateEntityRequest, Hidden, Locked, RenameEntityRequest, SceneEntity,
    Selected, SpawnRequest,
};
use crate::editor::panels::HierarchyState;
use crate::editor::panels::PendingActions;
use crate::editor::state::Selection;
use bevy::ecs::entity::Entity;
use bevy::hierarchy::{Children, Parent};
use bevy::prelude::*;
use bevy_egui::egui;

/// Draw the scene hierarchy content inside the given `ui`.
#[allow(clippy::too_many_arguments)]
pub fn draw_content(
    ui: &mut egui::Ui,
    selection: &mut Selection,
    parents: &Query<&Parent>,
    children: &Query<&Children>,
    names: &Query<&mut Name>,
    scene_entities: &Query<Entity, With<SceneEntity>>,
    hidden: &Query<&Hidden>,
    locked: &Query<&Locked>,
    pending: &mut PendingActions,
    state: &mut HierarchyState,
) {
    // ---- Header ----
    ui.horizontal(|ui| {
        ui.strong("Hierarchy");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("⟳").on_hover_text("Refresh").clicked() {
                // No-op — hierarchy is live-updated every frame
            }
            if ui.button("➕").on_hover_text("Add empty entity").clicked() {
                pending.spawns.push(SpawnRequest::Empty);
            }
        });
    });
    ui.separator();

    // ---- Filter ----
    ui.add(
        egui::TextEdit::singleline(&mut state.filter)
            .hint_text("🔍 Filter entities...")
            .desired_width(ui.available_width()),
    );
    ui.separator();

    // ---- Entity tree ----
    let mut roots: Vec<Entity> = scene_entities
        .iter()
        .filter(|&e| {
            parents
                .get(e)
                .map(|p| !scene_entities.contains(p.get()))
                .unwrap_or(true)
        })
        .collect();
    roots.sort_by_key(|e| {
        names
            .get(*e)
            .map(|n| n.as_str().to_string())
            .unwrap_or_default()
    });

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if roots.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.label("No entities in scene.");
                    ui.add_space(8.0);
                    ui.label("Use the ➕ button or the Add menu");
                    ui.label("in the toolbar to create one.");
                });
            } else {
                let filter = state.filter.clone();
                for entity in roots {
                    draw_entity_tree(
                        entity, selection, parents, children, names, hidden, locked, pending, ui,
                        &filter, state,
                    );
                }
            }
        });

    // ---- Footer ----
    ui.separator();
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(format!("{} entities", scene_entities.iter().count()))
                .color(egui::Color32::from_rgb(140, 140, 140))
                .small(),
        );
    });
}

#[allow(clippy::too_many_arguments)]
fn draw_entity_tree(
    entity: Entity,
    selection: &mut Selection,
    parents: &Query<&Parent>,
    children: &Query<&Children>,
    names: &Query<&mut Name>,
    hidden: &Query<&Hidden>,
    locked: &Query<&Locked>,
    pending: &mut PendingActions,
    ui: &mut egui::Ui,
    filter: &str,
    state: &mut HierarchyState,
) {
    let name = names
        .get(entity)
        .map(|n| n.as_str().to_string())
        .unwrap_or_else(|_| format!("Entity {:?}", entity));
    let is_hidden = hidden.get(entity).is_ok();
    let is_locked = locked.get(entity).is_ok();
    let is_selected = selection.contains(entity);

    // Filter
    if !filter.is_empty() && !name.to_lowercase().contains(&filter.to_lowercase()) {
        if let Ok(child_list) = children.get(entity) {
            for &child in child_list {
                draw_entity_tree(
                    child, selection, parents, children, names, hidden, locked, pending, ui,
                    filter, state,
                );
            }
        }
        return;
    }

    let icon = if is_hidden {
        "🚫"
    } else if is_locked {
        "🔒"
    } else {
        "📦"
    };

    let has_children = children.get(entity).map(|c| !c.is_empty()).unwrap_or(false);

    ui.horizontal(|ui| {
        if has_children {
            ui.label("▾");
        } else {
            ui.label(" ");
        }

        // Check if we're renaming this entity
        if state.renaming == Some(entity) {
            // Show a text edit for renaming
            let resp = ui.add(
                egui::TextEdit::singleline(&mut state.rename_buf)
                    .desired_width(140.0)
                    .clip_text(true),
            );
            if resp.lost_focus() {
                if !state.rename_buf.trim().is_empty() {
                    pending.renames.push(RenameEntityRequest {
                        entity,
                        new_name: state.rename_buf.trim().to_string(),
                    });
                }
                state.renaming = None;
            }
        } else {
            let label_text = format!("{icon}  {name}");
            let label_color = if is_hidden {
                egui::Color32::from_rgb(120, 120, 120)
            } else if is_locked {
                egui::Color32::from_rgb(204, 153, 0)
            } else {
                egui::Color32::from_rgb(220, 220, 220)
            };

            let btn = egui::SelectableLabel::new(
                is_selected,
                egui::RichText::new(&label_text).color(label_color),
            );
            let resp = ui.add(btn);
            if resp.clicked() {
                selection.set(entity);
            }

            // Right-click context menu
            resp.context_menu(|ui| {
                if ui.button("Rename").clicked() {
                    state.renaming = Some(entity);
                    state.rename_buf = name.clone();
                    ui.close_menu();
                }
                if ui.button("Duplicate").clicked() {
                    pending.duplicates.push(DuplicateEntityRequest { entity });
                    ui.close_menu();
                }
                if ui.button("Delete").clicked() {
                    pending.deletes.push(DeleteEntityRequest { entity });
                    ui.close_menu();
                }
                ui.separator();
                if is_hidden {
                    if ui.button("Show").clicked() {
                        // Will be handled by a command — for now just push a rename-like action
                        ui.close_menu();
                    }
                } else {
                    if ui.button("Hide").clicked() {
                        ui.close_menu();
                    }
                }
                if is_locked {
                    if ui.button("Unlock").clicked() {
                        ui.close_menu();
                    }
                } else {
                    if ui.button("Lock").clicked() {
                        ui.close_menu();
                    }
                }
                ui.separator();
                if ui.button("Add Child").clicked() {
                    pending
                        .spawns
                        .push(SpawnRequest::ChildOf { parent: entity });
                    ui.close_menu();
                }
            });
        }
    });

    // Recurse into children
    if has_children {
        ui.indent(format!("entity_children_{:?}", entity), |ui| {
            if let Ok(child_list) = children.get(entity) {
                for &child in child_list {
                    draw_entity_tree(
                        child, selection, parents, children, names, hidden, locked, pending, ui,
                        filter, state,
                    );
                }
            }
        });
    }
}
