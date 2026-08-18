//! Scene hierarchy panel content.
//!
//! Shows the entity tree (parent / child relationships) of the current scene.
//! Drawn inside a `SidePanel::left` by the master layout system.

use crate::editor::components::{Hidden, Locked, SceneEntity, Selected};
use crate::editor::panels::HierarchyState;
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
    names: &Query<&Name>,
    scene_entities: &Query<Entity, With<SceneEntity>>,
    hidden: &Query<&Hidden>,
    locked: &Query<&Locked>,
    commands: &mut Commands,
    state: &mut HierarchyState,
) {
    // ---- Header ----
    ui.horizontal(|ui| {
        ui.strong("Hierarchy");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("⟳").on_hover_text("Refresh").clicked() {
                info!("Hierarchy → Refresh");
            }
            if ui.button("➕").on_hover_text("Add entity").clicked() {
                info!("Hierarchy → Add entity (TODO)");
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
    // Collect root entities (no parent, or parent is not a SceneEntity).
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
                for entity in roots {
                    draw_entity_tree(
                        entity,
                        selection,
                        parents,
                        children,
                        names,
                        hidden,
                        locked,
                        commands,
                        ui,
                        &state.filter,
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
    names: &Query<&Name>,
    hidden: &Query<&Hidden>,
    locked: &Query<&Locked>,
    commands: &mut Commands,
    ui: &mut egui::Ui,
    filter: &str,
) {
    let name = names
        .get(entity)
        .map(|n| n.as_str().to_string())
        .unwrap_or_else(|_| format!("Entity {:?}", entity));
    let is_hidden = hidden.get(entity).is_ok();
    let is_locked = locked.get(entity).is_ok();
    let is_selected = selection.contains(entity);

    // Filter: if this entity doesn't match, still check children
    if !filter.is_empty() && !name.to_lowercase().contains(&filter.to_lowercase()) {
        if let Ok(child_list) = children.get(entity) {
            for &child in child_list {
                draw_entity_tree(
                    child, selection, parents, children, names, hidden, locked, commands, ui,
                    filter,
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
    let label_text = format!("{icon}  {name}");
    let label_color = if is_hidden {
        egui::Color32::from_rgb(120, 120, 120)
    } else if is_locked {
        egui::Color32::from_rgb(204, 153, 0)
    } else {
        egui::Color32::from_rgb(220, 220, 220)
    };

    let has_children = children.get(entity).map(|c| !c.is_empty()).unwrap_or(false);

    ui.horizontal(|ui| {
        // Expand/collapse indicator
        if has_children {
            ui.label("▾");
        } else {
            ui.label(" ");
        }

        // Entity label (clickable)
        let btn = egui::SelectableLabel::new(
            is_selected,
            egui::RichText::new(&label_text).color(label_color),
        );
        let resp = ui.add(btn);
        if resp.clicked() {
            selection.set(entity);
            commands.entity(entity).insert(Selected);
        }

        // Right-click context menu
        resp.context_menu(|ui| {
            if ui.button("Rename").clicked() {
                info!("Rename entity {:?} (TODO)", entity);
                ui.close_menu();
            }
            if ui.button("Duplicate").clicked() {
                info!("Duplicate entity {:?} (TODO)", entity);
                ui.close_menu();
            }
            if ui.button("Delete").clicked() {
                info!("Delete entity {:?} (TODO)", entity);
                ui.close_menu();
            }
            ui.separator();
            if is_hidden {
                if ui.button("Show").clicked() {
                    commands.entity(entity).remove::<Hidden>();
                    ui.close_menu();
                }
            } else {
                if ui.button("Hide").clicked() {
                    commands.entity(entity).insert(Hidden);
                    ui.close_menu();
                }
            }
            if is_locked {
                if ui.button("Unlock").clicked() {
                    commands.entity(entity).remove::<Locked>();
                    ui.close_menu();
                }
            } else {
                if ui.button("Lock").clicked() {
                    commands.entity(entity).insert(Locked);
                    ui.close_menu();
                }
            }
            ui.separator();
            if ui.button("Add Child").clicked() {
                info!("Add child to {:?} (TODO)", entity);
                ui.close_menu();
            }
        });
    });

    // Recurse into children
    if has_children {
        ui.indent(format!("entity_children_{:?}", entity), |ui| {
            if let Ok(child_list) = children.get(entity) {
                for &child in child_list {
                    draw_entity_tree(
                        child, selection, parents, children, names, hidden, locked, commands,
                        ui, filter,
                    );
                }
            }
        });
    }
}
