//! Editor UI panels built with `bevy_egui`.
//!
//! Each panel is its own Bevy system that draws itself on the shared
//! [`egui::Context`] every frame. This is more idiomatic than a registry
//! pattern because each panel declares exactly the queries/resources it
//! needs, and Bevy's change detection stays granular.
//!
//! ## Available panels
//!
//! | Panel              | File                          | Purpose                                  |
//! |--------------------|-------------------------------|------------------------------------------|
//! | MenuBar            | [`menu_bar`]                  | Top menu bar (File / Edit / View / ...)  |
//! | Toolbar            | [`toolbar`]                  | Play / Pause / Stop / Save / Load        |
//! | Viewport           | [`viewport`]                 | 3D scene rendering + gizmo               |
//! | SceneHierarchy     | [`scene_hierarchy`]           | Entity tree with parent/child display    |
//! | Inspector          | [`inspector`]                 | Reflection-based component editor       |
//! | AssetBrowser       | [`asset_browser`]             | File browser for `assets/`               |
//! | Console            | [`console`]                  | Log panel + command input                |
//! | About              | [`about`]                    | About dialog                             |

use bevy::prelude::*;

pub mod about;
pub mod asset_browser;
pub mod console;
pub mod inspector;
pub mod menu_bar;
pub mod scene_hierarchy;
pub mod toolbar;
pub mod viewport;

/// A per-panel visibility flag stored as a resource so the View menu can toggle panels.
#[derive(Resource, Debug, Clone, PartialEq)]
pub struct PanelVisibility {
    /// Viewport panel visible?
    pub viewport: bool,
    /// Scene hierarchy panel visible?
    pub scene_hierarchy: bool,
    /// Inspector panel visible?
    pub inspector: bool,
    /// Asset browser panel visible?
    pub asset_browser: bool,
    /// Console panel visible?
    pub console: bool,
    /// About dialog open?
    pub about_open: bool,
    /// Settings window open?
    pub settings_open: bool,
}

impl Default for PanelVisibility {
    fn default() -> Self {
        Self {
            viewport: true,
            scene_hierarchy: true,
            inspector: true,
            asset_browser: true,
            console: true,
            about_open: false,
            settings_open: false,
        }
    }
}

impl PanelVisibility {
    /// Toggle a panel by name (used by the View menu).
    pub fn toggle(&mut self, name: &str) {
        match name {
            "Viewport" => self.viewport = !self.viewport,
            "Scene Hierarchy" => self.scene_hierarchy = !self.scene_hierarchy,
            "Inspector" => self.inspector = !self.inspector,
            "Asset Browser" => self.asset_browser = !self.asset_browser,
            "Console" => self.console = !self.console,
            _ => {}
        }
    }
}
