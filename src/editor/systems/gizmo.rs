//! Transform gizmo: drag the selected entity's transform around with a visual gizmo.
//!
//! This is a stub — the real implementation uses [`bevy_transform_gizmo`] to
//! draw a gizmo on the selected entity and apply drag deltas back to the
//! [`Transform`] component.

use crate::editor::state::Selection;
use bevy::prelude::*;

/// Draw / update the transform gizmo on the selected entity.
pub fn transform_gizmo_system(
    selection: Res<Selection>,
    mut transforms: Query<&mut Transform>,
    _settings: Res<crate::editor::resources::EditorSettings>,
) {
    let Some(primary) = selection.primary else {
        return;
    };
    let Ok(mut _transform) = transforms.get_mut(primary) else {
        return;
    };
    // The bevy_transform_gizmo plugin handles its own rendering & input.
    // Here we would normally sync any gizmo-imposed changes back to our
    // CommandHistory (so that they're undoable), but the stub just exists
    // to keep the system wiring consistent.
    let _ = _transform;
}
