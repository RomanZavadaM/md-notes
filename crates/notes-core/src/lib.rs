//! Core library of MD Notes.
//!
//! The crate is UI-agnostic: it knows how to open a vault (a folder with
//! Markdown notes), list its contents, read, write and parse notes. The
//! Tauri application and any future tools are thin layers on top of it.

pub mod attachments;
pub mod error;
pub mod git_sync;
pub mod graph;
pub mod index;
pub mod markdown;
pub mod note;
pub mod paths;
pub mod presets;
pub mod refactor;
pub mod schema;
pub mod storage;
pub mod sync;
pub mod templates;
pub mod vault;
pub mod vault_storage;

pub use attachments::AttachmentInfo;
pub use error::{Error, Result};
pub use git_sync::{
    clone_git_repository, commit_git_worktree, git_has_changes, open_git_repository,
    validate_git_remote, GitRepositoryInfo,
};
pub use graph::{knowledge_graph, GraphEdge, GraphNode, KnowledgeGraph};
pub use index::{Backlink, Index, NoteSummary, SearchHit, SyncStats, TagCount, UnresolvedLink};
pub use markdown::WikiLink;
pub use note::Note;
pub use presets::{create_vault_with_preset, VaultPreset};
pub use refactor::{rename_with_links, RenameOutcome};
pub use schema::{FieldKind, FieldSpec, NoteTypeSpec, SchemaDocument};
pub use storage::{LocalFsProvider, StorageEntry, StorageEntryKind, StorageProvider};
pub use sync::{
    conflict_copy_path, plan_sync, SyncDecision, SyncManifest, SyncSnapshot, SyncStateStore,
};
pub use templates::TemplateInfo;
pub use vault::{EntryKind, TreeEntry, Vault, VaultConfig};
pub use vault_storage::VaultStorage;
