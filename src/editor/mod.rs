//! # Editor Plugin
//!
//! The [`EditorPlugin`] is the single integration point between the Bevy app and the editor.
//! It registers every editor-only resource, plugin, system and UI panel.
//!
//! ## What it adds to the [`App`](bevy::app::App)
//!
//! 1. **State machine** — [`EditorState`] (Loading / Editing / Playing / Paused).
//! 2. **Resources** — [`ProjectResource`], [`AssetDatabase`], [`EditorLog`],
//!    [`Selection`], [`EditorSettings`], [`PanelVisibility`].
//! 3. **Egui panels** — every panel under [`panels`] is registered as its own system.
//! 4. **Systems** — camera control, gizmo, picking, save/load, log capture, play-mode sync.
//! 5. **Subsystems** — physics, audio, scripting, AI bridges (under [`systems`]).

use bevy::app::App;
use bevy::prelude::*;

pub mod components;
pub mod panels;
pub mod resources;
pub mod state;
pub mod systems;
pub mod theme;

pub use state::{EditorState, Selection};
pub use theme::EditorTheme;

use resources::{AssetDatabase, EditorLog, EditorSettings, ProjectResource};

/// The editor's main [`Plugin`].
///
/// Adding this plugin to an [`App`] turns it into a fully-featured Bevy editor.
pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        // ----- State machine -----
        app.init_state::<EditorState>();

        // ----- Core resources -----
        app.init_resource::<ProjectResource>()
            .init_resource::<AssetDatabase>()
            .init_resource::<EditorLog>()
            .init_resource::<Selection>()
            .init_resource::<EditorSettings>()
            .init_resource::<panels::PanelVisibility>()
            .init_resource::<resources::CommandHistory>()
            .init_resource::<resources::EditorCameraState>()
            .init_resource::<theme::EditorTheme>();

        // ----- Startup: spawn the editor camera + light + grid -----
        app.add_systems(Startup, setup_editor_camera);

        // ----- Per-frame systems (run during editing / playing / paused) -----
        app.add_systems(
            Update,
            (
                panels::menu_bar::draw_system,
                panels::toolbar::draw_system,
                panels::viewport::draw_system
                    .run_if(|vis: Res<panels::PanelVisibility>| vis.viewport),
                panels::scene_hierarchy::draw_system
                    .run_if(|vis: Res<panels::PanelVisibility>| vis.scene_hierarchy),
                panels::inspector::draw_system
                    .run_if(|vis: Res<panels::PanelVisibility>| vis.inspector),
                panels::asset_browser::draw_system
                    .run_if(|vis: Res<panels::PanelVisibility>| vis.asset_browser),
                panels::console::draw_system
                    .run_if(|vis: Res<panels::PanelVisibility>| vis.console),
                panels::about::draw_system,
            ),
        );

        // ----- Editor logic systems -----
        app.add_systems(
            Update,
            (
                systems::camera::orbit_camera_system,
                systems::camera::viewport_picking_system,
                systems::gizmo::transform_gizmo_system,
                systems::history::record_undo_system,
                systems::log::capture_log_system,
                systems::play_mode::play_mode_sync_system,
                systems::save_load::autosave_system,
            ),
        );

        // ----- Subsystem integrations -----
        app.add_plugins(systems::physics::PhysicsBridgePlugin)
            .add_plugins(systems::audio::AudioBridgePlugin)
            .add_plugins(systems::scripting::ScriptingBridgePlugin)
            .add_plugins(systems::ai::AIBridgePlugin)
            .add_plugins(systems::assets::AssetPipelinePlugin);

        // ----- Asset scan on startup -----
        app.add_systems(Update, systems::assets::rescan_on_startup_system);

        info!("EditorPlugin initialized.");
    }
}

/// Spawn the editor's viewport camera and a default light + grid.
fn setup_editor_camera(
    mut commands: Commands,
    settings: Res<EditorSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Editor camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(8.0, 6.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                order: 0,
                ..default()
            },
            ..default()
        },
        components::EditorCamera,
        components::ViewportCamera {
            yaw: std::f32::consts::FRAC_PI_4,
            pitch: std::f32::consts::FRAC_PI_6,
            distance: 12.0,
            target: Vec3::ZERO,
        },
        Name::new("Editor Camera"),
    ));

    // Editor-only directional light (illuminates the scene in edit mode)
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 10_000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        components::EditorLight,
        Name::new("Editor Light"),
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
    });

    // Spawn a default ground plane + grid (only if grid is enabled in settings)
    if settings.show_grid {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(bevy::math::primitives::Plane3d::default()),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(0.2, 0.2, 0.22),
                    ..default()
                }),
                transform: Transform::from_scale(Vec3::new(50.0, 1.0, 50.0)),
                ..default()
            },
            bevy_rapier3d::prelude::Collider::cuboid(50.0, 0.1, 50.0),
            bevy_rapier3d::prelude::RigidBody::Fixed,
            Name::new("Ground Plane"),
        ));
    }

    info!("Editor scene spawned: camera + light + ground plane");
}
