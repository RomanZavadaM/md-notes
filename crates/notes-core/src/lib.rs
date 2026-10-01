//! Core library of MD Notes.
//!
//! The crate is UI-agnostic: it knows how to open a vault (a folder with
//! Markdown notes), list its contents, read, write and parse notes. The
//! Tauri application and any future tools are thin layers on top of it.

pub mod error;
pub mod index;
pub mod markdown;
pub mod note;
pub mod paths;
pub mod refactor;
pub mod vault;

pub use error::{Error, Result};
pub use index::{Backlink, Index, NoteSummary, SearchHit, SyncStats, TagCount, UnresolvedLink};
pub use markdown::WikiLink;
pub use note::Note;
pub use refactor::{rename_with_links, RenameOutcome};
pub use vault::{EntryKind, TreeEntry, Vault, VaultConfig};
