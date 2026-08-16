//! The editor log buffer: a ring buffer of recent log lines used by the Console panel.

use bevy::prelude::*;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::SystemTime;

/// One entry in the [`EditorLog`].
#[derive(Debug, Clone)]
pub struct LogEntry {
    /// When the message was logged.
    pub timestamp: SystemTime,
    /// Severity / level of the message.
    pub level: LogLevel,
    /// The message body.
    pub message: String,
    /// Module / file that emitted the message (if known).
    pub target: Option<String>,
}

/// Severity of a [`LogEntry`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogLevel {
    /// Verbose / debug-level trace.
    Trace,
    /// Informational.
    Info,
    /// Warning.
    Warn,
    /// Error.
    Error,
}

impl LogLevel {
    /// A short label for the level.
    pub fn label(self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }

    /// Whether this level should be displayed given a `min_level`.
    pub fn at_least(self, min_level: LogLevel) -> bool {
        let order = |l: LogLevel| match l {
            LogLevel::Trace => 0,
            LogLevel::Info => 1,
            LogLevel::Warn => 2,
            LogLevel::Error => 3,
        };
        order(self) >= order(min_level)
    }
}

/// The shared editor log buffer.
///
/// Stored as a `Mutex<Vec<_>>` inside an `Arc` because:
/// 1. The buffer is written to from the [`log::capture_log_system`] every frame.
/// 2. It is read from the egui draw context (which doesn't have a `&mut World`).
/// 3. The egui draw function only has a `&EditorLog`, so we need interior mutability.
#[derive(Resource, Clone, Default)]
pub struct EditorLog {
    inner: Arc<Mutex<Vec<LogEntry>>>,
}

impl EditorLog {
    /// Push a new log entry.
    pub fn push(&self, level: LogLevel, message: impl Into<String>) {
        let entry = LogEntry {
            timestamp: SystemTime::now(),
            level,
            message: message.into(),
            target: None,
        };
        let mut guard = self.inner.lock();
        guard.push(entry);
        // Keep at most 10k entries to bound memory.
        if guard.len() > 10_000 {
            let drop = guard.len() - 10_000;
            guard.drain(0..drop);
        }
    }

    /// Push a new log entry with a target.
    pub fn push_with_target(
        &self,
        level: LogLevel,
        target: impl Into<String>,
        message: impl Into<String>,
    ) {
        let entry = LogEntry {
            timestamp: SystemTime::now(),
            level,
            message: message.into(),
            target: Some(target.into()),
        };
        let mut guard = self.inner.lock();
        guard.push(entry);
        if guard.len() > 10_000 {
            let drop = guard.len() - 10_000;
            guard.drain(0..drop);
        }
    }

    /// Drain all entries (used by the console to render the latest snapshot).
    pub fn snapshot(&self) -> Vec<LogEntry> {
        self.inner.lock().clone()
    }

    /// Clear all log entries.
    pub fn clear(&self) {
        self.inner.lock().clear();
    }

    /// The number of entries currently in the buffer.
    pub fn len(&self) -> usize {
        self.inner.lock().len()
    }

    /// Whether the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.lock().is_empty()
    }
}
