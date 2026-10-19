//! Core library of MD Notes.
//!
//! The crate is UI-agnostic: it knows how to open a vault (a folder with
//! Markdown notes), list its contents, read, write and parse notes. The
//! Tauri application and any future tools are thin layers on top of it.

pub mod attachments;
#[cfg(feature = "encrypted-vault")]
pub mod encrypted;
#[cfg(feature = "encrypted-vault")]
pub mod encrypted_vault;
pub mod error;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub mod git_manage;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub mod git_push;
pub mod git_sync;
pub mod graph;
pub mod index;
pub mod markdown;
pub mod note;
pub mod paths;
pub mod presets;
pub mod push_guard;
pub mod refactor;
pub mod schema;
pub mod storage;
pub mod sync;
pub mod templates;
pub mod vault;
pub mod vault_storage;

pub use attachments::AttachmentInfo;
#[cfg(feature = "encrypted-vault")]
pub use encrypted::{EncryptedProvider, NameLayout, ENCRYPTED_SUFFIX};
pub use error::{Error, Result};
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub use git_manage::{git_init, git_set_remote, git_status, tracking_reference, GitStatus};
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub use git_push::{push_git_branch, GitPushOutcome};
pub use git_sync::{
    apply_git_fast_forward, classify_git_pull_relation, clone_git_repository, commit_git_worktree,
    fetch_git_remote_authenticated, fetch_git_remote_public, git_has_changes, open_git_repository,
    plan_git_pull, plan_git_pull_preflight, validate_git_remote, GitHttpsCredentials,
    GitPullPreflight, GitPullRelation, GitRepositoryInfo,
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
