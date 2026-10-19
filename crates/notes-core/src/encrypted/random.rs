//! Random-name layout of an encrypted vault (ADR-0007).
//!
//! Every file is stored as `<16 random characters>.age` directly in the vault
//! root (files of the trash in `.mdnotes/trash/`), so neither note titles nor
//! folder names are visible in the file system or in Git. The logical path
//! (`folder/note.md`) lives inside the ciphertext as the first line of an
//! envelope:
//!
//! ```text
//! MDN1\n<logical path>\n<file content>
//! ```
//!
//! The provider keeps an in-memory map from logical paths to physical files.
//! It is rebuilt by reading the envelope header of every file that is new or
//! changed (size/modification time), so changes made by another device (Git
//! pull, a synced folder) are picked up. Empty folders are stored as a marker
//! file named [`DIR_MARKER`] inside the folder.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::io::Read;
use std::sync::MutexGuard;

use super::{EncryptedProvider, ENCRYPTED_SUFFIX};
use crate::error::{Error, Result};
use crate::paths;
use crate::storage::{StorageEntry, StorageEntryKind};
use crate::vault::SERVICE_DIR;

const MAGIC: &[u8] = b"MDN1\n";
const MAX_PATH_BYTES: usize = 1024;
/// Hidden file that keeps an otherwise empty folder alive.
pub(super) const DIR_MARKER: &str = ".mdnotes-dir";

struct Known {
    size: u64,
    modified_ms: Option<u64>,
    logical: String,
}

/// Map between logical paths and physical files.
#[derive(Default)]
pub(super) struct NameIndex {
    /// Physical path -> what was read from its header.
    known: HashMap<String, Known>,
    /// Logical path -> physical path.
    by_logical: HashMap<String, String>,
}

fn crypto(err: impl std::fmt::Display) -> Error {
    Error::Crypto(err.to_string())
}

fn trash_dir() -> String {
    format!("{SERVICE_DIR}/trash")
}

fn random_name() -> String {
    // The last 16 characters of a ULID are its random part (80 bits); the
    // first 10 encode the time and must not leak into file names.
    let id = ulid::Ulid::new().to_string().to_lowercase();
    id[10..].to_string()
}

fn seal(logical: &str, content: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(MAGIC.len() + logical.len() + 1 + content.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(logical.as_bytes());
    out.push(b'\n');
    out.extend_from_slice(content);
    out
}

/// Position right after the envelope header (magic and path line).
fn header_end(plain: &[u8]) -> Result<usize> {
    if !plain.starts_with(MAGIC) {
        return Err(Error::Crypto("not an MD Notes envelope".into()));
    }
    let rest = &plain[MAGIC.len()..];
    let newline = rest
        .iter()
        .position(|byte| *byte == b'\n')
        .ok_or_else(|| Error::Crypto("envelope header is truncated".into()))?;
    Ok(MAGIC.len() + newline + 1)
}

fn header_path(plain: &[u8]) -> Result<String> {
    let end = header_end(plain)?;
    let path = std::str::from_utf8(&plain[MAGIC.len()..end - 1])
        .map_err(|_| Error::Crypto("envelope path is not valid UTF-8".into()))?;
    let path = paths::normalize(path)?;
    if path.is_empty() {
        return Err(Error::Crypto("envelope path is empty".into()));
    }
    Ok(path)
}

fn open_envelope(plain: Vec<u8>) -> Result<Vec<u8>> {
    let end = header_end(&plain)?;
    Ok(plain[end..].to_vec())
}

impl EncryptedProvider {
    fn lock_index(&self) -> Result<MutexGuard<'_, NameIndex>> {
        self.index
            .lock()
            .map_err(|_| Error::Crypto("name index is unavailable".into()))
    }

    /// Reads the envelope path of a file without decrypting all of it.
    fn envelope_path(&self, data: &[u8]) -> Result<String> {
        if self.identities.is_empty() {
            return Err(Error::Crypto("no identity available to decrypt".into()));
        }
        let decryptor = age::Decryptor::new(data).map_err(crypto)?;
        let identities = self
            .identities
            .iter()
            .map(|identity| identity as &dyn age::Identity);
        let reader = decryptor.decrypt(identities).map_err(crypto)?;
        let mut head = Vec::new();
        reader
            .take((MAGIC.len() + MAX_PATH_BYTES + 1) as u64)
            .read_to_end(&mut head)
            .map_err(crypto)?;
        header_path(&head)
    }

    /// Brings the map up to date with the files on disk.
    fn sync_names(&self, idx: &mut NameIndex) -> Result<()> {
        let mut seen: HashSet<String> = HashSet::new();
        let trash = trash_dir();
        for dir in ["", trash.as_str()] {
            let entries = match self.inner.list(dir) {
                Ok(entries) => entries,
                Err(Error::NotFound(_)) => continue,
                Err(err) => return Err(err),
            };
            for entry in entries {
                if entry.kind != StorageEntryKind::File || !entry.path.ends_with(ENCRYPTED_SUFFIX) {
                    continue;
                }
                seen.insert(entry.path.clone());
                let fresh = idx
                    .known
                    .get(&entry.path)
                    .is_some_and(|k| k.size == entry.size && k.modified_ms == entry.modified_ms);
                if fresh {
                    continue;
                }
                let data = self.inner.read(&entry.path)?;
                match self.envelope_path(&data) {
                    Ok(logical) => {
                        idx.known.insert(
                            entry.path.clone(),
                            Known {
                                size: entry.size,
                                modified_ms: entry.modified_ms,
                                logical,
                            },
                        );
                    }
                    // Without any identity nothing can be mapped: report it.
                    Err(err) if self.identities.is_empty() => return Err(err),
                    // A file that is not ours or is damaged stays invisible.
                    Err(_) => {
                        idx.known.remove(&entry.path);
                    }
                }
            }
        }
        idx.known.retain(|phys, _| seen.contains(phys));
        idx.by_logical = idx
            .known
            .iter()
            .map(|(phys, known)| (known.logical.clone(), phys.clone()))
            .collect();
        Ok(())
    }

    /// Physical path of a logical file; re-reads the disk once on a miss.
    fn find_file(&self, idx: &mut NameIndex, logical: &str) -> Result<Option<String>> {
        if let Some(phys) = idx.by_logical.get(logical) {
            return Ok(Some(phys.clone()));
        }
        self.sync_names(idx)?;
        Ok(idx.by_logical.get(logical).cloned())
    }

    fn new_physical(&self, idx: &NameIndex, logical: &str) -> Result<String> {
        let dir = if Self::is_trash(logical) {
            trash_dir()
        } else {
            String::new()
        };
        loop {
            let name = format!("{}{ENCRYPTED_SUFFIX}", random_name());
            let phys = if dir.is_empty() {
                name
            } else {
                format!("{dir}/{name}")
            };
            if !idx.known.contains_key(&phys) && self.inner.metadata(&phys)?.is_none() {
                return Ok(phys);
            }
        }
    }

    /// Remembers a freshly written file.
    fn record(&self, idx: &mut NameIndex, logical: &str, phys: &str) -> Result<()> {
        let (size, modified_ms) = match self.inner.metadata(phys)? {
            Some(entry) => (entry.size, entry.modified_ms),
            None => (0, None),
        };
        idx.known.insert(
            phys.to_string(),
            Known {
                size,
                modified_ms,
                logical: logical.to_string(),
            },
        );
        idx.by_logical.insert(logical.to_string(), phys.to_string());
        Ok(())
    }

    fn forget(idx: &mut NameIndex, logical: &str, phys: &str) {
        idx.known.remove(phys);
        idx.by_logical.remove(logical);
    }

    fn file_entry(idx: &NameIndex, logical: &str, phys: &str) -> StorageEntry {
        let known = idx.known.get(phys);
        StorageEntry {
            path: logical.to_string(),
            kind: StorageEntryKind::File,
            size: known.map_or(0, |k| k.size),
            modified_ms: known.and_then(|k| k.modified_ms),
        }
    }

    fn dir_entry(path: &str) -> StorageEntry {
        StorageEntry {
            path: path.to_string(),
            kind: StorageEntryKind::Dir,
            size: 0,
            modified_ms: None,
        }
    }

    fn dir_exists(idx: &NameIndex, rel: &str) -> bool {
        let prefix = format!("{rel}/");
        idx.by_logical.keys().any(|l| l.starts_with(&prefix))
    }

    pub(super) fn r_list(&self, rel: &str) -> Result<Vec<StorageEntry>> {
        let mut idx = self.lock_index()?;
        self.sync_names(&mut idx)?;
        let prefix = if rel.is_empty() {
            String::new()
        } else {
            format!("{rel}/")
        };
        let mut files: BTreeMap<String, StorageEntry> = BTreeMap::new();
        let mut dirs: BTreeSet<String> = BTreeSet::new();
        let mut exists = rel.is_empty();
        for (logical, phys) in &idx.by_logical {
            let Some(rest) = logical.strip_prefix(&prefix) else {
                continue;
            };
            exists = true;
            match rest.split_once('/') {
                None => {
                    if rest != DIR_MARKER {
                        files.insert(logical.clone(), Self::file_entry(&idx, logical, phys));
                    }
                }
                Some((dir, _)) => {
                    dirs.insert(format!("{prefix}{dir}"));
                }
            }
        }
        if !exists {
            return Err(Error::NotFound(rel.to_string()));
        }
        let mut out: Vec<StorageEntry> = Vec::new();
        if rel.is_empty() {
            // Folders that are really on disk (`.mdnotes`, `.git`).
            for entry in self.inner.list("")? {
                if entry.kind == StorageEntryKind::Dir && !dirs.contains(&entry.path) {
                    out.push(entry);
                }
            }
        }
        out.extend(dirs.iter().map(|dir| Self::dir_entry(dir)));
        out.extend(files.into_values());
        Ok(out)
    }

    pub(super) fn r_read(&self, rel: &str) -> Result<Vec<u8>> {
        let mut idx = self.lock_index()?;
        let phys = self
            .find_file(&mut idx, rel)?
            .ok_or_else(|| Error::NotFound(rel.to_string()))?;
        let plain = self.decrypt(&self.inner.read(&phys)?)?;
        open_envelope(plain)
    }

    fn write_locked(&self, idx: &mut NameIndex, rel: &str, bytes: &[u8]) -> Result<()> {
        let phys = match self.find_file(idx, rel)? {
            Some(phys) => phys,
            None => self.new_physical(idx, rel)?,
        };
        let data = self.encrypt(&seal(rel, bytes))?;
        self.inner.write(&phys, &data)?;
        self.record(idx, rel, &phys)
    }

    pub(super) fn r_write(&self, rel: &str, bytes: &[u8]) -> Result<()> {
        let mut idx = self.lock_index()?;
        self.write_locked(&mut idx, rel, bytes)
    }

    pub(super) fn r_remove(&self, rel: &str) -> Result<()> {
        let mut idx = self.lock_index()?;
        if let Some(phys) = self.find_file(&mut idx, rel)? {
            self.inner.remove(&phys)?;
            Self::forget(&mut idx, rel, &phys);
            return Ok(());
        }
        let prefix = format!("{rel}/");
        let victims: Vec<(String, String)> = idx
            .by_logical
            .iter()
            .filter(|(logical, _)| logical.starts_with(&prefix))
            .map(|(logical, phys)| (logical.clone(), phys.clone()))
            .collect();
        if victims.is_empty() {
            return Err(Error::NotFound(rel.to_string()));
        }
        for (logical, phys) in victims {
            self.inner.remove(&phys)?;
            Self::forget(&mut idx, &logical, &phys);
        }
        Ok(())
    }

    fn move_file(&self, idx: &mut NameIndex, phys: &str, from: &str, to: &str) -> Result<()> {
        let content = open_envelope(self.decrypt(&self.inner.read(phys)?)?)?;
        let data = self.encrypt(&seal(to, &content))?;
        let target = if Self::is_trash(from) == Self::is_trash(to) {
            phys.to_string()
        } else {
            self.new_physical(idx, to)?
        };
        self.inner.write(&target, &data)?;
        if target != phys {
            self.inner.remove(phys)?;
        }
        Self::forget(idx, from, phys);
        self.record(idx, to, &target)
    }

    pub(super) fn r_rename(&self, from: &str, to: &str) -> Result<()> {
        let mut idx = self.lock_index()?;
        if let Some(phys) = self.find_file(&mut idx, from)? {
            if self.find_file(&mut idx, to)?.is_some() {
                return Err(Error::AlreadyExists(to.to_string()));
            }
            return self.move_file(&mut idx, &phys, from, to);
        }
        let prefix = format!("{from}/");
        let moves: Vec<(String, String, String)> = idx
            .by_logical
            .iter()
            .filter_map(|(logical, phys)| {
                logical
                    .strip_prefix(&prefix)
                    .map(|rest| (logical.clone(), phys.clone(), format!("{to}/{rest}")))
            })
            .collect();
        if moves.is_empty() {
            return Err(Error::NotFound(from.to_string()));
        }
        if let Some((_, _, target)) = moves
            .iter()
            .find(|(_, _, target)| idx.by_logical.contains_key(target))
        {
            return Err(Error::AlreadyExists(target.clone()));
        }
        for (logical, phys, target) in moves {
            self.move_file(&mut idx, &phys, &logical, &target)?;
        }
        Ok(())
    }

    pub(super) fn r_metadata(&self, rel: &str) -> Result<Option<StorageEntry>> {
        let mut idx = self.lock_index()?;
        if rel.is_empty() {
            return Ok(Some(Self::dir_entry("")));
        }
        if let Some(phys) = self.find_file(&mut idx, rel)? {
            return Ok(Some(Self::file_entry(&idx, rel, &phys)));
        }
        if Self::dir_exists(&idx, rel) {
            return Ok(Some(Self::dir_entry(rel)));
        }
        // Folders that are really on disk (`.git`).
        Ok(self
            .inner
            .metadata(rel)?
            .filter(|entry| entry.kind == StorageEntryKind::Dir))
    }

    pub(super) fn r_changes_since(&self, since_ms: u64) -> Result<Vec<StorageEntry>> {
        let mut idx = self.lock_index()?;
        self.sync_names(&mut idx)?;
        let mut out: Vec<StorageEntry> = idx
            .known
            .iter()
            .filter(|(_, known)| known.modified_ms.is_some_and(|ms| ms > since_ms))
            .filter(|(_, known)| !known.logical.ends_with(DIR_MARKER))
            .map(|(phys, known)| Self::file_entry(&idx, &known.logical, phys))
            .collect();
        out.extend(
            self.inner
                .changes_since(since_ms)?
                .into_iter()
                .filter(|entry| Self::is_plain(&entry.path)),
        );
        Ok(out)
    }

    pub(super) fn r_create_dir(&self, rel: &str) -> Result<()> {
        let mut idx = self.lock_index()?;
        let marker = format!("{rel}/{DIR_MARKER}");
        if self.find_file(&mut idx, &marker)?.is_some() || Self::dir_exists(&idx, rel) {
            return Ok(());
        }
        self.write_locked(&mut idx, &marker, b"")
    }
}
