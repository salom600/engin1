# Bevy Engine Editor

A complete, integrated game engine editor built on top of the [Bevy engine](https://bevyengine.org)
using [`bevy_egui`](https://github.com/mvlabat/bevy_egui) for the editor UI.

![Status](https://img.shields.io/badge/status-alpha-orange)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)
![Bevy](https://img.shields.io/badge/Bevy-0.14-red)
![Rust](https://img.shields.io/badge/Rust-stable-orange)

## Features

- **3D Viewport** — Orbit camera, transform gizmo, entity picking, grid + axes overlay
- **Scene Hierarchy** — Parent / child tree with rename, duplicate, delete, lock, hide
- **Inspector** — Reflection-based editor for Transform, Visibility, Mesh, Material, Physics, Audio, Script, AI
- **Asset Browser** — Drag-and-drop import, organized by kind (Texture / Model / Audio / Scene / Script / Config), re-scan, compress
- **Console** — Log panel with severity filter, search, command input (`help`, `clear`, `version`, `scan`, `save`)
- **Toolbar** — Play / Pause / Stop / Step, Save, Load, Undo, Redo, Build Debug / Release
- **Menu Bar** — File / Edit / View / Asset / Build / Help
- **Physics** — Rapier3D integration with debug renderer, manual step in pause mode
- **Audio** — `bevy_audio` integration with preview, volume, 3D positional audio
- **Scripting** — `bevy_mod_scripting` Lua integration with hot-reload (stub)
- **AI** — `big-brain` utility AI integration with debug visualization (stub)
- **Asset Pipeline** — Periodic rescan, organize, compress textures / audio
- **Undo / Redo** — Command history with up to 100 undoable steps
- **Autosave** — Configurable interval (default: every 5 minutes)
- **Multi-platform CI/CD** — GitHub Actions builds for Windows / macOS / Linux

## Quick Start

### Prerequisites

- Rust toolchain (stable, 1.74+)
- System dependencies (Linux only):
  ```bash
  sudo apt install libwayland-dev libxkbcommon-dev libasound2-dev libudev-dev pkg-config
  ```

### Build & Run

```bash
git clone https://github.com/salom600/engin1.git
cd engin1
cargo run --release
```

For faster dev iteration:

```bash
cargo run --features dynamic
```

### Build for Production

```bash
cargo build --release --features production
```

## Project Structure

```
src/
├── main.rs                       Entry point, plugin setup
├── editor/
│   ├── mod.rs                    EditorPlugin: registers all resources/systems/panels
│   ├── state.rs                  EditorState, Selection
│   ├── theme.rs                  Egui theme (Dark / Light)
│   ├── resources/
│   │   ├── project.rs            Active project (name, paths, recent scenes)
│   │   ├── asset_db.rs           Asset database (scans assets/)
│   │   ├── editor_log.rs         Log buffer (used by Console panel)
│   │   ├── history.rs            Undo / redo command stack
│   │   └── settings.rs           Editor settings (theme, autosave, gizmo)
│   ├── components/
│   │   └── mod.rs                Marker components (EditorCamera, Selected, etc.)
│   ├── panels/
│   │   ├── menu_bar.rs           Top menu bar (File / Edit / View / Asset / Build / Help)
│   │   ├── toolbar.rs            Play / Pause / Stop / Save / Load
│   │   ├── viewport.rs           3D scene rendering + status overlays
│   │   ├── scene_hierarchy.rs    Entity tree
│   │   ├── inspector.rs          Component editor
│   │   ├── asset_browser.rs      File browser for assets/
│   │   ├── console.rs            Log + command input
│   │   └── about.rs              About + Settings dialogs
│   └── systems/
│       ├── camera.rs             Orbit camera + picking
│       ├── gizmo.rs              Transform gizmo
│       ├── history.rs            Undo recording
│       ├── log.rs                Log capture
│       ├── play_mode.rs          Play / Pause / Stop state sync
│       ├── save_load.rs          Autosave
│       ├── physics.rs            Rapier3D bridge
│       ├── audio.rs              bevy_audio bridge
│       ├── scripting.rs          bevy_mod_scripting bridge
│       ├── ai.rs                 big-brain bridge
│       └── assets.rs             Asset pipeline (rescan, organize, compress)
└── game/
    └── mod.rs                    Sample game content (starter scene)
```

## Architecture

The editor is structured as a single [`EditorPlugin`] that registers all
editor-only resources, panels, and systems with the Bevy [`App`]. The user's
game code is a separate [`GamePlugin`] that can be enabled / disabled at will.

```text
                    ┌──────────────────────────┐
                    │       Bevy Engine         │
                    │   (Default plugins)       │
                    └──────────┬───────────────┘
                               │
              ┌────────────────┼───────────────┐
              │                │               │
       ┌──────▼───────┐  ┌─────▼──────┐  ┌─────▼──────┐
       │ EditorPlugin │  │ GamePlugin │  │  CI/CD     │
       │ - Panels    │  │ - Scene    │  │ - Workflow │
       │ - Systems   │  │ - Camera   │  │ - Builds   │
       │ - Resources │  │ - Logic    │  │ - Releases │
       └──────────────┘  └────────────┘  └────────────┘
```

## Editor Panels

| Panel            | Default Location | Purpose                                         |
|------------------|------------------|-------------------------------------------------|
| Menu Bar         | Top              | File / Edit / View / Asset / Build / Help menus |
| Toolbar          | Below menu bar  | Play / Pause / Stop / Save / Load / Build       |
| Viewport         | Center          | 3D scene rendering with orbit camera + gizmo    |
| Scene Hierarchy  | Left            | Entity tree with parent / child display         |
| Inspector        | Right           | Component editor for selected entity            |
| Asset Browser    | Bottom          | File browser for `assets/`                      |
| Console          | Bottom          | Log + command input                             |

## Controls

### Viewport Camera

| Input                              | Action                |
|------------------------------------|-----------------------|
| Right mouse button + drag           | Orbit around target   |
| Middle mouse button + drag         | Pan target            |
| Mouse wheel                        | Zoom in / out         |
| `W` / `S`                          | Move forward / back   |
| `A` / `D`                          | Move left / right     |
| `Q` / `E`                          | Move down / up        |
| `Shift` (held)                     | 2× movement speed     |

### Console Commands

| Command  | Description                  |
|----------|------------------------------|
| `help`   | List available commands      |
| `clear`  | Clear the log buffer         |
| `version`| Print editor version         |
| `scan`   | Re-scan the assets folder    |
| `save`   | Save the current scene       |

## CI/CD

The repository includes a [GitHub Actions workflow](.github/workflows/ci.yml)
that:

1. Builds the editor on Windows, macOS, and Linux in parallel.
2. Runs `cargo fmt --check`, `cargo clippy`, and `cargo test`.
3. Uploads build artifacts (one per platform) on every push.
4. Creates a GitHub Release with binaries on every tag.

See [.github/workflows/ci.yml](.github/workflows/ci.yml) for details.

## License

Dual-licensed under MIT or Apache-2.0, at your option.

## Acknowledgments

- [Bevy](https://bevyengine.org) — The game engine
- [bevy_egui](https://github.com/mvlabat/bevy_egui) — Immediate-mode GUI for Bevy
- [bevy_rapier3d](https://github.com/dimforge/bevy_rapier) — 3D physics
- [bevy-inspector-egui](https://github.com/jakobhellermann/bevy-inspector-egui) — Reflection-based inspector
- [bevy_mod_picking](https://github.com/aevyrie/bevy_mod_picking) — Entity picking
- [egui](https://github.com/emilk/egui) — Immediate-mode GUI library
