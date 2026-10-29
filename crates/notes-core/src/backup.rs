use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Component, Path, PathBuf};
use std::time::SystemTime;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use crate::error::{io_err, Error, Result};
use crate::vault::SERVICE_DIR;

const MANIFEST_NAME: &str = "manifest.json";
const FORMAT_VERSION: u32 = 1;
const MAX_FILES: usize = 100_000;
const MAX_FILE_SIZE: u64 = 4 * 1024 * 1024 * 1024;
const MAX_TOTAL_SIZE: u64 = 16 * 1024 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BackupManifestEntry {
    pub path: String,
    pub size: u64,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BackupManifest {
    pub format: String,
    pub format_version: u32,
    pub app_version: String,
    pub created_at: String,
    pub vault_name: String,
    pub encrypted: bool,
    pub files: Vec<BackupManifestEntry>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BackupSummary {
    pub archive: String,
    pub files: usize,
    pub bytes: u64,
    pub encrypted: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RestoreSummary {
    pub path: String,
    pub files: usize,
    pub bytes: u64,
    pub encrypted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SourceFile {
    rel: String,
    path: PathBuf,
    size: u64,
    modified: Option<SystemTime>,
}

fn backup_error(message: impl Into<String>) -> Error {
    Error::Backup(message.into())
}

fn sha256_hex(bytes: impl AsRef<[u8]>) -> String {
    let digest = Sha256::digest(bytes.as_ref());
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn valid_relative_path(text: &str) -> Result<PathBuf> {
    if text.is_empty() || text.contains('\\') {
        return Err(backup_error(format!("unsafe archive path: {text:?}")));
    }
    let path = Path::new(text);
    if path.is_absolute() {
        return Err(backup_error(format!("absolute archive path: {text:?}")));
    }
    for component in path.components() {
        if !matches!(component, Component::Normal(_)) {
            return Err(backup_error(format!("unsafe archive path: {text:?}")));
        }
    }
    Ok(path.to_path_buf())
}

fn should_exclude(rel: &str, file_name: &str) -> bool {
    let parts = rel.split('/').collect::<Vec<_>>();
    if parts.iter().any(|part| *part == ".git") {
        return true;
    }
    if parts.len() >= 2 && parts[0] == SERVICE_DIR && parts[1] == "cache" {
        return true;
    }
    if file_name.ends_with(".mdnotes-tmp")
        || file_name == ".DS_Store"
        || file_name.eq_ignore_ascii_case("Thumbs.db")
        || file_name.eq_ignore_ascii_case("desktop.ini")
    {
        return true;
    }
    false
}

fn relative_text(root: &Path, path: &Path) -> Result<String> {
    let rel = path
        .strip_prefix(root)
        .map_err(|_| backup_error("file escaped vault root"))?;
    let mut parts = Vec::new();
    for component in rel.components() {
        match component {
            Component::Normal(value) => parts.push(value.to_string_lossy().into_owned()),
            _ => return Err(backup_error(format!("unsafe vault path: {}", rel.display()))),
        }
    }
    let text = parts.join("/");
    valid_relative_path(&text)?;
    Ok(text)
}

fn collect_source_files(root: &Path) -> Result<Vec<SourceFile>> {
    if !root.is_dir() {
        return Err(backup_error(format!(
            "vault root is not a directory: {}",
            root.display()
        )));
    }
    let mut stack = vec![root.to_path_buf()];
    let mut files = Vec::new();

    while let Some(dir) = stack.pop() {
        let entries = fs::read_dir(&dir).map_err(io_err(&dir))?;
        for entry in entries {
            let entry = entry.map_err(io_err(&dir))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path).map_err(io_err(&path))?;
            let rel = relative_text(root, &path)?;
            let name = entry.file_name().to_string_lossy().into_owned();
            if should_exclude(&rel, &name) {
                continue;
            }
            if metadata.file_type().is_symlink() {
                return Err(backup_error(format!(
                    "symbolic links are not allowed in a vault backup: {rel}"
                )));
            }
            if metadata.is_dir() {
                stack.push(path);
            } else if metadata.is_file() {
                if files.len() >= MAX_FILES {
                    return Err(backup_error("vault contains too many files"));
                }
                if metadata.len() > MAX_FILE_SIZE {
                    return Err(backup_error(format!("file is too large for backup: {rel}")));
                }
                files.push(SourceFile {
                    rel,
                    path,
                    size: metadata.len(),
                    modified: metadata.modified().ok(),
                });
            }
        }
    }

    files.sort_by(|a, b| a.rel.cmp(&b.rel));
    let mut total = 0_u64;
    let mut case_keys = BTreeSet::new();
    for file in &files {
        total = total
            .checked_add(file.size)
            .ok_or_else(|| backup_error("vault size overflow"))?;
        if total > MAX_TOTAL_SIZE {
            return Err(backup_error("vault is too large for a single backup"));
        }
        let key = file.rel.to_lowercase();
        if !case_keys.insert(key) {
            return Err(backup_error(format!(
                "case-insensitive path collision: {}",
                file.rel
            )));
        }
    }
    Ok(files)
}

fn source_snapshot(files: &[SourceFile]) -> Vec<(String, u64, Option<SystemTime>)> {
    files
        .iter()
        .map(|file| (file.rel.clone(), file.size, file.modified))
        .collect()
}

fn current_source_file(file: &SourceFile) -> Result<(u64, Option<SystemTime>)> {
    let metadata = fs::metadata(&file.path).map_err(io_err(&file.path))?;
    Ok((metadata.len(), metadata.modified().ok()))
}

pub fn create_vault_backup(root: &Path, destination: &Path) -> Result<BackupSummary> {
    if destination.exists() {
        return Err(Error::AlreadyExists(destination.display().to_string()));
    }
    let parent = destination
        .parent()
        .ok_or_else(|| backup_error("backup destination has no parent directory"))?;
    if !parent.is_dir() {
        return Err(backup_error(format!(
            "backup destination directory does not exist: {}",
            parent.display()
        )));
    }

    let files = collect_source_files(root)?;
    let before = source_snapshot(&files);
    let encrypted = root.join(SERVICE_DIR).join("encryption.json").is_file();
    let vault_name = root
        .file_name()
        .map(|value| value.to_string_lossy().into_owned())
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "MD Notes".to_string());

    let file_name = destination
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("backup.zip");
    let temp = parent.join(format!(".{file_name}.mdnotes-tmp-{}", ulid::Ulid::new()));

    let result = (|| -> Result<BackupSummary> {
        let file = File::create_new(&temp).map_err(io_err(&temp))?;
        let writer = BufWriter::new(file);
        let mut zip = ZipWriter::new(writer);
        let options = SimpleFileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .unix_permissions(0o600);
        let mut manifest_files = Vec::with_capacity(files.len());
        let mut total = 0_u64;
        let mut buffer = vec![0_u8; 128 * 1024];

        for source in &files {
            zip.start_file(&source.rel, options)
                .map_err(|error| backup_error(error.to_string()))?;
            let mut reader = BufReader::new(File::open(&source.path).map_err(io_err(&source.path))?);
            let mut hasher = Sha256::new();
            let mut written = 0_u64;
            loop {
                let count = reader.read(&mut buffer).map_err(io_err(&source.path))?;
                if count == 0 {
                    break;
                }
                written = written
                    .checked_add(count as u64)
                    .ok_or_else(|| backup_error("backup byte count overflow"))?;
                if written > source.size {
                    return Err(backup_error(format!(
                        "source changed while backup was running: {}",
                        source.rel
                    )));
                }
                hasher.update(&buffer[..count]);
                zip.write_all(&buffer[..count])
                    .map_err(|error| backup_error(error.to_string()))?;
            }
            let after = current_source_file(source)?;
            if written != source.size || after != (source.size, source.modified) {
                return Err(backup_error(format!(
                    "source changed while backup was running: {}",
                    source.rel
                )));
            }
            total += written;
            manifest_files.push(BackupManifestEntry {
                path: source.rel.clone(),
                size: written,
                sha256: hasher
                    .finalize()
                    .iter()
                    .map(|byte| format!("{byte:02x}"))
                    .collect(),
            });
        }

        let after_files = collect_source_files(root)?;
        if before != source_snapshot(&after_files) {
            return Err(backup_error(
                "vault contents changed while backup was running; try again",
            ));
        }

        let manifest = BackupManifest {
            format: "md-notes-backup".to_string(),
            format_version: FORMAT_VERSION,
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            created_at: Utc::now().to_rfc3339(),
            vault_name,
            encrypted,
            files: manifest_files,
        };
        let manifest_json = serde_json::to_vec_pretty(&manifest)
            .map_err(|error| backup_error(error.to_string()))?;
        zip.start_file(MANIFEST_NAME, options)
            .map_err(|error| backup_error(error.to_string()))?;
        zip.write_all(&manifest_json)
            .map_err(|error| backup_error(error.to_string()))?;
        zip.finish()
            .map_err(|error| backup_error(error.to_string()))?
            .flush()
            .map_err(io_err(&temp))?;

        fs::rename(&temp, destination).map_err(io_err(destination))?;
        Ok(BackupSummary {
            archive: destination.display().to_string(),
            files: files.len(),
            bytes: total,
            encrypted,
        })
    })();

    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}

fn read_manifest(archive: &mut ZipArchive<File>) -> Result<BackupManifest> {
    let mut file = archive
        .by_name(MANIFEST_NAME)
        .map_err(|_| backup_error("backup manifest.json is missing"))?;
    if file.size() > 8 * 1024 * 1024 {
        return Err(backup_error("backup manifest is unexpectedly large"));
    }
    let mut raw = Vec::with_capacity(file.size() as usize);
    file.read_to_end(&mut raw)
        .map_err(|error| backup_error(error.to_string()))?;
    let manifest: BackupManifest =
        serde_json::from_slice(&raw).map_err(|error| backup_error(error.to_string()))?;
    if manifest.format != "md-notes-backup" || manifest.format_version != FORMAT_VERSION {
        return Err(backup_error(format!(
            "unsupported backup format {} v{}",
            manifest.format, manifest.format_version
        )));
    }
    Ok(manifest)
}

fn validate_manifest(manifest: &BackupManifest) -> Result<BTreeMap<String, BackupManifestEntry>> {
    if manifest.files.len() > MAX_FILES {
        return Err(backup_error("backup contains too many files"));
    }
    let mut total = 0_u64;
    let mut paths = BTreeMap::new();
    let mut case_keys = BTreeSet::new();
    for entry in &manifest.files {
        valid_relative_path(&entry.path)?;
        if entry.path == MANIFEST_NAME {
            return Err(backup_error("manifest cannot list itself"));
        }
        if entry.size > MAX_FILE_SIZE {
            return Err(backup_error(format!(
                "backup file is too large: {}",
                entry.path
            )));
        }
        total = total
            .checked_add(entry.size)
            .ok_or_else(|| backup_error("backup size overflow"))?;
        if total > MAX_TOTAL_SIZE {
            return Err(backup_error("backup is too large to restore safely"));
        }
        if entry.sha256.len() != 64 || !entry.sha256.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(backup_error(format!(
                "invalid checksum in manifest: {}",
                entry.path
            )));
        }
        if !case_keys.insert(entry.path.to_lowercase()) {
            return Err(backup_error(format!(
                "case-insensitive path collision: {}",
                entry.path
            )));
        }
        if paths.insert(entry.path.clone(), entry.clone()).is_some() {
            return Err(backup_error(format!(
                "duplicate path in manifest: {}",
                entry.path
            )));
        }
    }
    Ok(paths)
}

fn verify_archive(
    archive: &mut ZipArchive<File>,
    manifest: &BackupManifest,
) -> Result<BTreeMap<String, BackupManifestEntry>> {
    let expected = validate_manifest(manifest)?;
    let mut seen = BTreeSet::new();
    let mut buffer = vec![0_u8; 128 * 1024];

    for index in 0..archive.len() {
        let mut file = archive
            .by_index(index)
            .map_err(|error| backup_error(error.to_string()))?;
        let name = file.name().to_string();
        if !seen.insert(name.clone()) {
            return Err(backup_error(format!("duplicate ZIP entry: {name}")));
        }
        if name == MANIFEST_NAME {
            continue;
        }
        if file.is_dir() {
            return Err(backup_error(format!(
                "directory entries are not allowed in backup: {name}"
            )));
        }
        valid_relative_path(&name)?;
        let entry = expected
            .get(&name)
            .ok_or_else(|| backup_error(format!("unexpected file in backup: {name}")))?;
        if file.size() != entry.size {
            return Err(backup_error(format!("size mismatch: {name}")));
        }

        let mut hasher = Sha256::new();
        let mut count_total = 0_u64;
        loop {
            let count = file
                .read(&mut buffer)
                .map_err(|error| backup_error(error.to_string()))?;
            if count == 0 {
                break;
            }
            count_total += count as u64;
            if count_total > entry.size {
                return Err(backup_error(format!("expanded size mismatch: {name}")));
            }
            hasher.update(&buffer[..count]);
        }
        let checksum: String = hasher
            .finalize()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        if checksum != entry.sha256.to_lowercase() {
            return Err(backup_error(format!("checksum mismatch: {name}")));
        }
    }

    if !seen.contains(MANIFEST_NAME) {
        return Err(backup_error("backup manifest.json is missing"));
    }
    for path in expected.keys() {
        if !seen.contains(path) {
            return Err(backup_error(format!("backup file is missing: {path}")));
        }
    }
    Ok(expected)
}

fn safe_restore_name(name: &str) -> String {
    let mut result = String::new();
    for ch in name.trim().chars() {
        if matches!(ch, '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*') || ch.is_control() {
            result.push('_');
        } else {
            result.push(ch);
        }
    }
    let result = result.trim_matches([' ', '.']).to_string();
    if result.is_empty() {
        "MD Notes".to_string()
    } else {
        result
    }
}

fn unique_restore_target(parent: &Path, vault_name: &str) -> PathBuf {
    let base = format!("{}-restored", safe_restore_name(vault_name));
    let mut candidate = parent.join(&base);
    let mut number = 2_u32;
    while candidate.exists() {
        candidate = parent.join(format!("{base}-{number}"));
        number += 1;
    }
    candidate
}

pub fn restore_vault_backup(archive_path: &Path, destination_parent: &Path) -> Result<RestoreSummary> {
    if !destination_parent.is_dir() {
        return Err(backup_error(format!(
            "restore destination is not a directory: {}",
            destination_parent.display()
        )));
    }

    let mut archive = ZipArchive::new(File::open(archive_path).map_err(io_err(archive_path))?)
        .map_err(|error| backup_error(error.to_string()))?;
    let manifest = read_manifest(&mut archive)?;
    let expected = verify_archive(&mut archive, &manifest)?;
    let target = unique_restore_target(destination_parent, &manifest.vault_name);
    let temp = destination_parent.join(format!(".mdnotes-restore-{}", ulid::Ulid::new()));

    let result = (|| -> Result<RestoreSummary> {
        fs::create_dir(&temp).map_err(io_err(&temp))?;
        let mut total = 0_u64;
        let mut buffer = vec![0_u8; 128 * 1024];

        for entry in expected.values() {
            let rel = valid_relative_path(&entry.path)?;
            let out = temp.join(rel);
            if let Some(parent) = out.parent() {
                fs::create_dir_all(parent).map_err(io_err(parent))?;
            }
            let mut input = archive
                .by_name(&entry.path)
                .map_err(|error| backup_error(error.to_string()))?;
            let mut output = BufWriter::new(File::create_new(&out).map_err(io_err(&out))?);
            let mut written = 0_u64;
            loop {
                let count = input
                    .read(&mut buffer)
                    .map_err(|error| backup_error(error.to_string()))?;
                if count == 0 {
                    break;
                }
                written += count as u64;
                if written > entry.size {
                    return Err(backup_error(format!(
                        "expanded size mismatch during restore: {}",
                        entry.path
                    )));
                }
                output.write_all(&buffer[..count]).map_err(io_err(&out))?;
            }
            output.flush().map_err(io_err(&out))?;
            if written != entry.size {
                return Err(backup_error(format!(
                    "size mismatch during restore: {}",
                    entry.path
                )));
            }
            total += written;
        }

        fs::rename(&temp, &target).map_err(io_err(&target))?;
        Ok(RestoreSummary {
            path: target.display().to_string(),
            files: expected.len(),
            bytes: total,
            encrypted: manifest.encrypted,
        })
    })();

    if result.is_err() {
        let _ = fs::remove_dir_all(&temp);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn make_vault() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join(".mdnotes/cache")).unwrap();
        fs::create_dir_all(dir.path().join(".mdnotes/trash")).unwrap();
        fs::create_dir_all(dir.path().join("attachments")).unwrap();
        fs::write(dir.path().join(".mdnotes/config.json"), b"{\"version\":1}\n").unwrap();
        fs::write(dir.path().join(".mdnotes/cache/index.db"), b"CACHE").unwrap();
        fs::write(dir.path().join(".mdnotes/trash/deleted.md"), b"deleted").unwrap();
        fs::write(dir.path().join("Нотатка.md"), "Привіт".as_bytes()).unwrap();
        fs::write(dir.path().join("attachments/file.bin"), [1_u8, 2, 3, 4]).unwrap();
        dir
    }

    #[test]
    fn round_trip_skips_cache_and_git() {
        let vault = make_vault();
        fs::create_dir_all(vault.path().join(".git")).unwrap();
        fs::write(vault.path().join(".git/config"), b"secret-ish remote").unwrap();
        let holder = tempfile::tempdir().unwrap();
        let archive = holder.path().join("vault.zip");
        let summary = create_vault_backup(vault.path(), &archive).unwrap();
        assert!(summary.files >= 4);

        let restore_parent = tempfile::tempdir().unwrap();
        let restored = restore_vault_backup(&archive, restore_parent.path()).unwrap();
        let restored = PathBuf::from(restored.path);
        assert_eq!(fs::read(restored.join("Нотатка.md")).unwrap(), "Привіт".as_bytes());
        assert_eq!(
            fs::read(restored.join(".mdnotes/trash/deleted.md")).unwrap(),
            b"deleted"
        );
        assert!(!restored.join(".mdnotes/cache/index.db").exists());
        assert!(!restored.join(".git").exists());
    }

    #[test]
    fn checksum_mismatch_is_rejected_before_restore() {
        let vault = make_vault();
        let holder = tempfile::tempdir().unwrap();
        let archive = holder.path().join("vault.zip");
        create_vault_backup(vault.path(), &archive).unwrap();

        let broken = holder.path().join("broken.zip");
        let source = File::open(&archive).unwrap();
        let mut input = ZipArchive::new(source).unwrap();
        let output = File::create(&broken).unwrap();
        let mut writer = ZipWriter::new(output);
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
        for index in 0..input.len() {
            let mut file = input.by_index(index).unwrap();
            let name = file.name().to_string();
            writer.start_file(&name, options).unwrap();
            if name == "Нотатка.md" {
                writer.write_all(b"tampered").unwrap();
            } else {
                std::io::copy(&mut file, &mut writer).unwrap();
            }
        }
        writer.finish().unwrap();

        let parent = tempfile::tempdir().unwrap();
        let error = restore_vault_backup(&broken, parent.path()).unwrap_err();
        assert!(error.to_string().contains("checksum mismatch") || error.to_string().contains("size mismatch"));
        assert!(fs::read_dir(parent.path()).unwrap().next().is_none());
    }

    #[test]
    fn traversal_entry_is_rejected() {
        let holder = tempfile::tempdir().unwrap();
        let archive = holder.path().join("evil.zip");
        let output = File::create(&archive).unwrap();
        let mut writer = ZipWriter::new(output);
        let options = SimpleFileOptions::default();
        writer.start_file("../evil.md", options).unwrap();
        writer.write_all(b"evil").unwrap();
        let manifest = BackupManifest {
            format: "md-notes-backup".into(),
            format_version: FORMAT_VERSION,
            app_version: "test".into(),
            created_at: Utc::now().to_rfc3339(),
            vault_name: "vault".into(),
            encrypted: false,
            files: vec![BackupManifestEntry {
                path: "../evil.md".into(),
                size: 4,
                sha256: sha256_hex(b"evil"),
            }],
        };
        writer.start_file(MANIFEST_NAME, options).unwrap();
        writer
            .write_all(&serde_json::to_vec(&manifest).unwrap())
            .unwrap();
        writer.finish().unwrap();

        let parent = tempfile::tempdir().unwrap();
        let error = restore_vault_backup(&archive, parent.path()).unwrap_err();
        assert!(error.to_string().contains("unsafe archive path"));
        assert!(fs::read_dir(parent.path()).unwrap().next().is_none());
    }

    #[test]
    fn encrypted_backup_keeps_ciphertext_and_never_adds_secret_key() {
        let vault = tempfile::tempdir().unwrap();
        fs::create_dir_all(vault.path().join(".mdnotes")).unwrap();
        fs::write(
            vault.path().join(".mdnotes/encryption.json"),
            b"{\"format\":\"age\",\"version\":1,\"vaultId\":\"v\",\"recipients\":[\"age1public\"],\"names\":\"random\"}\n",
        )
        .unwrap();
        fs::write(vault.path().join("cipher.age"), b"AGE-ENCRYPTED-CONTENT").unwrap();

        let holder = tempfile::tempdir().unwrap();
        let archive = holder.path().join("encrypted.zip");
        let summary = create_vault_backup(vault.path(), &archive).unwrap();
        assert!(summary.encrypted);

        let mut zip = ZipArchive::new(File::open(&archive).unwrap()).unwrap();
        let mut cipher = Vec::new();
        zip.by_name("cipher.age")
            .unwrap()
            .read_to_end(&mut cipher)
            .unwrap();
        assert_eq!(cipher, b"AGE-ENCRYPTED-CONTENT");
        for index in 0..zip.len() {
            let mut file = zip.by_index(index).unwrap();
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes).unwrap();
            assert!(!String::from_utf8_lossy(&bytes).contains("AGE-SECRET-KEY-"));
        }
    }
}
