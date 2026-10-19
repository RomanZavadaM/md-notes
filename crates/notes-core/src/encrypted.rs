//! Optional encrypted vault (ADR-0007).
//!
//! `EncryptedProvider` decorates any [`StorageProvider`]: a logical note path
//! such as `a/b.md` is stored as `a/b.md.age` (age encryption, X25519
//! recipients). Decryption happens in memory on read, encryption in memory
//! before the inner provider writes, so decrypted note or attachment bytes are
//! never handed to the underlying storage.
//!
//! The service folder (`.mdnotes/`) is passed through unencrypted: it only
//! holds settings, the schema and the list of public recipient keys. The one
//! exception is `.mdnotes/trash/`, which keeps encrypted files. Plain files
//! that are not `*.age` are hidden from listings of an encrypted vault.
//! Private keys never live in the vault; the caller supplies identities that
//! were loaded from the OS secret store.

use std::io::{Read, Write};
use std::sync::Arc;

use crate::error::{Error, Result};
use crate::paths;
use crate::storage::{StorageEntry, StorageEntryKind, StorageProvider};
use crate::vault::SERVICE_DIR;

/// Suffix of encrypted files in the underlying storage.
pub const ENCRYPTED_SUFFIX: &str = ".age";

/// Storage provider that keeps every non-service file encrypted with age.
pub struct EncryptedProvider {
    inner: Arc<dyn StorageProvider>,
    recipients: Vec<age::x25519::Recipient>,
    identities: Vec<age::x25519::Identity>,
}

impl EncryptedProvider {
    /// Creates a provider that encrypts for every recipient and decrypts with
    /// any of the identities. At least one recipient is required; a provider
    /// without identities can write but cannot read.
    pub fn new(
        inner: Arc<dyn StorageProvider>,
        recipients: Vec<age::x25519::Recipient>,
        identities: Vec<age::x25519::Identity>,
    ) -> Result<Self> {
        if recipients.is_empty() {
            return Err(Error::Crypto("at least one recipient is required".into()));
        }
        Ok(Self {
            inner,
            recipients,
            identities,
        })
    }

    /// Convenience constructor: the identity's own public key is always a
    /// recipient, `extra_recipients` (for example a recovery key) are added.
    pub fn from_identity(
        inner: Arc<dyn StorageProvider>,
        identity: age::x25519::Identity,
        extra_recipients: Vec<age::x25519::Recipient>,
    ) -> Result<Self> {
        let mut recipients = vec![identity.to_public()];
        recipients.extend(extra_recipients);
        Self::new(inner, recipients, vec![identity])
    }

    fn is_service(rel: &str) -> bool {
        rel == SERVICE_DIR || rel.starts_with(&format!("{SERVICE_DIR}/"))
    }

    fn is_trash(rel: &str) -> bool {
        let trash = format!("{SERVICE_DIR}/trash");
        rel == trash || rel.starts_with(&format!("{trash}/"))
    }

    /// Service files that stay unencrypted. The trash is the exception: it
    /// holds moved notes and attachments, so its files keep the `.age` suffix.
    fn is_plain(rel: &str) -> bool {
        Self::is_service(rel) && !Self::is_trash(rel)
    }

    fn physical(rel: &str) -> String {
        format!("{rel}{ENCRYPTED_SUFFIX}")
    }

    fn encrypt(&self, plain: &[u8]) -> Result<Vec<u8>> {
        let recipients: Vec<&dyn age::Recipient> = self
            .recipients
            .iter()
            .map(|recipient| recipient as &dyn age::Recipient)
            .collect();
        let encryptor = age::Encryptor::with_recipients(recipients.into_iter())
            .map_err(|err| Error::Crypto(err.to_string()))?;
        let mut out = Vec::new();
        let mut writer = encryptor
            .wrap_output(&mut out)
            .map_err(|err| Error::Crypto(err.to_string()))?;
        writer
            .write_all(plain)
            .map_err(|err| Error::Crypto(err.to_string()))?;
        writer
            .finish()
            .map_err(|err| Error::Crypto(err.to_string()))?;
        Ok(out)
    }

    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        if self.identities.is_empty() {
            return Err(Error::Crypto("no identity available to decrypt".into()));
        }
        let decryptor = age::Decryptor::new(data).map_err(|err| Error::Crypto(err.to_string()))?;
        let identities = self
            .identities
            .iter()
            .map(|identity| identity as &dyn age::Identity);
        let mut reader = decryptor
            .decrypt(identities)
            .map_err(|err| Error::Crypto(err.to_string()))?;
        let mut out = Vec::new();
        reader
            .read_to_end(&mut out)
            .map_err(|err| Error::Crypto(err.to_string()))?;
        Ok(out)
    }

    /// Maps an entry of the inner storage to its logical form; `None` hides it.
    fn to_logical(entry: StorageEntry) -> Option<StorageEntry> {
        if entry.kind == StorageEntryKind::Dir || Self::is_plain(&entry.path) {
            return Some(entry);
        }
        let logical = entry.path.strip_suffix(ENCRYPTED_SUFFIX)?.to_string();
        Some(StorageEntry {
            path: logical,
            ..entry
        })
    }

    fn is_inner_dir(&self, rel: &str) -> Result<bool> {
        Ok(self
            .inner
            .metadata(rel)?
            .is_some_and(|entry| entry.kind == StorageEntryKind::Dir))
    }
}

impl StorageProvider for EncryptedProvider {
    fn list(&self, rel: &str) -> Result<Vec<StorageEntry>> {
        let rel = paths::normalize(rel)?;
        let entries = self.inner.list(&rel)?;
        if Self::is_plain(&rel) {
            return Ok(entries);
        }
        Ok(entries.into_iter().filter_map(Self::to_logical).collect())
    }

    fn read(&self, rel: &str) -> Result<Vec<u8>> {
        let rel = paths::normalize(rel)?;
        if Self::is_plain(&rel) {
            return self.inner.read(&rel);
        }
        let data = self.inner.read(&Self::physical(&rel))?;
        self.decrypt(&data)
    }

    fn write(&self, rel: &str, bytes: &[u8]) -> Result<()> {
        let rel = paths::normalize(rel)?;
        if Self::is_plain(&rel) {
            return self.inner.write(&rel, bytes);
        }
        let encrypted = self.encrypt(bytes)?;
        self.inner.write(&Self::physical(&rel), &encrypted)
    }

    fn remove(&self, rel: &str) -> Result<()> {
        let rel = paths::normalize(rel)?;
        if Self::is_plain(&rel) || self.is_inner_dir(&rel)? {
            return self.inner.remove(&rel);
        }
        self.inner.remove(&Self::physical(&rel))
    }

    fn rename(&self, from: &str, to: &str) -> Result<()> {
        let from = paths::normalize(from)?;
        let to = paths::normalize(to)?;
        if Self::is_plain(&from) || Self::is_plain(&to) || self.is_inner_dir(&from)? {
            return self.inner.rename(&from, &to);
        }
        self.inner
            .rename(&Self::physical(&from), &Self::physical(&to))
    }

    fn metadata(&self, rel: &str) -> Result<Option<StorageEntry>> {
        let rel = paths::normalize(rel)?;
        if Self::is_plain(&rel) {
            return self.inner.metadata(&rel);
        }
        if let Some(entry) = self.inner.metadata(&rel)? {
            if entry.kind == StorageEntryKind::Dir {
                return Ok(Some(entry));
            }
        }
        Ok(self
            .inner
            .metadata(&Self::physical(&rel))?
            .map(|entry| StorageEntry {
                path: rel.clone(),
                ..entry
            }))
    }

    fn changes_since(&self, since_ms: u64) -> Result<Vec<StorageEntry>> {
        Ok(self
            .inner
            .changes_since(since_ms)?
            .into_iter()
            .filter_map(Self::to_logical)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::LocalFsProvider;
    use crate::vault::Vault;
    use std::path::Path;
    use std::sync::Mutex;

    fn local(dir: &Path) -> Arc<LocalFsProvider> {
        Arc::new(LocalFsProvider::new(dir).unwrap())
    }

    fn provider(dir: &Path, identity: &age::x25519::Identity) -> EncryptedProvider {
        EncryptedProvider::from_identity(local(dir), identity.clone(), vec![]).unwrap()
    }

    /// Records every byte string the inner provider is asked to write.
    struct Spy {
        inner: LocalFsProvider,
        writes: Mutex<Vec<Vec<u8>>>,
    }

    impl StorageProvider for Spy {
        fn list(&self, rel: &str) -> Result<Vec<StorageEntry>> {
            self.inner.list(rel)
        }
        fn read(&self, rel: &str) -> Result<Vec<u8>> {
            self.inner.read(rel)
        }
        fn write(&self, rel: &str, bytes: &[u8]) -> Result<()> {
            self.writes.lock().unwrap().push(bytes.to_vec());
            self.inner.write(rel, bytes)
        }
        fn remove(&self, rel: &str) -> Result<()> {
            self.inner.remove(rel)
        }
        fn rename(&self, from: &str, to: &str) -> Result<()> {
            self.inner.rename(from, to)
        }
        fn metadata(&self, rel: &str) -> Result<Option<StorageEntry>> {
            self.inner.metadata(rel)
        }
        fn changes_since(&self, since_ms: u64) -> Result<Vec<StorageEntry>> {
            self.inner.changes_since(since_ms)
        }
    }

    #[test]
    fn round_trip_stores_ciphertext_under_age_suffix() {
        let dir = tempfile::tempdir().unwrap();
        let identity = age::x25519::Identity::generate();
        let enc = provider(dir.path(), &identity);

        enc.write("a/b.md", b"# Secret title\nbody").unwrap();
        assert_eq!(enc.read("a/b.md").unwrap(), b"# Secret title\nbody");

        assert!(!dir.path().join("a/b.md").exists());
        let raw = std::fs::read(dir.path().join("a/b.md.age")).unwrap();
        assert!(raw.starts_with(b"age-encryption.org/v1\n"));
        assert!(!raw.windows(6).any(|w| w == b"Secret"));
    }

    #[test]
    fn requires_a_recipient() {
        let dir = tempfile::tempdir().unwrap();
        assert!(EncryptedProvider::new(local(dir.path()), vec![], vec![]).is_err());
    }

    #[test]
    fn second_recipient_can_decrypt() {
        let dir = tempfile::tempdir().unwrap();
        let work = age::x25519::Identity::generate();
        let recovery = age::x25519::Identity::generate();
        let writer =
            EncryptedProvider::from_identity(local(dir.path()), work, vec![recovery.to_public()])
                .unwrap();
        writer.write("n.md", b"hello").unwrap();

        let reader = provider(dir.path(), &recovery);
        assert_eq!(reader.read("n.md").unwrap(), b"hello");
    }

    #[test]
    fn wrong_identity_and_missing_identity_fail() {
        let dir = tempfile::tempdir().unwrap();
        let identity = age::x25519::Identity::generate();
        provider(dir.path(), &identity)
            .write("n.md", b"hello")
            .unwrap();

        let stranger = age::x25519::Identity::generate();
        assert!(matches!(
            provider(dir.path(), &stranger).read("n.md"),
            Err(Error::Crypto(_))
        ));

        let write_only =
            EncryptedProvider::new(local(dir.path()), vec![identity.to_public()], vec![]).unwrap();
        assert!(matches!(write_only.read("n.md"), Err(Error::Crypto(_))));
        write_only.write("m.md", b"ok").unwrap();
    }

    #[test]
    fn tampered_ciphertext_is_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let identity = age::x25519::Identity::generate();
        let enc = provider(dir.path(), &identity);
        enc.write("n.md", b"hello world").unwrap();

        let path = dir.path().join("n.md.age");
        let mut raw = std::fs::read(&path).unwrap();
        let last = raw.len() - 1;
        raw[last] ^= 0x01;
        std::fs::write(&path, raw).unwrap();

        assert!(enc.read("n.md").is_err());
    }

    #[test]
    fn list_hides_suffix_and_plain_files() {
        let dir = tempfile::tempdir().unwrap();
        let identity = age::x25519::Identity::generate();
        let enc = provider(dir.path(), &identity);
        enc.write("n.md", b"x").unwrap();
        enc.write("dir/m.md", b"y").unwrap();
        std::fs::write(dir.path().join("stray.txt"), "plain").unwrap();

        let names: Vec<String> = enc.list("").unwrap().into_iter().map(|e| e.path).collect();
        assert!(names.contains(&"n.md".to_string()));
        assert!(names.contains(&"dir".to_string()));
        assert!(!names
            .iter()
            .any(|n| n.ends_with(".age") || n == "stray.txt"));
        assert_eq!(
            enc.list("dir")
                .unwrap()
                .into_iter()
                .map(|e| e.path)
                .collect::<Vec<_>>(),
            vec!["dir/m.md".to_string()]
        );
    }

    #[test]
    fn service_folder_is_not_encrypted() {
        let dir = tempfile::tempdir().unwrap();
        let identity = age::x25519::Identity::generate();
        let enc = provider(dir.path(), &identity);
        enc.write(".mdnotes/recipients.txt", b"age1public").unwrap();
        assert_eq!(
            std::fs::read(dir.path().join(".mdnotes/recipients.txt")).unwrap(),
            b"age1public"
        );
        assert_eq!(enc.read(".mdnotes/recipients.txt").unwrap(), b"age1public");
    }

    #[test]
    fn metadata_rename_and_remove_use_logical_paths() {
        let dir = tempfile::tempdir().unwrap();
        let identity = age::x25519::Identity::generate();
        let enc = provider(dir.path(), &identity);
        enc.write("one.md", b"1").unwrap();

        let meta = enc.metadata("one.md").unwrap().unwrap();
        assert_eq!(meta.path, "one.md");
        assert_eq!(meta.kind, StorageEntryKind::File);
        assert!(enc.metadata("missing.md").unwrap().is_none());

        enc.rename("one.md", "sub/two.md").unwrap();
        assert!(enc.metadata("one.md").unwrap().is_none());
        assert_eq!(enc.read("sub/two.md").unwrap(), b"1");
        assert!(dir.path().join("sub/two.md.age").is_file());

        enc.rename("sub", "moved").unwrap();
        assert_eq!(enc.read("moved/two.md").unwrap(), b"1");

        enc.remove("moved/two.md").unwrap();
        assert!(enc.metadata("moved/two.md").unwrap().is_none());
        enc.write("keep/x.md", b"z").unwrap();
        enc.remove("keep").unwrap();
        assert!(enc.metadata("keep").unwrap().is_none());
    }

    #[test]
    fn inner_provider_never_receives_plaintext() {
        let dir = tempfile::tempdir().unwrap();
        let identity = age::x25519::Identity::generate();
        let spy = Arc::new(Spy {
            inner: LocalFsProvider::new(dir.path()).unwrap(),
            writes: Mutex::new(Vec::new()),
        });
        let enc = EncryptedProvider::from_identity(spy.clone(), identity.clone(), vec![]).unwrap();

        enc.write("n.md", b"PLAINTEXT-MARKER body").unwrap();
        enc.write("a/img.bin", &[0xAB; 64]).unwrap();

        let writes = spy.writes.lock().unwrap();
        assert_eq!(writes.len(), 2);
        for bytes in writes.iter() {
            assert!(bytes.starts_with(b"age-encryption.org/v1\n"));
            assert!(!bytes.windows(9).any(|w| w == b"PLAINTEXT"));
        }
    }

    #[test]
    fn vault_works_on_top_of_the_encrypted_provider() {
        let dir = tempfile::tempdir().unwrap();
        let identity = age::x25519::Identity::generate();
        let enc: Arc<dyn StorageProvider> = Arc::new(provider(dir.path(), &identity));
        let vault = Vault::open_with_provider(dir.path(), enc).unwrap();

        vault
            .write_note("folder/entry.md", "---\ntitle: Visible title\n---\nbody\n")
            .unwrap();

        assert_eq!(vault.note_paths().unwrap(), vec!["folder/entry.md"]);
        let note = vault.read_note("folder/entry.md").unwrap();
        assert_eq!(note.title, "Visible title");
        assert!(dir.path().join("folder/entry.md.age").is_file());
        assert!(!dir.path().join("folder/entry.md").exists());
    }
}

#[cfg(test)]
mod vault_tests {
    use super::*;
    use crate::index::{Index, SyncStats};
    use crate::refactor::rename_with_links;
    use crate::storage::LocalFsProvider;
    use crate::vault::Vault;
    use std::path::Path;

    fn encrypted_vault(dir: &Path) -> Vault {
        let identity = age::x25519::Identity::generate();
        let provider = EncryptedProvider::from_identity(
            Arc::new(LocalFsProvider::new(dir).unwrap()),
            identity,
            vec![],
        )
        .unwrap();
        Vault::open_with_provider(dir, Arc::new(provider)).unwrap()
    }

    /// Every file under `dir` (recursively) whose bytes contain `needle`.
    fn files_containing(dir: &Path, needle: &[u8]) -> Vec<String> {
        let mut found = Vec::new();
        let mut stack = vec![dir.to_path_buf()];
        while let Some(current) = stack.pop() {
            for entry in std::fs::read_dir(&current).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    stack.push(path);
                } else if std::fs::read(&path)
                    .unwrap()
                    .windows(needle.len())
                    .any(|w| w == needle)
                {
                    found.push(path.display().to_string());
                }
            }
        }
        found
    }

    #[test]
    fn index_and_search_work_over_encrypted_storage() {
        let dir = tempfile::tempdir().unwrap();
        let vault = encrypted_vault(dir.path());
        vault
            .write_note("a.md", "# Alpha\nuniqueword text\n")
            .unwrap();

        let mut index = Index::open_in_memory().unwrap();
        assert_eq!(index.sync(&vault).unwrap().added, 1);
        assert_eq!(index.search("uniqueword", 10).unwrap().len(), 1);
        assert_eq!(index.sync(&vault).unwrap(), SyncStats::default());
    }

    #[test]
    fn rename_with_links_keeps_notes_encrypted() {
        let dir = tempfile::tempdir().unwrap();
        let vault = encrypted_vault(dir.path());
        vault.write_note("folder/Target.md", "# T\n").unwrap();
        vault.write_note("a.md", "see [[Target]]\n").unwrap();

        let mut index = Index::open_in_memory().unwrap();
        rename_with_links(&vault, &mut index, "folder/Target.md", "folder/Renamed.md").unwrap();

        assert!(vault
            .read_note("a.md")
            .unwrap()
            .content
            .contains("[[Renamed]]"));
        assert_eq!(vault.read_note("folder/Renamed.md").unwrap().title, "T");
        assert!(dir.path().join("folder/Renamed.md.age").is_file());
        assert!(!dir.path().join("folder/Target.md.age").exists());
    }

    #[test]
    fn trash_keeps_ciphertext_only() {
        let dir = tempfile::tempdir().unwrap();
        let vault = encrypted_vault(dir.path());
        vault.write_note("t.md", "TRASH-MARKER body").unwrap();

        let trash = vault.move_to_trash("t.md").unwrap();
        assert!(trash.starts_with(".mdnotes/trash/"));
        assert!(vault.note_paths().unwrap().is_empty());
        assert!(dir.path().join(format!("{trash}.age")).is_file());
        assert!(files_containing(dir.path(), b"TRASH-MARKER").is_empty());
        let restored = vault.provider().read(&trash).unwrap();
        assert!(restored.windows(12).any(|w| w == b"TRASH-MARKER"));
    }

    #[test]
    fn attachments_are_stored_encrypted_and_listed_by_logical_name() {
        let dir = tempfile::tempdir().unwrap();
        let vault = encrypted_vault(dir.path());
        let source_dir = tempfile::tempdir().unwrap();
        let source = source_dir.path().join("pic.png");
        std::fs::write(&source, b"ATTACH-MARKER bytes").unwrap();

        let info = vault.import_attachment(&source).unwrap();
        assert!(info.path.ends_with("pic.png"));
        assert!(dir.path().join(format!("{}.age", info.path)).is_file());
        assert!(files_containing(dir.path(), b"ATTACH-MARKER").is_empty());

        let listed = vault.attachments().unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].path, info.path);
        assert_eq!(
            vault.provider().read(&info.path).unwrap(),
            b"ATTACH-MARKER bytes"
        );
    }
}
