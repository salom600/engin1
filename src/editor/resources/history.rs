//! Undo / redo history.
//!
//! A simplified command pattern: each [`Command`] captures a description and
//! a pair of closures (forward / backward). The [`CommandHistory`] stores
//! them in two stacks (undo / redo).

use bevy::prelude::*;

/// A single undoable / redoable action.
pub struct Command {
    /// Human-readable description shown in the Edit → Undo / Redo menus.
    pub description: String,
    /// Apply the command forward.
    pub do_fn: Box<dyn Fn(&mut bevy::ecs::world::World) + Send + Sync>,
    /// Apply the command backward (undo).
    pub undo_fn: Box<dyn Fn(&mut bevy::ecs::world::World) + Send + Sync>,
}

impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Command")
            .field("description", &self.description)
            .finish()
    }
}

/// History of [`Command`]s with undo / redo support.
#[derive(Resource, Default)]
pub struct CommandHistory {
    undo_stack: Vec<Command>,
    redo_stack: Vec<Command>,
    /// Maximum number of undo steps to keep.
    pub max_history: usize,
}

impl CommandHistory {
    /// Push a new command onto the undo stack and execute it.
    pub fn push_and_execute(&mut self, world: &mut bevy::ecs::world::World, command: Command) {
        (command.do_fn)(world);
        self.undo_stack.push(command);
        self.redo_stack.clear();
        if self.undo_stack.len() > self.max_history.max(1) {
            self.undo_stack.remove(0);
        }
    }

    /// Undo the most recent command (if any).
    pub fn undo(&mut self, world: &mut bevy::ecs::world::World) -> bool {
        if let Some(command) = self.undo_stack.pop() {
            (command.undo_fn)(world);
            self.redo_stack.push(command);
            true
        } else {
            false
        }
    }

    /// Redo the most recently undone command (if any).
    pub fn redo(&mut self, world: &mut bevy::ecs::world::World) -> bool {
        if let Some(command) = self.redo_stack.pop() {
            (command.do_fn)(world);
            self.undo_stack.push(command);
            true
        } else {
            false
        }
    }

    /// Description of the next undo (for menu items), if any.
    pub fn next_undo_description(&self) -> Option<&str> {
        self.undo_stack.last().map(|c| c.description.as_str())
    }

    /// Description of the next redo (for menu items), if any.
    pub fn next_redo_description(&self) -> Option<&str> {
        self.redo_stack.last().map(|c| c.description.as_str())
    }

    /// Clear all history.
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

impl Default for Command {
    fn default() -> Self {
        Self {
            description: "Untitled Command".to_string(),
            do_fn: Box::new(|_| {}),
            undo_fn: Box::new(|_| {}),
        }
    }
}
