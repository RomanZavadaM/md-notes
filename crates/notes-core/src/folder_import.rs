//! Import of a plain folder into the open vault.
//!
//! Every file goes through the vault's storage provider, so in an encrypted
//! vault it is encrypted on the way in (the source folder stays as it was and
//! should be deleted by the user afterwards). Existing vault files are never
//! overwritten. Hidden files and folders are skipped, with one exception: a
//! `.mdnotes/schema.json` that the vault does not have yet is copied, because
//! it only describes note types.

use std::fs;
use std::path::Path;

use serde::Serialize;

use crate::error::{io_err, Error, Result};
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
        self.import_dir(source, "", overwrite, &mut report)?;
        Ok(report)
    }

    fn import_dir(
        &self,
        dir: &Path,
        rel: &str,
        overwrite: bool,
        report: &mut ImportReport,
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
                    self.import_dir(&entry.path(), &child_rel, overwrite, report)?;
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
            if exists {
                if provider.read(&path)? == bytes {
                    report.skipped += 1;
                    continue;
                }
                provider.write(&path, &bytes)?;
                report.updated += 1;
            } else {
                provider.write(&path, &bytes)?;
                report.created += 1;
            }
        }
        Ok(())
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
                rejected: 0
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
