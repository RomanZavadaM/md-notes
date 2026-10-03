use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use crate::error::{io_err, Error, Result};
use crate::paths;

const TMP_SUFFIX: &str = ".mdnotes-tmp";

/// Kind of an entry exposed by a storage provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageEntryKind {
    Dir,
    File,
}

/// Provider-neutral metadata for one vault entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageEntry {
    pub path: String,
    pub kind: StorageEntryKind,
    pub size: u64,
    /// Unix timestamp in milliseconds when the provider can expose it.
    pub modified_ms: Option<u64>,
}

/// Minimal storage contract used by the core.
///
/// Paths are always relative to the provider root. Implementations must reject
/// attempts to escape that root. Remote/mobile providers may keep their own
/// native handles internally; callers never depend on platform paths.
pub trait StorageProvider: Send + Sync {
    fn list(&self, rel: &str) -> Result<Vec<StorageEntry>>;
    fn read(&self, rel: &str) -> Result<Vec<u8>>;
    fn write(&self, rel: &str, bytes: &[u8]) -> Result<()>;
    fn remove(&self, rel: &str) -> Result<()>;
    fn metadata(&self, rel: &str) -> Result<Option<StorageEntry>>;

    /// Returns entries modified strictly after `since_ms`.
    ///
    /// Providers with a native change journal can override this efficiently.
    /// The local provider uses a recursive metadata scan as the portable
    /// baseline; the SQLite index remains rebuildable local state.
    fn changes_since(&self, since_ms: u64) -> Result<Vec<StorageEntry>>;
}

/// Local-folder provider used by desktop vaults and by the mobile sandbox.
#[derive(Debug, Clone)]
pub struct LocalFsProvider {
    root: PathBuf,
}

impl LocalFsProvider {
    pub fn new(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        if !root.is_dir() {
            return Err(Error::NotFound(root.display().to_string()));
        }
        Ok(Self { root })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    fn entry(&self, rel: &str, path: &Path) -> Result<StorageEntry> {
        let metadata = fs::metadata(path).map_err(io_err(path))?;
        Ok(StorageEntry {
            path: paths::normalize(rel)?,
            kind: if metadata.is_dir() {
                StorageEntryKind::Dir
            } else {
                StorageEntryKind::File
            },
            size: if metadata.is_file() {
                metadata.len()
            } else {
                0
            },
            modified_ms: modified_ms(&metadata),
        })
    }

    fn collect_changes(&self, rel: &str, since_ms: u64, out: &mut Vec<StorageEntry>) -> Result<()> {
        for entry in self.list(rel)? {
            if entry.modified_ms.is_some_and(|value| value > since_ms) {
                out.push(entry.clone());
            }
            if entry.kind == StorageEntryKind::Dir {
                self.collect_changes(&entry.path, since_ms, out)?;
            }
        }
        Ok(())
    }
}

impl StorageProvider for LocalFsProvider {
    fn list(&self, rel: &str) -> Result<Vec<StorageEntry>> {
        let rel = paths::normalize(rel)?;
        let dir = paths::resolve(&self.root, &rel)?;
        if !dir.is_dir() {
            return Err(Error::NotFound(rel));
        }

        let mut out = Vec::new();
        for item in fs::read_dir(&dir).map_err(io_err(&dir))? {
            let item = item.map_err(io_err(&dir))?;
            let name = item.file_name().to_string_lossy().into_owned();
            if name.ends_with(TMP_SUFFIX) {
                continue;
            }
            let child_rel = paths::join(&rel, &name);
            out.push(self.entry(&child_rel, &item.path())?);
        }
        out.sort_by_key(|entry| entry.path.to_lowercase());
        Ok(out)
    }

    fn read(&self, rel: &str) -> Result<Vec<u8>> {
        let rel = paths::normalize(rel)?;
        if rel.is_empty() {
            return Err(Error::InvalidPath("vault root".into()));
        }
        let path = paths::resolve(&self.root, &rel)?;
        if !path.is_file() {
            return Err(Error::NotFound(rel));
        }
        fs::read(&path).map_err(io_err(&path))
    }

    fn write(&self, rel: &str, bytes: &[u8]) -> Result<()> {
        let rel = paths::normalize(rel)?;
        if rel.is_empty() {
            return Err(Error::InvalidPath("vault root".into()));
        }
        let path = paths::resolve(&self.root, &rel)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(io_err(parent))?;
        }

        let mut tmp = path.as_os_str().to_owned();
        tmp.push(TMP_SUFFIX);
        let tmp = PathBuf::from(tmp);
        fs::write(&tmp, bytes).map_err(io_err(&tmp))?;
        fs::rename(&tmp, &path).map_err(io_err(&path))
    }

    fn remove(&self, rel: &str) -> Result<()> {
        let rel = paths::normalize(rel)?;
        if rel.is_empty() {
            return Err(Error::InvalidPath("vault root".into()));
        }
        let path = paths::resolve(&self.root, &rel)?;
        if path.is_dir() {
            fs::remove_dir_all(&path).map_err(io_err(&path))
        } else if path.is_file() {
            fs::remove_file(&path).map_err(io_err(&path))
        } else {
            Err(Error::NotFound(rel))
        }
    }

    fn metadata(&self, rel: &str) -> Result<Option<StorageEntry>> {
        let rel = paths::normalize(rel)?;
        if rel.is_empty() {
            return Ok(None);
        }
        let path = paths::resolve(&self.root, &rel)?;
        if !path.exists() {
            return Ok(None);
        }
        self.entry(&rel, &path).map(Some)
    }

    fn changes_since(&self, since_ms: u64) -> Result<Vec<StorageEntry>> {
        let mut out = Vec::new();
        self.collect_changes("", since_ms, &mut out)?;
        out.sort_by_key(|entry| entry.path.to_lowercase());
        Ok(out)
    }
}

fn modified_ms(metadata: &fs::Metadata) -> Option<u64> {
    let modified: SystemTime = metadata.modified().ok()?;
    let millis = modified.duration_since(UNIX_EPOCH).ok()?.as_millis();
    u64::try_from(millis).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_provider_round_trips_and_rejects_escape() {
        let dir = tempfile::tempdir().unwrap();
        let provider = LocalFsProvider::new(dir.path()).unwrap();

        provider.write("notes/Привіт.md", b"# Hello").unwrap();
        assert_eq!(provider.read("notes/Привіт.md").unwrap(), b"# Hello");

        let entries = provider.list("notes").unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, "notes/Привіт.md");
        assert_eq!(entries[0].kind, StorageEntryKind::File);

        assert!(provider.read("../outside.md").is_err());
        assert!(provider.write("../outside.md", b"x").is_err());
    }

    #[test]
    fn local_provider_removes_files_and_directories() {
        let dir = tempfile::tempdir().unwrap();
        let provider = LocalFsProvider::new(dir.path()).unwrap();
        provider.write("a/b.md", b"x").unwrap();

        assert!(provider.metadata("a/b.md").unwrap().is_some());
        provider.remove("a").unwrap();
        assert!(provider.metadata("a/b.md").unwrap().is_none());
    }

    #[test]
    fn local_provider_reports_recent_changes() {
        let dir = tempfile::tempdir().unwrap();
        let provider = LocalFsProvider::new(dir.path()).unwrap();
        provider.write("a.md", b"x").unwrap();

        let changes = provider.changes_since(0).unwrap();
        assert!(changes.iter().any(|entry| entry.path == "a.md"));
    }
}
