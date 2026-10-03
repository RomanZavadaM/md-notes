use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{io_err, Error, Result};

const SYNC_STATE_FILE: &str = "sync-state.json";
const TMP_SUFFIX: &str = ".mdnotes-tmp";

/// Last-known state of one side of a synchronized path.
///
/// `content_hash == None` means the path is absent/deleted. Hash computation is
/// deliberately adapter-owned so Git/WebDAV can use the same planner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SyncSnapshot {
    pub content_hash: Option<String>,
    pub remote_revision: Option<String>,
}

/// Rebuildable sync metadata stored in `.mdnotes/cache/`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SyncManifest {
    pub version: u32,
    pub files: BTreeMap<String, SyncSnapshot>,
}

impl SyncManifest {
    pub fn v1() -> Self {
        Self {
            version: 1,
            files: BTreeMap::new(),
        }
    }
}

/// Local persistence for rebuildable sync metadata.
///
/// The store lives under the vault cache directory and is deliberately kept
/// separate from user Markdown. Writes are atomic so an interrupted sync cannot
/// leave a partially written manifest.
#[derive(Debug, Clone)]
pub struct SyncStateStore {
    path: PathBuf,
}

impl SyncStateStore {
    pub fn in_cache_dir(cache_dir: impl AsRef<Path>) -> Self {
        Self {
            path: cache_dir.as_ref().join(SYNC_STATE_FILE),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn load(&self) -> Result<SyncManifest> {
        if !self.path.exists() {
            return Ok(SyncManifest::v1());
        }
        let raw = fs::read(&self.path).map_err(io_err(&self.path))?;
        let manifest: SyncManifest =
            serde_json::from_slice(&raw).map_err(|error| Error::Sync(error.to_string()))?;
        if manifest.version != 1 {
            return Err(Error::Sync(format!(
                "unsupported sync manifest version: {}",
                manifest.version
            )));
        }
        Ok(manifest)
    }

    pub fn save(&self, manifest: &SyncManifest) -> Result<()> {
        if manifest.version != 1 {
            return Err(Error::Sync(format!(
                "unsupported sync manifest version: {}",
                manifest.version
            )));
        }
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(io_err(parent))?;
        }
        let mut bytes =
            serde_json::to_vec_pretty(manifest).map_err(|error| Error::Sync(error.to_string()))?;
        bytes.push(b'\n');

        let mut tmp = self.path.as_os_str().to_owned();
        tmp.push(TMP_SUFFIX);
        let tmp = PathBuf::from(tmp);
        fs::write(&tmp, bytes).map_err(io_err(&tmp))?;
        fs::rename(&tmp, &self.path).map_err(io_err(&self.path))
    }
}

/// Provider-neutral decision for one vault-relative path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncDecision {
    NoChange,
    UploadLocal,
    DownloadRemote,
    DeleteRemote,
    DeleteLocal,
    /// Both sides contain different changed content. Text adapters should try
    /// a three-way merge using the cached base; otherwise create a conflict copy.
    MergeOrConflict,
}

/// Plans one path according to ADR-0005.
///
/// `base` is the last synchronized state; `local` and `remote` are current
/// states. A missing state is represented by `content_hash: None`.
pub fn plan_sync(base: &SyncSnapshot, local: &SyncSnapshot, remote: &SyncSnapshot) -> SyncDecision {
    let local_changed = local.content_hash != base.content_hash;
    let remote_changed = remote.content_hash != base.content_hash;

    match (local_changed, remote_changed) {
        (false, false) => SyncDecision::NoChange,
        (true, false) => match &local.content_hash {
            Some(_) => SyncDecision::UploadLocal,
            None => SyncDecision::DeleteRemote,
        },
        (false, true) => match &remote.content_hash {
            Some(_) => SyncDecision::DownloadRemote,
            None => SyncDecision::DeleteLocal,
        },
        (true, true) => {
            if local.content_hash == remote.content_hash {
                return SyncDecision::NoChange;
            }

            // ADR-0005: deletion versus a modification keeps the modified file.
            match (&local.content_hash, &remote.content_hash) {
                (None, Some(_)) => SyncDecision::DownloadRemote,
                (Some(_), None) => SyncDecision::UploadLocal,
                (Some(_), Some(_)) => SyncDecision::MergeOrConflict,
                (None, None) => SyncDecision::NoChange,
            }
        }
    }
}

/// Builds the visible conflict-copy path required by ADR-0005.
pub fn conflict_copy_path(path: &str, stamp: &str, device: &str) -> String {
    let safe_device: String = device
        .chars()
        .map(|ch| match ch {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            other => other,
        })
        .collect();

    let (stem, extension) = match path.rsplit_once('.') {
        Some((stem, extension)) if !stem.is_empty() && !extension.contains('/') => {
            (stem, format!(".{extension}"))
        }
        _ => (path, String::new()),
    };
    format!("{stem} (конфлікт {stamp}, {safe_device}){extension}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state(hash: Option<&str>) -> SyncSnapshot {
        SyncSnapshot {
            content_hash: hash.map(str::to_string),
            remote_revision: None,
        }
    }

    #[test]
    fn sync_state_store_round_trips_and_defaults_when_missing() {
        let dir = tempfile::tempdir().unwrap();
        let store = SyncStateStore::in_cache_dir(dir.path());
        assert_eq!(store.load().unwrap(), SyncManifest::v1());

        let mut manifest = SyncManifest::v1();
        manifest.files.insert(
            "Notes/Idea.md".into(),
            SyncSnapshot {
                content_hash: Some("abc".into()),
                remote_revision: Some("etag-1".into()),
            },
        );
        store.save(&manifest).unwrap();
        assert_eq!(store.load().unwrap(), manifest);
        assert!(store.path().is_file());
    }

    #[test]
    fn sync_state_store_rejects_invalid_or_unknown_versions() {
        let dir = tempfile::tempdir().unwrap();
        let store = SyncStateStore::in_cache_dir(dir.path());
        fs::write(store.path(), b"not json").unwrap();
        assert!(matches!(store.load(), Err(Error::Sync(_))));

        let invalid = SyncManifest {
            version: 2,
            files: BTreeMap::new(),
        };
        assert!(matches!(store.save(&invalid), Err(Error::Sync(_))));
    }

    #[test]
    fn propagates_one_sided_changes_and_deletions() {
        let base = state(Some("base"));
        assert_eq!(
            plan_sync(&base, &state(Some("local")), &base),
            SyncDecision::UploadLocal
        );
        assert_eq!(
            plan_sync(&base, &base, &state(Some("remote"))),
            SyncDecision::DownloadRemote
        );
        assert_eq!(
            plan_sync(&base, &state(None), &base),
            SyncDecision::DeleteRemote
        );
        assert_eq!(
            plan_sync(&base, &base, &state(None)),
            SyncDecision::DeleteLocal
        );
    }

    #[test]
    fn modification_wins_over_concurrent_deletion() {
        let base = state(Some("base"));
        assert_eq!(
            plan_sync(&base, &state(None), &state(Some("remote-change"))),
            SyncDecision::DownloadRemote
        );
        assert_eq!(
            plan_sync(&base, &state(Some("local-change")), &state(None)),
            SyncDecision::UploadLocal
        );
    }

    #[test]
    fn divergent_changes_require_merge_or_conflict_copy() {
        let base = state(Some("base"));
        assert_eq!(
            plan_sync(
                &base,
                &state(Some("local-change")),
                &state(Some("remote-change"))
            ),
            SyncDecision::MergeOrConflict
        );
        assert_eq!(
            plan_sync(&base, &state(Some("same")), &state(Some("same"))),
            SyncDecision::NoChange
        );
    }

    #[test]
    fn conflict_copy_keeps_extension_and_sanitizes_device() {
        assert_eq!(
            conflict_copy_path("Notes/Idea.md", "2026-10-03 11-30", "Pixel:8"),
            "Notes/Idea (конфлікт 2026-10-03 11-30, Pixel_8).md"
        );
    }
}
