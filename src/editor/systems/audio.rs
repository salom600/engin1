//! Audio bridge: integrates `bevy_audio` with the editor.
//!
//! Provides:
//! - Play / stop / loop audio from the Asset Browser.
//! - Volume control.
//! - 3D positional audio (via `AudioSink` + `Transform`).

use bevy::prelude::*;

/// Plugin that registers the audio bridge systems.
pub struct AudioBridgePlugin;

impl Plugin for AudioBridgePlugin {
    fn build(&self, app: &mut App) {
        // Audio is provided by the DefaultPlugins set in main.rs; here we just
        // register the editor-side helpers (volume control, asset preview).
        app.add_systems(Update, preview_audio_system);
        info!("AudioBridgePlugin initialized.");
    }
}

/// Listen for "preview audio" requests (from the Asset Browser) and play them.
///
/// This is a stub — the real implementation would read an event queue
/// populated by the Asset Browser panel and spawn `AudioBundle` entities
/// for each requested file.
fn preview_audio_system(_commands: Commands) {
    // In Bevy 0.14, to play audio you'd typically do:
    //   commands.spawn(AudioBundle { source: audio_handle, ..default() });
    // and then optionally add an `AudioSink` to control playback.
    let _ = _commands;
}
