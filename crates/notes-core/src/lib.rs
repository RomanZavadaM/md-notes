//! Core library of MD Notes.
//!
//! The crate is UI-agnostic: it knows how to open a vault (a folder with
//! Markdown notes), list its contents, read, write and parse notes. The
//! Tauri application and any future tools are thin layers on top of it.

pub mod error;
pub mod markdown;
pub mod note;
pub mod paths;
pub mod vault;

pub use error::{Error, Result};
pub use markdown::WikiLink;
pub use note::Note;
pub use vault::{EntryKind, TreeEntry, Vault, VaultConfig};
