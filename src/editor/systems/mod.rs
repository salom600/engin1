//! Editor systems: camera control, gizmo, picking, save/load, log capture,
//! play-mode sync, undo/redo, and the per-subsystem bridges
//! (physics / audio / scripting / AI / assets).

pub mod ai;
pub mod assets;
pub mod audio;
pub mod camera;
pub mod gizmo;
pub mod history;
pub mod log;
pub mod physics;
pub mod play_mode;
pub mod save_load;
pub mod scripting;
