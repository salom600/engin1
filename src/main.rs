//! # Bevy Engine Editor
//!
//! A complete, integrated game engine editor built on top of the [Bevy engine](https://bevyengine.org)
//! using [bevy_egui](https://github.com/mvlabat/bevy_egui) for the editor UI.
//!
//! ## Features
//!
//! - 3D viewport with orbit camera, gizmo, and entity picking
//! - Scene hierarchy (entity tree) with parent / child relationships
//! - Component inspector (reflection-based, edit any `Reflect` component)
//! - Asset browser (textures, models, audio, scenes, scripts)
//! - Built-in console / log panel
//! - Toolbar: Play / Pause / Stop / Step / Save / Load
//! - Menu bar: File / Edit / View / Asset / Help
//! - Integrated subsystems: Physics (Rapier3D), Audio (bevy_audio),
//!   Scripting (Lua), AI (big-brain), Asset pipeline
//! - GitHub Actions CI/CD for Windows / macOS / Linux
//!
//! ## Architecture
//!
//! ```text
//! src/
//! ├── main.rs                     - Entry point, plugin setup
//! ├── editor/
//! │   ├── mod.rs                  - EditorPlugin: registers all editor resources/systems
//! │   ├── state.rs                - EditorState, EditorMode, Selection
//! │   ├── theme.rs                - Egui theme + Bevy color palette
//! │   ├── resources/              - Editor resources (asset DB, log buffer, project)
//! │   ├── components/             - Marker components used by the editor
//! │   ├── panels/                 - Egui panels (viewport, hierarchy, inspector, ...)
//! │   └── systems/                - Subsystem integrations (physics, audio, scripting, AI, camera)
//! └── game/                       - The user's game code (sample scene + camera)
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod editor;
mod game;

/// Entry point for the Bevy Editor application.
///
/// Initializes the [`bevy::app::App`] with:
/// - The default Bevy plugins (rendering, input, audio, assets, winit window).
/// - [`EguiPlugin`] for the immediate-mode UI layer.
/// - [`bevy_rapier3d::prelude::RapierPhysicsPlugin`] for 3D physics.
/// - [`editor::EditorPlugin`] which registers every editor panel, resource and system.
/// - [`game::GamePlugin`] which provides a sample scene (camera, lights, primitives) so the
///   viewport is not empty on first launch.
fn main() {
    // Initialize logging early so plugin/system panics leave a trace.
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,wgpu=error,naga=warn")),
        )
        .with_file(true)
        .with_line_number(true)
        .try_init();

    info!("Starting Bevy Editor v{}", env!("CARGO_PKG_VERSION"));

    App::new()
        // ---- Bevy core ----
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Engine Editor".to_string(),
                resolution: (1600.0, 900.0),
                present_mode: bevy::window::PresentMode::AutoVsync,
                visible: true,
                resizable: true,
                decorations: true,
                ..default()
            }),
            ..default()
        }))
        // ---- Egui ----
        .add_plugins(EguiPlugin)
        // ---- Physics ----
        .add_plugins(bevy_rapier3d::prelude::RapierPhysicsPlugin::<()>::default())
        .add_plugins(bevy_rapier3d::prelude::RapierDebugRenderPlugin::default())
        // ---- Editor ----
        .add_plugins(editor::EditorPlugin)
        // ---- Sample game content ----
        .add_plugins(game::GamePlugin)
        .run();
}
