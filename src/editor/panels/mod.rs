//! Editor UI panels built with `bevy_egui`.
//!
//! Each panel exposes a `draw(ctx, ...)` or `draw_content(ui, ...)` function
//! that is called by the master [`crate::editor::layout::draw_editor_ui`]
//! system in the correct egui panel order.
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
pub mod add_component;
pub mod asset_browser;
pub mod console;
pub mod inspector;
pub mod menu_bar;
pub mod scene_hierarchy;
pub mod script_editor;
pub mod toolbar;
pub mod viewport;

/// Per-panel visibility flags. Toggled via the View menu.
#[derive(Resource, Debug, Clone, PartialEq)]
pub struct PanelVisibility {
    /// Viewport panel visible?
    pub viewport: bool,
    /// Scene hierarchy panel visible?
    pub scene_hierarchy: bool,
    /// Inspector panel visible?
    pub inspector: bool,
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
            about_open: false,
            settings_open: false,
        }
    }
}

/// Which tab is active in the bottom panel (Console / Assets / Output).
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BottomTab {
    /// Console tab — log output + command input.
    #[default]
    Console,
    /// Asset browser tab — file list.
    Assets,
    /// Output tab — reserved for build output.
    Output,
}

/// Persistent state for the Console panel (filter flags + command input).
#[derive(Resource, Debug, Clone)]
pub struct ConsoleState {
    /// Show TRACE-level messages?
    pub show_trace: bool,
    /// Show INFO-level messages?
    pub show_info: bool,
    /// Show WARN-level messages?
    pub show_warn: bool,
    /// Show ERROR-level messages?
    pub show_error: bool,
    /// Current command input text.
    pub command: String,
}

impl Default for ConsoleState {
    fn default() -> Self {
        Self {
            show_trace: false,
            show_info: true,
            show_warn: true,
            show_error: true,
            command: String::new(),
        }
    }
}

/// Persistent state for the Hierarchy panel (filter text + rename state).
#[derive(Resource, Default, Debug, Clone)]
pub struct HierarchyState {
    /// Filter text for the entity list.
    pub filter: String,
    /// Which entity is currently being renamed (if any).
    pub renaming: Option<bevy::prelude::Entity>,
    /// Buffer for the rename text input.
    pub rename_buf: String,
}

/// Persistent state for the Viewport panel (transform mode + add-component dialog).
#[derive(Resource, Debug, Clone)]
pub struct ViewportState {
    /// Current transform tool mode (Select / Move / Rotate / Scale).
    pub transform_mode: crate::editor::panels::viewport::TransformMode,
    /// Whether the Add Component dialog is open.
    pub add_component_open: bool,
}

impl Default for ViewportState {
    fn default() -> Self {
        Self {
            transform_mode: crate::editor::panels::viewport::TransformMode::default(),
            add_component_open: false,
        }
    }
}

/// Pending editor actions queued by the UI, to be flushed by a system.
#[derive(Resource, Default, Debug)]
pub struct PendingActions {
    /// Spawn requests
    pub spawns: Vec<crate::editor::components::SpawnRequest>,
    /// Delete requests
    pub deletes: Vec<crate::editor::components::DeleteEntityRequest>,
    /// Rename requests
    pub renames: Vec<crate::editor::components::RenameEntityRequest>,
    /// Duplicate requests
    pub duplicates: Vec<crate::editor::components::DuplicateEntityRequest>,
    /// Save scene request
    pub save: bool,
    /// Load scene request
    pub load: Option<std::path::PathBuf>,
    /// Open the Add Component dialog
    pub open_add_component: bool,
    /// Open the Script Editor
    pub open_script_editor: bool,
}

/// Persistent state for the Asset Browser panel (filter text).
#[derive(Resource, Default, Debug, Clone)]
pub struct AssetBrowserState {
    /// Filter text for the asset list.
    pub filter: String,
}
