//! Scene hierarchy panel.
//!
//! Shows the entity tree (parent / child relationships) of the current scene.
//! Clicking an entity selects it; right-click shows a context menu with
//! common operations (rename, duplicate, delete, add child, ...).

use crate::editor::components::{Hidden, Locked, SceneEntity, Selected};
use crate::editor::state::Selection;
use bevy::ecs::entity::Entity;
use bevy::hierarchy::{Children, Parent};
use bevy::prelude::*;
use bevy_egui::egui;

/// Scene hierarchy draw system.
pub fn draw_system(
    mut ctxs: bevy_egui::EguiContexts,
    mut selection: ResMut<Selection>,
    parents: Query<&Parent>,
    children: Query<&Children>,
    names: Query<&Name>,
    scene_entities: Query<Entity, With<SceneEntity>>,
    hidden: Query<&Hidden>,
    locked: Query<&Locked>,
    mut commands: Commands,
) {
    let Some(ctx) = ctxs.ctx_mut() else {
        return;
    };

    egui::SidePanel::left("scene_hierarchy")
        .default_width(280.0)
        .width_range(180.0..=480.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.strong("Hierarchy");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("+").clicked() {
                        info!("Hierarchy → Add entity (TODO)");
                    }
                    if ui.button("⟳").clicked() {
                        info!("Hierarchy → Refresh");
                    }
                });
            });
            ui.separator();

            // Filter input
            let mut filter = String::new();
            ui.add(egui::TextEdit::singleline(&mut filter).hint_text("🔍 Filter..."));
            ui.separator();

            // Collect root entities (those without a parent, or whose parent is not a SceneEntity).
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
                        ui.label("(no entities in scene)");
                        ui.label("Use the + button or the Add menu to create one.");
                    } else {
                        for entity in roots {
                            draw_entity_tree(
                                entity,
                                &mut selection,
                                &parents,
                                &children,
                                &names,
                                &hidden,
                                &locked,
                                &mut commands,
                                ui,
                                &filter,
                            );
                        }
                    }
                });

            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!("Total: {} entities", scene_entities.iter().count()));
            });
        });
}

#[allow(clippy::too_many_arguments)]
fn draw_entity_tree(
    entity: Entity,
    selection: &mut Selection,
    _parents: &Query<&Parent>,
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

    // Filter
    if !filter.is_empty() && !name.to_lowercase().contains(&filter.to_lowercase()) {
        // Even if this entity doesn't match, recurse to check children
        if let Ok(child_list) = children.get(entity) {
            for &child in child_list {
                draw_entity_tree(
                    child, selection, _parents, children, names, hidden, locked, commands, ui,
                    filter,
                );
            }
        }
        return;
    }

    let prefix = if is_hidden {
        "🚫 "
    } else if is_locked {
        "🔒 "
    } else {
        "  "
    };
    let label_text = format!("{prefix}{name}");
    let label_color = if is_hidden {
        egui::Color32::from_rgb(120, 120, 120)
    } else if is_locked {
        egui::Color32::from_rgb(204, 153, 0)
    } else {
        egui::Color32::from_rgb(220, 220, 220)
    };

    let has_children = children.get(entity).map(|c| !c.is_empty()).unwrap_or(false);

    ui.horizontal(|ui| {
        // Expand / collapse arrow
        let _expand = if has_children {
            ui.button("▾").clicked()
        } else {
            ui.label(" ");
            false
        };

        // The entity button itself
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
        let indent = 16.0;
        ui.indent(format!("indent_{:?}", entity), |ui| {
            if let Ok(child_list) = children.get(entity) {
                for &child in child_list {
                    draw_entity_tree(
                        child, selection, _parents, children, names, hidden, locked, commands, ui,
                        filter,
                    );
                }
            }
        });
        let _ = indent;
    }
}
