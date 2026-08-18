//! Editor state, play-mode, and selection resources.
//!
//! These are the small, fast, frequently-queried bits of editor state that drive
//! most of the UI logic. Heavier resources live in [`super::resources`].

use bevy::prelude::*;
use std::path::PathBuf;

/// Top-level editor lifecycle state machine.
///
/// Drives `run_if` conditions and `StateScoped` entities.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EditorState {
    /// The asset loader is still loading the initial editor assets (fonts, theme, etc.).
    #[default]
    Loading,
    /// The editor is open and the user is editing a scene.
    Editing,
    /// The user pressed Play — the game's systems are running.
    Playing,
    /// The user pressed Pause — the game's systems are paused but the editor UI is live.
    Paused,
}

impl EditorState {
    /// True if the editor is currently in play mode (Playing or Paused).
    pub fn is_play_mode(self) -> bool {
        matches!(self, EditorState::Playing | EditorState::Paused)
    }

    /// True if the editor is currently in edit mode.
    pub fn is_edit_mode(self) -> bool {
        matches!(self, EditorState::Editing)
    }
}

/// The currently selected entity (or entities) in the editor.
///
/// The inspector panel reads this to know which entity's components to display,
/// the viewport draws a gizmo on the selected entity, and the scene hierarchy
/// highlights the row.
#[derive(Resource, Default, Debug, Clone)]
pub struct Selection {
    /// The primary selected entity (drives the inspector).
    pub primary: Option<Entity>,
    /// The full set of selected entities (multi-select).
    pub entities: Vec<Entity>,
}

impl Selection {
    /// Replace the current selection with a single entity.
    pub fn set(&mut self, entity: Entity) {
        self.primary = Some(entity);
        self.entities = vec![entity];
    }

    /// Toggle an entity in the current selection (multi-select).
    pub fn toggle(&mut self, entity: Entity) {
        if let Some(idx) = self.entities.iter().position(|&e| e == entity) {
            self.entities.remove(idx);
            self.primary = self.entities.last().copied();
        } else {
            self.entities.push(entity);
            self.primary = Some(entity);
        }
    }

    /// Clear the selection.
    pub fn clear(&mut self) {
        self.primary = None;
        self.entities.clear();
    }

    /// True if the given entity is in the selection.
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.iter().any(|&e| e == entity)
    }

    /// True if anything is selected.
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

/// Convenience wrapper for a path that may or may not be set yet.
#[derive(Resource, Default, Debug, Clone)]
pub struct CurrentScenePath(pub Option<PathBuf>);
