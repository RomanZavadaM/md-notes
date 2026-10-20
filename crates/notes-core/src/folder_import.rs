//! Import of a plain folder into the open vault.
//!
//! Every file goes through the vault's storage provider, so in an encrypted
//! vault it is encrypted on the way in (the source folder stays as it was and
//! should be deleted by the user afterwards). Existing vault files are never
//! overwritten. Hidden files and folders are skipped, with one exception: a
//! `.mdnotes/schema.json` that the vault does not have yet is copied, because
//! it only describes note types.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::Serialize;

use crate::error::{io_err, Error, Result};
use crate::identity;
use crate::paths;
use crate::vault::{Vault, SERVICE_DIR};

/// Largest single file the import accepts.
const MAX_FILE_BYTES: u64 = 100 * 1024 * 1024;
const SCHEMA_FILE: &str = "schema.json";

/// What an import did.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportReport {
    /// Files written to the vault.
    pub created: u32,
    /// Existing files replaced because their content differed (overwrite mode).
    pub updated: u32,
    /// Files left alone because the vault already has a file at that path.
    pub skipped: u32,
    /// Files that could not be imported (too large, odd names, unreadable).
    pub rejected: u32,
    /// Notes folded into an existing note about the same subject (same identity code, ADR-0008).
    pub merged: u32,
    /// Differing scalar values kept from the existing note during merges.
    pub conflicts: u32,
}

/// Identity codes of the notes already in the vault, built on first use.
#[derive(Default)]
struct IdentityIndex {
    map: Option<HashMap<String, String>>,
}

impl IdentityIndex {
    fn lookup(&mut self, vault: &Vault, key: &str) -> Result<Option<String>> {
        if self.map.is_none() {
            let provider = vault.provider();
            let mut map = HashMap::new();
            for path in vault.note_paths()? {
                let Ok(bytes) = provider.read(&path) else {
                    continue;
                };
                let Ok(text) = String::from_utf8(bytes) else {
                    continue;
                };
                if let Some(found) = identity::identity_of_content(&text) {
                    map.entry(found).or_insert(path);
                }
            }
            self.map = Some(map);
        }
        Ok(self.map.as_ref().and_then(|m| m.get(key)).cloned())
    }

    fn remember(&mut self, key: String, path: &str) {
        if let Some(map) = self.map.as_mut() {
            map.entry(key).or_insert_with(|| path.to_string());
        }
    }
}

impl Vault {
    /// Imports `source` (a plain folder) into the vault, keeping its folder
    /// structure. Existing files are left alone unless `overwrite` is set, in
    /// which case files with different content are replaced. Returns how many
    /// files were created, updated, skipped or rejected.
    pub fn import_folder(&self, source: &Path, overwrite: bool) -> Result<ImportReport> {
        if !source.is_dir() {
            return Err(Error::NotFound(source.display().to_string()));
        }
        // Importing the vault into itself would loop through its own files.
        if let (Ok(src), Ok(root)) = (source.canonicalize(), self.root().canonicalize()) {
            if src.starts_with(&root) || root.starts_with(&src) {
                return Err(Error::InvalidPath(
                    "the source folder and the vault overlap".into(),
                ));
            }
        }
        let mut report = ImportReport::default();
        let mut identities = IdentityIndex::default();
        self.import_dir(source, "", overwrite, &mut report, &mut identities)?;
        Ok(report)
    }

    fn import_dir(
        &self,
        dir: &Path,
        rel: &str,
        overwrite: bool,
        report: &mut ImportReport,
        identities: &mut IdentityIndex,
    ) -> Result<()> {
        let provider = self.provider();
        for entry in fs::read_dir(dir).map_err(io_err(dir))? {
            let entry = entry.map_err(io_err(dir))?;
            let Some(name) = entry.file_name().to_str().map(str::to_string) else {
                report.rejected += 1;
                continue;
            };
            let file_type = entry.file_type().map_err(io_err(&entry.path()))?;
            if file_type.is_symlink() {
                continue;
            }
            let child_rel = if rel.is_empty() {
                name.clone()
            } else {
                format!("{rel}/{name}")
            };
            if file_type.is_dir() {
                if name == SERVICE_DIR && rel.is_empty() {
                    self.import_schema(&entry.path(), overwrite, report)?;
                } else if !name.starts_with('.') {
                    self.import_dir(&entry.path(), &child_rel, overwrite, report, identities)?;
                }
                continue;
            }
            if name.starts_with('.') {
                continue;
            }
            let Ok(path) = paths::normalize(&child_rel) else {
                report.rejected += 1;
                continue;
            };
            let size = entry.metadata().map_err(io_err(&entry.path()))?.len();
            if path.is_empty() || size > MAX_FILE_BYTES {
                report.rejected += 1;
                continue;
            }
            let exists = provider.metadata(&path)?.is_some();
            if exists && !overwrite {
                report.skipped += 1;
                continue;
            }
            let bytes = fs::read(entry.path()).map_err(io_err(&entry.path()))?;
            let identity = if paths::is_note(&path) {
                std::str::from_utf8(&bytes)
                    .ok()
                    .and_then(identity::identity_of_content)
            } else {
                None
            };
            if exists {
                if provider.read(&path)? == bytes {
                    report.skipped += 1;
                    continue;
                }
                provider.write(&path, &bytes)?;
                report.updated += 1;
            } else {
                // A note about a subject the vault already knows (same code) is folded in, not duplicated.
                if let Some(key) = &identity {
                    if let Some(target) = identities.lookup(self, key)? {
                        if self.merge_import(&target, &bytes, &child_rel, report)? {
                            continue;
                        }
                    }
                }
                provider.write(&path, &bytes)?;
                report.created += 1;
                if let Some(key) = identity {
                    identities.remember(key, &path);
                }
            }
        }
        Ok(())
    }

    /// Folds an incoming note into the existing note `target` (ADR-0008). Returns
    /// false when the merge is not possible (no valid front matter), so the
    /// caller imports the file as a separate note.
    fn merge_import(
        &self,
        target: &str,
        incoming: &[u8],
        incoming_path: &str,
        report: &mut ImportReport,
    ) -> Result<bool> {
        let provider = self.provider();
        let Ok(existing) = String::from_utf8(provider.read(target)?) else {
            return Ok(false);
        };
        let Ok(incoming) = std::str::from_utf8(incoming) else {
            return Ok(false);
        };
        let Some(outcome) = identity::merge_into(&existing, incoming, incoming_path) else {
            return Ok(false);
        };
        if outcome.changed {
            provider.write(target, outcome.content.as_bytes())?;
            report.merged += 1;
            report.conflicts += outcome.conflicts;
        } else {
            report.skipped += 1;
        }
        Ok(true)
    }

    /// Copies `.mdnotes/schema.json` when the vault has none yet.
    fn import_schema(
        &self,
        service_dir: &Path,
        overwrite: bool,
        report: &mut ImportReport,
    ) -> Result<()> {
        let source = service_dir.join(SCHEMA_FILE);
        if !source.is_file() {
            return Ok(());
        }
        let target = format!("{SERVICE_DIR}/{SCHEMA_FILE}");
        let provider = self.provider();
        let exists = provider.metadata(&target)?.is_some();
        if exists && !overwrite {
            report.skipped += 1;
            return Ok(());
        }
        let bytes = fs::read(&source).map_err(io_err(&source))?;
        if exists {
            if provider.read(&target)? == bytes {
                report.skipped += 1;
                return Ok(());
            }
            provider.write(&target, &bytes)?;
            report.updated += 1;
        } else {
            provider.write(&target, &bytes)?;
            report.created += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source_folder() -> tempfile::TempDir {
        let src = tempfile::tempdir().unwrap();
        fs::create_dir_all(src.path().join("Справи")).unwrap();
        fs::create_dir_all(src.path().join(".mdnotes")).unwrap();
        fs::create_dir_all(src.path().join(".hidden")).unwrap();
        fs::write(
            src.path().join("Головна.md"),
            "# Головна\n[[Справи/Одна]]\n",
        )
        .unwrap();
        fs::write(src.path().join("Справи/Одна.md"), "# Одна\n").unwrap();
        fs::write(src.path().join("pic.png"), [1u8, 2, 3]).unwrap();
        fs::write(src.path().join(".secret"), "no").unwrap();
        fs::write(src.path().join(".hidden/x.md"), "no").unwrap();
        fs::write(src.path().join(".mdnotes/schema.json"), "{\"version\":1}").unwrap();
        fs::write(src.path().join(".mdnotes/other.json"), "{}").unwrap();
        src
    }

    #[test]
    fn imports_a_folder_into_a_plain_vault_without_overwriting() {
        let dir = tempfile::tempdir().unwrap();
        let vault = Vault::init(dir.path(), None).unwrap();
        let src = source_folder();

        let report = vault.import_folder(src.path(), false).unwrap();
        assert_eq!(
            report,
            ImportReport {
                created: 4,
                updated: 0,
                skipped: 0,
                rejected: 0,
                merged: 0,
                conflicts: 0
            }
        );
        assert_eq!(vault.read_note("Справи/Одна.md").unwrap().title, "Одна");
        assert!(dir.path().join("pic.png").is_file());
        assert!(dir.path().join(".mdnotes/schema.json").is_file());
        assert!(!dir.path().join(".mdnotes/other.json").exists());
        assert!(!dir.path().join(".secret").exists());
        assert!(!dir.path().join(".hidden").exists());

        // The second run leaves existing files untouched.
        fs::write(dir.path().join("Головна.md"), "# Змінено\n").unwrap();
        let again = vault.import_folder(src.path(), false).unwrap();
        assert_eq!(again.created, 0);
        assert_eq!(again.skipped, 4);
        assert!(fs::read_to_string(dir.path().join("Головна.md"))
            .unwrap()
            .contains("Змінено"));

        // Overwrite mode replaces only what differs.
        let replaced = vault.import_folder(src.path(), true).unwrap();
        assert_eq!(replaced.updated, 1);
        assert_eq!(replaced.created, 0);
        assert_eq!(replaced.skipped, 3);
        assert!(fs::read_to_string(dir.path().join("Головна.md"))
            .unwrap()
            .contains("[[Справи/Одна]]"));
    }

    const FIAKR_VAULT: &str = "---\ntitle: \"ТОВ «ФІАКР-ЛЬВІВ»\"\nedrpou: \"03114848\"\nstatus: unverified\nrelations:\n  - {type: засновник, to: Левик, dir: in}\n---\n# ТОВ «ФІАКР-ЛЬВІВ»\n\nТекст у сховищі.\n";
    const FIAKR_IMPORT: &str = "---\ntitle: \"ТЗОВ ФІАКР-ЛЬВІВ\"\nedrpou: \"03114848\"\nstatus: verified\nrelations:\n  - {type: керівник, to: Коцан, dir: in}\n---\n# ТЗОВ ФІАКР-ЛЬВІВ\n\nНовий текст.\n";

    #[test]
    fn a_note_with_a_known_code_is_merged_not_duplicated() {
        let dir = tempfile::tempdir().unwrap();
        let vault = Vault::init(dir.path(), None).unwrap();
        vault.write_note("Суб'єкти/Фіакр.md", FIAKR_VAULT).unwrap();

        let src = tempfile::tempdir().unwrap();
        fs::write(src.path().join("Фіакр Львів.md"), FIAKR_IMPORT).unwrap();

        let report = vault.import_folder(src.path(), false).unwrap();
        assert_eq!(report.merged, 1);
        assert_eq!(report.created, 0);
        assert_eq!(report.conflicts, 1); // `status` differs
        assert!(!dir.path().join("Фіакр Львів.md").exists());

        let merged = fs::read_to_string(dir.path().join("Суб'єкти/Фіакр.md")).unwrap();
        assert!(merged.contains("Текст у сховищі."));
        assert!(merged.contains("Новий текст."));
        assert!(merged.contains("ТЗОВ ФІАКР-ЛЬВІВ")); // kept as an alias
        assert!(merged.contains("Коцан"));
        assert!(merged.contains("Левик"));
        assert!(merged.contains("`status`"));

        // The same import again changes nothing and is not counted as a merge.
        let again = vault.import_folder(src.path(), true).unwrap();
        assert_eq!(again.merged, 0);
        assert_eq!(again.created, 0);
        assert_eq!(again.skipped, 1);
        assert_eq!(
            fs::read_to_string(dir.path().join("Суб'єкти/Фіакр.md")).unwrap(),
            merged
        );
    }

    #[test]
    fn notes_without_a_matching_code_are_imported_as_before() {
        let dir = tempfile::tempdir().unwrap();
        let vault = Vault::init(dir.path(), None).unwrap();
        vault.write_note("Суб'єкти/Фіакр.md", FIAKR_VAULT).unwrap();

        let src = tempfile::tempdir().unwrap();
        // Same name pattern but another code, and a note without any code.
        fs::write(
            src.path().join("Інша.md"),
            "---\ntitle: Інша\nedrpou: \"12345678\"\n---\n# Інша\n",
        )
        .unwrap();
        fs::write(src.path().join("Без коду.md"), "# Без коду\n").unwrap();

        let report = vault.import_folder(src.path(), false).unwrap();
        assert_eq!(report.merged, 0);
        assert_eq!(report.created, 2);
    }

    #[test]
    fn two_incoming_notes_with_one_code_are_folded_together() {
        let dir = tempfile::tempdir().unwrap();
        let vault = Vault::init(dir.path(), None).unwrap();

        let src = tempfile::tempdir().unwrap();
        fs::write(src.path().join("A.md"), FIAKR_VAULT).unwrap();
        fs::write(src.path().join("B.md"), FIAKR_IMPORT).unwrap();

        let report = vault.import_folder(src.path(), false).unwrap();
        assert_eq!(report.created, 1);
        assert_eq!(report.merged, 1);
    }

    #[test]
    fn refuses_a_source_that_overlaps_the_vault_or_is_missing() {
        let dir = tempfile::tempdir().unwrap();
        let vault = Vault::init(dir.path(), None).unwrap();
        assert!(matches!(
            vault.import_folder(dir.path(), false),
            Err(Error::InvalidPath(_))
        ));
        assert!(matches!(
            vault.import_folder(&dir.path().join("missing"), false),
            Err(Error::NotFound(_))
        ));
    }

    #[cfg(feature = "encrypted-vault")]
    #[test]
    fn imported_files_are_encrypted_in_an_encrypted_vault() {
        use crate::encrypted_vault::create_encrypted_vault;

        let dir = tempfile::tempdir().unwrap();
        let created = create_encrypted_vault(dir.path(), None, &[]).unwrap();
        let src = source_folder();

        let report = created.vault.import_folder(src.path(), false).unwrap();
        assert_eq!(report.created, 4);
        assert_eq!(
            created.vault.read_note("Справи/Одна.md").unwrap().title,
            "Одна"
        );
        assert!(created.vault.read_note("Головна.md").is_ok());

        // Nothing readable and no names on disk; the schema stays open.
        let mut names = Vec::new();
        for entry in fs::read_dir(dir.path()).unwrap() {
            let entry = entry.unwrap();
            names.push(entry.file_name().to_string_lossy().into_owned());
        }
        assert!(names.iter().all(|n| n.ends_with(".age") || n == ".mdnotes"));
        assert!(!names.iter().any(|n| n.contains("Головна")));
        assert!(dir.path().join(".mdnotes/schema.json").is_file());
        crate::push_guard::ensure_push_safe(dir.path()).unwrap();
    }
}
