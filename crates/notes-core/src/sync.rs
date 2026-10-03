use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

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
