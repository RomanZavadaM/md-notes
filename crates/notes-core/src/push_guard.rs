//! Push guard for encrypted vaults (ADR-0007): before anything is sent to a
//! remote, every file the next commit would contain must be age ciphertext.
//!
//! The guard looks at the working tree, which is what `commit_git_worktree`
//! turns into the next commit. It refuses on the safe side: an unexpected
//! plain file is reported even if Git would ignore it. History that was
//! committed earlier is not scanned here (an encrypted vault is created in an
//! empty folder, so plaintext can only enter through later commits).

use std::fs;
use std::io::Read;
use std::path::Path;

use serde::Serialize;

use crate::error::{io_err, Error, Result};
use crate::vault::SERVICE_DIR;

/// File name inside `.mdnotes/` that marks an encrypted vault (same as
/// `encrypted_vault::ENCRYPTION_FILE`, repeated so the guard works in builds
/// without the `encrypted-vault` feature).
const ENCRYPTION_FILE: &str = "encryption.json";
const AGE_MAGIC: &[u8] = b"age-encryption.org/v1";
const ENCRYPTED_SUFFIX: &str = ".age";
/// Local-only service folders that Git never receives (`.mdnotes/.gitignore`).
const LOCAL_ONLY: &[&str] = &["cache", "trash"];
/// Plain files that may sit next to an encrypted vault.
const PLAIN_ROOT_FILES: &[&str] = &[".gitignore", ".gitattributes"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PlaintextReason {
    /// The file does not end with `.age`.
    NotEncrypted,
    /// The file ends with `.age` but does not start with the age header.
    InvalidHeader,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaintextFinding {
    /// Vault-relative path with `/` separators.
    pub path: String,
    pub reason: PlaintextReason,
}

/// Whether `root` is an encrypted vault.
pub fn is_encrypted_vault(root: &Path) -> bool {
    root.join(SERVICE_DIR).join(ENCRYPTION_FILE).is_file()
}

/// Lists files that would be sent in plaintext. Empty for a plain vault or
/// when everything that would be pushed is ciphertext.
pub fn find_plaintext(root: &Path) -> Result<Vec<PlaintextFinding>> {
    let mut findings = Vec::new();
    if !is_encrypted_vault(root) {
        return Ok(findings);
    }
    scan(root, "", &mut findings)?;
    findings.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(findings)
}

/// Refuses (with an error naming the first offending file) when an encrypted
/// vault holds anything that is not ciphertext. Plain vaults pass.
pub fn ensure_push_safe(root: &Path) -> Result<()> {
    let findings = find_plaintext(root)?;
    match findings.first() {
        None => Ok(()),
        Some(first) => Err(Error::Crypto(format!(
            "push refused: {} file(s) in the encrypted vault are not encrypted, first: {}",
            findings.len(),
            first.path
        ))),
    }
}

fn scan(dir: &Path, rel: &str, out: &mut Vec<PlaintextFinding>) -> Result<()> {
    for entry in fs::read_dir(dir).map_err(io_err(dir))? {
        let entry = entry.map_err(io_err(dir))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let child_rel = if rel.is_empty() {
            name.clone()
        } else {
            format!("{rel}/{name}")
        };
        let file_type = entry.file_type().map_err(io_err(&entry.path()))?;
        if file_type.is_symlink() {
            continue;
        }
        if file_type.is_dir() {
            if name == ".git" && rel.is_empty() {
                continue;
            }
            if rel == SERVICE_DIR && LOCAL_ONLY.contains(&name.as_str()) {
                continue;
            }
            scan(&entry.path(), &child_rel, out)?;
            continue;
        }
        if rel == SERVICE_DIR || rel.starts_with(&format!("{SERVICE_DIR}/")) {
            continue; // open service files: settings, schema, templates, public keys
        }
        if rel.is_empty() && PLAIN_ROOT_FILES.contains(&name.as_str()) {
            continue;
        }
        if !name.ends_with(ENCRYPTED_SUFFIX) {
            out.push(PlaintextFinding {
                path: child_rel,
                reason: PlaintextReason::NotEncrypted,
            });
            continue;
        }
        if !has_age_header(&entry.path())? {
            out.push(PlaintextFinding {
                path: child_rel,
                reason: PlaintextReason::InvalidHeader,
            });
        }
    }
    Ok(())
}

fn has_age_header(path: &Path) -> Result<bool> {
    let mut file = fs::File::open(path).map_err(io_err(path))?;
    let mut head = vec![0u8; AGE_MAGIC.len()];
    match file.read_exact(&mut head) {
        Ok(()) => Ok(head == AGE_MAGIC),
        Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => Ok(false),
        Err(err) => Err(io_err(path)(err)),
    }
}

#[cfg(all(test, feature = "encrypted-vault"))]
mod tests {
    use super::*;
    use crate::encrypted_vault::create_encrypted_vault;

    fn encrypted_vault() -> (tempfile::TempDir, crate::vault::Vault) {
        let tmp = tempfile::tempdir().unwrap();
        let created = create_encrypted_vault(tmp.path(), None, &[]).unwrap();
        (tmp, created.vault)
    }

    #[test]
    fn plain_vault_is_not_checked() {
        let tmp = tempfile::tempdir().unwrap();
        crate::vault::Vault::init(tmp.path(), None).unwrap();
        fs::write(tmp.path().join("a.md"), "# plain").unwrap();
        assert!(!is_encrypted_vault(tmp.path()));
        assert!(find_plaintext(tmp.path()).unwrap().is_empty());
        ensure_push_safe(tmp.path()).unwrap();
    }

    #[test]
    fn fresh_encrypted_vault_with_notes_is_safe() {
        let (tmp, vault) = encrypted_vault();
        vault.write_note("a.md", "# Secret\n\nbody").unwrap();
        vault.write_note("dir/b.md", "# Other").unwrap();
        assert!(find_plaintext(tmp.path()).unwrap().is_empty());
        ensure_push_safe(tmp.path()).unwrap();
    }

    #[test]
    fn plain_markdown_file_blocks_the_push() {
        let (tmp, vault) = encrypted_vault();
        vault.write_note("a.md", "# Secret").unwrap();
        fs::write(tmp.path().join("leak.md"), "# leaked").unwrap();
        fs::create_dir_all(tmp.path().join("dir")).unwrap();
        fs::write(tmp.path().join("dir").join("pic.png"), [1u8, 2, 3]).unwrap();
        let found = find_plaintext(tmp.path()).unwrap();
        let paths: Vec<_> = found.iter().map(|f| f.path.as_str()).collect();
        assert_eq!(paths, vec!["dir/pic.png", "leak.md"]);
        assert!(found
            .iter()
            .all(|f| f.reason == PlaintextReason::NotEncrypted));
        let err = ensure_push_safe(tmp.path()).unwrap_err().to_string();
        assert!(err.contains("push refused"), "{err}");
        assert!(err.contains("dir/pic.png"), "{err}");
    }

    #[test]
    fn age_suffix_without_age_header_is_rejected() {
        let (tmp, _vault) = encrypted_vault();
        fs::write(tmp.path().join("fake.md.age"), "# I am plain text").unwrap();
        fs::write(tmp.path().join("empty.md.age"), "").unwrap();
        let found = find_plaintext(tmp.path()).unwrap();
        assert_eq!(found.len(), 2);
        assert!(found
            .iter()
            .all(|f| f.reason == PlaintextReason::InvalidHeader));
    }

    #[test]
    fn service_files_cache_trash_git_and_symlinks_are_ignored() {
        let (tmp, vault) = encrypted_vault();
        vault.write_note("a.md", "# Secret").unwrap();
        let service = tmp.path().join(SERVICE_DIR);
        fs::create_dir_all(service.join("cache")).unwrap();
        fs::write(service.join("cache").join("index.db"), "plain cache").unwrap();
        fs::create_dir_all(service.join("templates")).unwrap();
        fs::write(service.join("templates").join("t.md"), "# template").unwrap();
        fs::create_dir_all(tmp.path().join(".git")).unwrap();
        fs::write(tmp.path().join(".git").join("HEAD"), "ref").unwrap();
        fs::write(tmp.path().join(".gitignore"), "cache/\n").unwrap();
        assert!(find_plaintext(tmp.path()).unwrap().is_empty());
    }

    #[test]
    fn a_plain_file_inside_a_user_folder_named_like_service_dir_is_still_found() {
        let (tmp, _vault) = encrypted_vault();
        fs::create_dir_all(tmp.path().join("notes").join(SERVICE_DIR)).unwrap();
        fs::write(
            tmp.path().join("notes").join(SERVICE_DIR).join("x.md"),
            "# plain",
        )
        .unwrap();
        let found = find_plaintext(tmp.path()).unwrap();
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].path, "notes/.mdnotes/x.md");
    }
}
