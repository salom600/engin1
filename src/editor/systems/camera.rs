//! Editor camera system: orbit / pan / zoom around a target point.
//!
//! The camera is identified by the [`EditorCamera`](crate::editor::components::EditorCamera)
//! marker component. Its orbit state (yaw, pitch, distance, target) is stored
//! in a [`ViewportCamera`](crate::editor::components::ViewportCamera) component.

use crate::editor::components::{EditorCamera, ViewportCamera};
use crate::editor::state::Selection;
use bevy::input::mouse::{MouseButton, MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy_egui::egui;

/// Update the editor camera's transform based on user input.
///
/// Controls:
/// - **Right mouse button + drag** — orbit around the target.
/// - **Middle mouse button + drag** — pan the target.
/// - **Mouse wheel** — zoom in/out.
/// - **WASD** — move the target (forward / left / back / right).
/// - **Q / E** — move the target down / up.
/// - **Shift** (held while moving) — 2× speed.
///
/// Input is ignored whenever egui is consuming pointer events (e.g. when
/// hovering / dragging inside a panel) so the camera doesn't fly off when
/// the user is interacting with the UI.
pub fn orbit_camera_system(
    mut camera_query: Query<(&mut Transform, &mut ViewportCamera), With<EditorCamera>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut ctxs: bevy_egui::EguiContexts,
) {
    let Some(ctx) = ctxs.try_ctx_mut() else {
        return;
    };

    // Don't move the camera when egui is consuming input.
    if ctx.is_using_pointer() || ctx.wants_pointer_input() {
        mouse_motion.clear();
        return;
    }

    let pan_speed = 10.0;
    let zoom_factor = 0.1;
    let rotation_sensitivity = 0.005;

    let boost = if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
        2.0
    } else {
        1.0
    };

    for (mut transform, mut camera) in camera_query.iter_mut() {
        // --- Orbit (RMB + drag) ---
        let mut delta = Vec2::ZERO;
        if buttons.pressed(MouseButton::Right) {
            for ev in mouse_motion.read() {
                delta += ev.delta;
            }
            camera.yaw -= delta.x * rotation_sensitivity;
            camera.pitch = (camera.pitch + delta.y * rotation_sensitivity).clamp(-1.4, 1.4);
        } else {
            mouse_motion.clear();
        }

        // --- Pan (MMB + drag) ---
        if buttons.pressed(MouseButton::Middle) {
            let right = transform.rotation * Vec3::X;
            let up = transform.rotation * Vec3::Y;
            for ev in mouse_motion.read() {
                camera.target -= right * ev.delta.x * 0.01;
                camera.target += up * ev.delta.y * 0.01;
            }
        }

        // --- WASDQE movement ---
        let forward = transform.rotation * -Vec3::Z;
        let right = transform.rotation * Vec3::X;
        let up = Vec3::Y;
        let mut move_dir = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            move_dir += forward;
        }
        if keys.pressed(KeyCode::KeyS) {
            move_dir -= forward;
        }
        if keys.pressed(KeyCode::KeyA) {
            move_dir -= right;
        }
        if keys.pressed(KeyCode::KeyD) {
            move_dir += right;
        }
        if keys.pressed(KeyCode::KeyQ) {
            move_dir -= up;
        }
        if keys.pressed(KeyCode::KeyE) {
            move_dir += up;
        }
        if move_dir != Vec3::ZERO {
            let delta = move_dir.normalize() * pan_speed * boost * time.delta_seconds();
            camera.target += delta;
        }

        // --- Zoom (mouse wheel) ---
        for ev in mouse_wheel.read() {
            camera.distance =
                (camera.distance - ev.y * zoom_factor * camera.distance).clamp(1.0, 1000.0);
        }

        // --- Apply: position camera at the orbit position, looking at the target. ---
        let pos = camera.position();
        transform.translation = pos;
        transform.look_at(camera.target, Vec3::Y);
    }
}

/// Viewport picking: click on an entity in the viewport to select it.
///
/// This is a lightweight stub: the full implementation uses [`bevy_mod_picking`]
/// to raycast against all meshed entities and update the [`Selection`] resource
/// based on which entity was hit. The `bevy_mod_picking` plugin is already
/// registered in the [`crate::editor::EditorPlugin`]; this stub just prevents
/// the system from being unused.
pub fn viewport_picking_system(
    buttons: Res<ButtonInput<MouseButton>>,
    mut ctxs: bevy_egui::EguiContexts,
    _selection: ResMut<Selection>,
) {
    let _ = (buttons, ctxs, _selection);
    // The actual picking is performed by bevy_mod_picking's systems.
    // Here we would normally translate the picking plugin's selected entity
    // into our Selection resource.
}
