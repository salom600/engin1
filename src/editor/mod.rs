//! # Editor Plugin
//!
//! The [`EditorPlugin`] is the single integration point between the Bevy app
//! and the editor. It registers every editor-only resource, plugin, system
//! and UI panel.
//!
//! ## What it adds to the [`App`](bevy::app::App)
//!
//! 1. **State machine** — [`EditorState`] (Loading / Editing / Playing / Paused).
//! 2. **Resources** — [`ProjectResource`], [`AssetDatabase`], [`EditorLog`],
//!    [`Selection`], [`EditorSettings`], [`PanelVisibility`].
//! 3. **UI** — the single [`crate::editor::layout::draw_editor_ui`] system
//!    that draws ALL egui panels in the correct order.
//! 4. **Systems** — camera control, gizmo, picking, save/load, log capture.
//! 5. **Subsystems** — physics, audio, scripting, AI bridges.

use bevy::app::App;
use bevy::prelude::*;

pub mod components;
pub mod layout;
pub mod panels;
pub mod resources;
pub mod state;
pub mod systems;
pub mod theme;

pub use state::{EditorState, Selection};
pub use theme::EditorTheme;

use resources::{AssetDatabase, EditorLog, EditorSettings, ProjectResource};

/// The editor's main [`Plugin`].
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
            .init_resource::<panels::BottomTab>()
            .init_resource::<panels::ConsoleState>()
            .init_resource::<panels::HierarchyState>()
            .init_resource::<panels::AssetBrowserState>()
            .init_resource::<panels::PendingActions>()
            .init_resource::<resources::CommandHistory>()
            .init_resource::<resources::EditorCameraState>()
            .init_resource::<theme::EditorTheme>()
            .init_resource::<state::CurrentScenePath>()
            .init_resource::<systems::play_mode::PlayModeSnapshot>();

        // ----- Register types for scene serialization -----
        app.register_type::<components::SceneEntity>()
            .register_type::<components::Selected>()
            .register_type::<components::Hidden>()
            .register_type::<components::Locked>();

        // ----- Editor events -----
        app.add_event::<components::SpawnRequest>()
            .add_event::<components::DeleteEntityRequest>()
            .add_event::<components::RenameEntityRequest>()
            .add_event::<components::DuplicateEntityRequest>()
            .add_event::<components::SaveSceneRequest>()
            .add_event::<components::LoadSceneRequest>();

        // ----- Startup: spawn the editor camera + light + grid -----
        app.add_systems(Startup, setup_editor_camera);

        // ----- Master UI layout system -----
        app.add_systems(Update, layout::draw_editor_ui);

        // ----- Editor action systems (process PendingActions → real ECS mutations) -----
        // Non-exclusive systems can run in parallel:
        app.add_systems(
            Update,
            (
                flush_pending_actions,
                systems::spawn::handle_spawn_requests,
                systems::edit::handle_delete_requests,
                systems::edit::handle_rename_requests,
                systems::edit::handle_duplicate_requests,
                systems::edit::cleanup_selection_after_despawn,
                systems::save_load::autosave_system,
            ),
        );
        // Exclusive systems (&mut World) must be registered separately:
        app.add_systems(Update, systems::save_load::handle_save_requests);
        app.add_systems(Update, systems::save_load::handle_load_requests);

        // ----- Play mode snapshot/restore (exclusive systems) -----
        app.add_systems(
            OnEnter(EditorState::Playing),
            systems::play_mode::snapshot_scene_before_play,
        )
        .add_systems(
            OnExit(EditorState::Playing),
            systems::play_mode::restore_scene_after_play,
        )
        .add_systems(Update, systems::play_mode::play_mode_sync_system);

        // ----- Editor logic systems -----
        app.add_systems(
            Update,
            (
                systems::camera::orbit_camera_system,
                systems::camera::viewport_picking_system,
                systems::gizmo::transform_gizmo_system,
                systems::history::record_undo_system,
                systems::log::capture_log_system,
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

        info!("EditorPlugin initialized — functional editor with real ECS mutations.");
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

    // Editor-only directional light
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

/// Flush pending editor actions (from the UI) into Bevy events.
///
/// The UI system writes to `PendingActions` (a resource) because it can't
/// hold `EventWriter` params (it's already at the 14-param limit). This
/// system drains `PendingActions` into the proper event channels, which
/// the spawn/edit/save/load systems then read.
fn flush_pending_actions(
    mut pending: ResMut<panels::PendingActions>,
    mut spawn_writer: EventWriter<components::SpawnRequest>,
    mut delete_writer: EventWriter<components::DeleteEntityRequest>,
    mut rename_writer: EventWriter<components::RenameEntityRequest>,
    mut duplicate_writer: EventWriter<components::DuplicateEntityRequest>,
    mut save_writer: EventWriter<components::SaveSceneRequest>,
    mut load_writer: EventWriter<components::LoadSceneRequest>,
) {
    for req in pending.spawns.drain(..) {
        spawn_writer.send(req);
    }
    for req in pending.deletes.drain(..) {
        delete_writer.send(req);
    }
    for req in pending.renames.drain(..) {
        rename_writer.send(req);
    }
    for req in pending.duplicates.drain(..) {
        duplicate_writer.send(req);
    }
    if pending.save {
        save_writer.send(components::SaveSceneRequest);
        pending.save = false;
    }
    if let Some(path) = pending.load.take() {
        load_writer.send(components::LoadSceneRequest { path });
    }
}
