use std::sync::Arc;

use crate::error::{Error, Result};
use crate::note::Note;
use crate::paths;
use crate::storage::{StorageEntryKind, StorageProvider};
use crate::vault::{EntryKind, TreeEntry, VaultConfig, SERVICE_DIR};

/// Provider-backed vault operations that do not depend on desktop path semantics.
///
/// The desktop `Vault` can delegate to this layer while keeping local-only cache,
/// rename and trash behavior separate. Mobile SAF/bookmark adapters can implement
/// `StorageProvider` and reuse this logic unchanged.
#[derive(Clone)]
pub struct VaultStorage {
    provider: Arc<dyn StorageProvider>,
    config: VaultConfig,
}

impl VaultStorage {
    /// Opens provider-backed vault content without mutating it.
    pub fn open(provider: Arc<dyn StorageProvider>) -> Result<Self> {
        let config_path = format!("{SERVICE_DIR}/config.json");
        let config = if provider.metadata(&config_path)?.is_some() {
            let raw = provider.read(&config_path)?;
            serde_json::from_slice(&raw).map_err(|e| Error::Config(e.to_string()))?
        } else {
            VaultConfig::default()
        };
        Ok(Self { provider, config })
    }

    pub fn config(&self) -> &VaultConfig {
        &self.config
    }

    pub fn provider(&self) -> Arc<dyn StorageProvider> {
        Arc::clone(&self.provider)
    }

    /// Full file tree. Hidden entries are intentionally excluded just like the
    /// desktop vault tree, including `.mdnotes` and `.git`.
    pub fn tree(&self) -> Result<Vec<TreeEntry>> {
        self.read_dir("")
    }

    fn read_dir(&self, rel: &str) -> Result<Vec<TreeEntry>> {
        let mut entries = Vec::new();
        for item in self.provider.list(rel)? {
            let name = item
                .path
                .rsplit('/')
                .next()
                .unwrap_or(&item.path)
                .to_string();
            if name.starts_with('.') || name.ends_with(".mdnotes-tmp") {
                continue;
            }

            match item.kind {
                StorageEntryKind::Dir => entries.push(TreeEntry {
                    name,
                    path: item.path.clone(),
                    kind: EntryKind::Dir,
                    children: Some(self.read_dir(&item.path)?),
                }),
                StorageEntryKind::File => entries.push(TreeEntry {
                    kind: if paths::is_note(&name) {
                        EntryKind::Note
                    } else {
                        EntryKind::File
                    },
                    name,
                    path: item.path,
                    children: None,
                }),
            }
        }

        entries.sort_by(|a, b| {
            (a.kind != EntryKind::Dir)
                .cmp(&(b.kind != EntryKind::Dir))
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });
        Ok(entries)
    }

    pub fn note_paths(&self) -> Result<Vec<String>> {
        fn collect(entries: &[TreeEntry], out: &mut Vec<String>) {
            for entry in entries {
                match entry.kind {
                    EntryKind::Note => out.push(entry.path.clone()),
                    EntryKind::Dir => collect(entry.children.as_deref().unwrap_or_default(), out),
                    EntryKind::File => {}
                }
            }
        }

        let mut out = Vec::new();
        collect(&self.tree()?, &mut out);
        Ok(out)
    }

    pub fn read_note(&self, rel: &str) -> Result<Note> {
        let rel = note_path(rel)?;
        let raw = self.provider.read(&rel)?;
        let content = String::from_utf8(raw)
            .map_err(|_| Error::Config(format!("note is not valid UTF-8: {rel}")))?;
        Ok(Note::parse(&rel, content))
    }

    pub fn write_note(&self, rel: &str, content: &str) -> Result<Note> {
        let rel = note_path(rel)?;
        self.provider.write(&rel, content.as_bytes())?;
        Ok(Note::parse(&rel, content.to_string()))
    }

    pub fn unique_path(&self, dir: &str, base: &str, ext: &str) -> Result<String> {
        let dir = paths::normalize(dir)?;
        let mut n = 0;
        loop {
            let name = if n == 0 {
                format!("{base}{ext}")
            } else {
                format!("{base} {n}{ext}")
            };
            let rel = paths::join(&dir, &name);
            if self.provider.metadata(&rel)?.is_none() {
                return Ok(rel);
            }
            n += 1;
        }
    }
}

fn note_path(rel: &str) -> Result<String> {
    let norm = paths::normalize(rel)?;
    if !paths::is_note(&norm) {
        return Err(Error::InvalidPath(rel.to_string()));
    }
    Ok(norm)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::LocalFsProvider;

    fn storage() -> (tempfile::TempDir, VaultStorage) {
        let dir = tempfile::tempdir().unwrap();
        let provider = Arc::new(LocalFsProvider::new(dir.path()).unwrap());
        let storage = VaultStorage::open(provider).unwrap();
        (dir, storage)
    }

    #[test]
    fn provider_tree_and_note_round_trip_match_vault_semantics() {
        let (dir, storage) = storage();
        std::fs::create_dir_all(dir.path().join("Projects")).unwrap();
        std::fs::create_dir_all(dir.path().join(".git")).unwrap();
        storage.write_note("Projects/A.md", "# A").unwrap();
        std::fs::write(dir.path().join("image.png"), [0u8]).unwrap();

        let tree = storage.tree().unwrap();
        let names: Vec<_> = tree.iter().map(|entry| entry.name.as_str()).collect();
        assert_eq!(names, vec!["Projects", "image.png"]);
        assert_eq!(storage.note_paths().unwrap(), vec!["Projects/A.md"]);
        assert_eq!(storage.read_note("Projects/A.md").unwrap().title, "A");
    }

    #[test]
    fn provider_storage_reads_config_and_generates_unique_paths() {
        let dir = tempfile::tempdir().unwrap();
        let provider = Arc::new(LocalFsProvider::new(dir.path()).unwrap());
        provider
            .write(
                ".mdnotes/config.json",
                br#"{"version":1,"name":"Mobile","attachmentsDir":"files","dailyNotesDir":"days"}"#,
            )
            .unwrap();
        provider.write("Idea.md", b"x").unwrap();

        let storage = VaultStorage::open(provider).unwrap();
        assert_eq!(storage.config().name.as_deref(), Some("Mobile"));
        assert_eq!(storage.unique_path("", "Idea", ".md").unwrap(), "Idea 1.md");
    }
}
