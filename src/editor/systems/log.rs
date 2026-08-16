//! Log capture system: pushes tracing / log messages into the [`EditorLog`]
//! buffer so the Console panel can display them.

use crate::editor::resources::{EditorLog, LogLevel};
use bevy::prelude::*;

/// Periodically pushes a "heartbeat" entry into the log so the user can see
/// the editor is alive. In a real implementation, this system would also
/// subscribe to `tracing` events and convert them into [`EditorLog`] entries.
pub fn capture_log_system(editor_log: Res<EditorLog>, time: Res<Time>) {
    // Throttle: emit a heartbeat every 5 seconds.
    let now = time.elapsed_seconds();
    if (now as u64) % 5 == 0 && (now * 100.0) as u64 % 100 < 2 {
        editor_log.push_with_target(
            LogLevel::Trace,
            "editor::heartbeat",
            format!("alive @ {:.2}s", now),
        );
    }
}
