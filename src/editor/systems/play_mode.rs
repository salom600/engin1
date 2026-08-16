//! Play-mode sync system: bridges editor ↔ game state.
//!
//! When the user presses Play:
//! 1. The editor's state transitions to [`EditorState::Playing`].
//! 2. The game's logic systems start running.
//! 3. The inspector becomes read-only.
//!
//! When the user presses Stop:
//! 1. The editor's state transitions back to [`EditorState::Editing`].
//! 2. The game's logic systems stop running.
//! 3. The scene is reverted to the last-saved state.

use crate::editor::state::EditorState;
use bevy::prelude::*;

/// Sync game-side state with the editor's [`EditorState`].
pub fn play_mode_sync_system(
    _current_state: Res<State<EditorState>>,
    _next_state: ResMut<NextState<EditorState>>,
    _time: Res<Time>,
) {
    // Stub: the state machine itself handles the transitions; this system
    // is a place for additional logic that needs to run on transitions
    // (e.g. clearing the game's temporary entities when stopping play mode).
    let _ = (_current_state, _next_state, _time);
}
