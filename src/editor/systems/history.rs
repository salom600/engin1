//! Undo / redo recording.
//!
//! Watches for user-initiated entity edits and pushes [`Command`]s onto the
//! [`CommandHistory`] so that they become undoable.
//!
//! This is a stub — a full implementation would diff the entity registry
//! every frame (or use Bevy's change detection) and synthesize [`Command`]s
//! that capture the before / after state.

use crate::editor::resources::CommandHistory;
use bevy::prelude::*;

/// Watch for entity edits and push undo records.
pub fn record_undo_system(_history: Res<CommandHistory>) {
    // Stub: in a real implementation, we'd:
    // 1. Listen for Added<Component> events.
    // 2. Capture the previous value (if any) via reflection.
    // 3. Push a Command onto CommandHistory with do / undo closures that
    //    insert / remove the component on the affected entity.
    let _ = _history;
}
