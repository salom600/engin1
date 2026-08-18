//! Asset database: scans `assets/`, tracks files, exposes them to the Asset Browser panel.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

/// One entry in the [`AssetDatabase`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetEntry {
    /// Human-readable file name (without directory).
    pub name: String,
    /// Absolute path to the file.
    pub path: PathBuf,
    /// Path relative to the project `assets/` folder.
    pub relative_path: String,
    /// What kind of asset this is (image / audio / model / ...).
    pub kind: AssetKind,
    /// File size in bytes.
    pub size_bytes: u64,
    /// Last-modified timestamp.
    pub modified: SystemTime,
}

/// Classification of an asset based on its extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssetKind {
    /// PNG / JPEG / HDR / KTX2 textures.
    Texture,
    /// GLTF / GLB 3D models.
    Model,
    /// WAV / MP3 / OGG / FLAC audio.
    Audio,
    /// `.scn.ron` Bevy scene files.
    Scene,
    /// `.lua` / `.rhai` scripts.
    Script,
    /// `.toml` / `.json` / `.ron` config files.
    Config,
    /// Anything else.
    Other,
}

impl AssetKind {
    /// Determine the kind from a file extension.
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_ascii_lowercase().as_str() {
            "png" | "jpg" | "jpeg" | "hdr" | "ktx2" | "tga" | "bmp" => AssetKind::Texture,
            "gltf" | "glb" | "obj" | "fbx" => AssetKind::Model,
            "wav" | "mp3" | "ogg" | "flac" => AssetKind::Audio,
            "scn" | "scn.ron" => AssetKind::Scene,
            "lua" | "rhai" => AssetKind::Script,
            "toml" | "json" | "ron" | "yaml" | "yml" => AssetKind::Config,
            _ => AssetKind::Other,
        }
    }

    /// A short label for the asset kind, suitable for use in the asset browser.
    pub fn label(self) -> &'static str {
        match self {
            AssetKind::Texture => "Texture",
            AssetKind::Model => "Model",
            AssetKind::Audio => "Audio",
            AssetKind::Scene => "Scene",
            AssetKind::Script => "Script",
            AssetKind::Config => "Config",
            AssetKind::Other => "Other",
        }
    }

    /// An emoji / icon character used in the asset browser (kept ASCII-safe).
    pub fn icon(self) -> &'static str {
        match self {
            AssetKind::Texture => "[T]",
            AssetKind::Model => "[M]",
            AssetKind::Audio => "[A]",
            AssetKind::Scene => "[S]",
            AssetKind::Script => "[#]",
            AssetKind::Config => "[C]",
            AssetKind::Other => "[?]",
        }
    }
}

/// The asset database: a flat, in-memory index of everything in `assets/`.
#[derive(Resource, Default, Debug, Clone)]
pub struct AssetDatabase {
    /// All discovered asset entries, sorted by `relative_path`.
    pub entries: Vec<AssetEntry>,
    /// True if the last scan completed without error.
    pub last_scan_ok: bool,
    /// Timestamp of the last scan.
    pub last_scan_at: Option<SystemTime>,
}

impl AssetDatabase {
    /// Rescan the project's `assets/` directory.
    pub fn rescan(&mut self, assets_dir: &std::path::Path) -> std::io::Result<()> {
        let mut entries = Vec::new();
        self.scan_dir(assets_dir, assets_dir, &mut entries)?;
        entries.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
        self.entries = entries;
        self.last_scan_ok = true;
        self.last_scan_at = Some(SystemTime::now());
        Ok(())
    }

    fn scan_dir(
        &self,
        root: &std::path::Path,
        dir: &std::path::Path,
        out: &mut Vec<AssetEntry>,
    ) -> std::io::Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // Skip hidden directories (e.g. `.git`).
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with('.') {
                        continue;
                    }
                }
                self.scan_dir(root, &path, out)?;
            } else if path.is_file() {
                let metadata = entry.metadata()?;
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                let relative_path = path
                    .strip_prefix(root)
                    .ok()
                    .and_then(|p| p.to_str())
                    .unwrap_or("")
                    .replace('\\', "/");
                out.push(AssetEntry {
                    name: path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string(),
                    path: path.clone(),
                    relative_path,
                    kind: AssetKind::from_extension(ext),
                    size_bytes: metadata.len(),
                    modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                });
            }
        }
        Ok(())
    }

    /// Returns the number of assets of a given kind.
    pub fn count_of(&self, kind: AssetKind) -> usize {
        self.entries.iter().filter(|e| e.kind == kind).count()
    }

    /// Returns a filtered iterator over assets of a given kind.
    pub fn of_kind(&self, kind: AssetKind) -> impl Iterator<Item = &AssetEntry> {
        self.entries.iter().filter(move |e| e.kind == kind)
    }

    /// Total size of all assets, in bytes.
    pub fn total_size_bytes(&self) -> u64 {
        self.entries.iter().map(|e| e.size_bytes).sum()
    }
}
