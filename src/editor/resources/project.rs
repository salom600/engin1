//! The active Bevy project: name, root path, recently-opened scenes, default scene.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Top-level resource describing the currently-open Bevy project.
///
/// Populated by the File → Open Project menu action. The editor falls back to
/// a sensible default (the in-tree `assets/` folder) when no project is loaded.
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct ProjectResource {
    /// Human-readable project name (shown in the window title).
    pub name: String,
    /// Absolute path to the project root (where `Cargo.toml` lives).
    pub root: PathBuf,
    /// Absolute path to the project's `assets/` folder.
    pub assets_dir: PathBuf,
    /// Absolute path to the project's `src/` folder.
    pub src_dir: PathBuf,
    /// Most recently opened scenes (most recent first).
    pub recent_scenes: Vec<PathBuf>,
    /// Bevy version this project was created against.
    pub bevy_version: String,
    /// Project version (semver).
    pub version: String,
}

impl Default for ProjectResource {
    fn default() -> Self {
        let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self {
            name: "Untitled Project".to_string(),
            assets_dir: root.join("assets"),
            src_dir: root.join("src"),
            root,
            recent_scenes: Vec::new(),
            bevy_version: "0.14".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl ProjectResource {
    /// Mark a scene as recently opened (moves it to the front of the list).
    pub fn touch_recent_scene(&mut self, scene: PathBuf) {
        self.recent_scenes.retain(|p| p != &scene);
        self.recent_scenes.insert(0, scene);
        if self.recent_scenes.len() > 10 {
            self.recent_scenes.truncate(10);
        }
    }

    /// True if the project's `assets/` folder exists on disk.
    pub fn assets_dir_exists(&self) -> bool {
        self.assets_dir.is_dir()
    }
}

/// An event sent when a new project is loaded (or the current one is closed).
#[derive(Event, Debug, Clone)]
pub struct ProjectChangedEvent {
    /// The new project (or `None` if the project was closed).
    pub project: Option<ProjectResource>,
}
