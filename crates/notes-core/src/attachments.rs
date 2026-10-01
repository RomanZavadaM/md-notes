use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::error::{io_err, Error, Result};
use crate::paths;
use crate::vault::Vault;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
}

impl Vault {
    /// Copies a file into `<attachments_dir>/YYYY/MM/` and returns its
    /// vault-relative path. Existing files are never overwritten silently.
    pub fn import_attachment(&self, source: impl AsRef<Path>) -> Result<AttachmentInfo> {
        let source = source.as_ref();
        if !source.is_file() {
            return Err(Error::NotFound(source.display().to_string()));
        }

        let original_name = source
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| Error::InvalidPath(source.display().to_string()))?;
        let clean_name = paths::sanitize_file_name(original_name);
        if clean_name.is_empty() {
            return Err(Error::InvalidPath(original_name.to_string()));
        }

        let now = chrono::Local::now();
        let dir = format!(
            "{}/{}/{}",
            paths::normalize(&self.config().attachments_dir)?,
            now.format("%Y"),
            now.format("%m")
        );
        let dir_path = paths::resolve(self.root(), &dir)?;
        fs::create_dir_all(&dir_path).map_err(io_err(&dir_path))?;

        let rel = unique_attachment_path(self.root(), &dir, &clean_name);
        let target = paths::resolve(self.root(), &rel)?;
        copy_atomic(source, &target)?;
        attachment_info(&rel, &target)
    }

    /// Lists all files under the configured attachments directory.
    pub fn attachments(&self) -> Result<Vec<AttachmentInfo>> {
        let base = paths::normalize(&self.config().attachments_dir)?;
        let root = paths::resolve(self.root(), &base)?;
        if !root.exists() {
            return Ok(Vec::new());
        }
        let mut out = Vec::new();
        collect_attachments(self.root(), &root, &mut out)?;
        out.sort_by(|a, b| a.path.to_lowercase().cmp(&b.path.to_lowercase()));
        Ok(out)
    }

    /// Returns notes whose raw Markdown contains a reference to an attachment.
    /// Both literal and URL-encoded vault-relative paths are recognized.
    pub fn attachment_used_by(&self, attachment_path: &str) -> Result<Vec<String>> {
        let attachment_path = paths::normalize(attachment_path)?;
        let encoded_path = markdown_url_path(&attachment_path);
        let mut used_by = Vec::new();
        for note_path in self.note_paths()? {
            let note = self.read_note(&note_path)?;
            if note.content.contains(&attachment_path) || note.content.contains(&encoded_path) {
                used_by.push(note_path);
            }
        }
        Ok(used_by)
    }

    /// Attachments that are not referenced by any Markdown note.
    pub fn orphan_attachments(&self) -> Result<Vec<AttachmentInfo>> {
        let mut out = Vec::new();
        for attachment in self.attachments()? {
            if self.attachment_used_by(&attachment.path)?.is_empty() {
                out.push(attachment);
            }
        }
        Ok(out)
    }
}

fn attachment_info(rel: &str, path: &Path) -> Result<AttachmentInfo> {
    let meta = fs::metadata(path).map_err(io_err(path))?;
    Ok(AttachmentInfo {
        name: paths::file_name(rel).to_string(),
        path: rel.to_string(),
        size: meta.len(),
    })
}

fn collect_attachments(vault_root: &Path, dir: &Path, out: &mut Vec<AttachmentInfo>) -> Result<()> {
    for entry in fs::read_dir(dir).map_err(io_err(dir))? {
        let entry = entry.map_err(io_err(dir))?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') {
            continue;
        }
        let file_type = entry.file_type().map_err(io_err(&path))?;
        if file_type.is_dir() {
            collect_attachments(vault_root, &path, out)?;
        } else if file_type.is_file() {
            let rel = path
                .strip_prefix(vault_root)
                .map_err(|_| Error::InvalidPath(path.display().to_string()))?
                .to_string_lossy()
                .replace('\\', "/");
            out.push(attachment_info(&rel, &path)?);
        }
    }
    Ok(())
}

fn markdown_url_path(path: &str) -> String {
    let mut out = String::with_capacity(path.len());
    for byte in path.as_bytes() {
        match *byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' | b'/' => {
                out.push(*byte as char)
            }
            other => {
                write!(&mut out, "%{other:02X}").expect("writing to String cannot fail");
            }
        }
    }
    out
}

fn unique_attachment_path(root: &Path, dir: &str, file_name: &str) -> String {
    let source = Path::new(file_name);
    let stem = source
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(file_name);
    let ext = source
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| format!(".{s}"))
        .unwrap_or_default();

    for n in 0u32.. {
        let name = if n == 0 {
            format!("{stem}{ext}")
        } else {
            format!("{stem} {n}{ext}")
        };
        let rel = paths::join(dir, &name);
        if !root.join(&rel).exists() {
            return rel;
        }
    }
    unreachable!()
}

fn copy_atomic(source: &Path, target: &Path) -> Result<()> {
    let mut tmp_name = target.as_os_str().to_owned();
    tmp_name.push(".mdnotes-tmp");
    let tmp = PathBuf::from(tmp_name);
    fs::copy(source, &tmp).map_err(io_err(&tmp))?;
    fs::rename(&tmp, target).map_err(io_err(target))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vault() -> (tempfile::TempDir, Vault) {
        let dir = tempfile::tempdir().unwrap();
        let vault = Vault::init(dir.path(), Some("Test".into())).unwrap();
        (dir, vault)
    }

    #[test]
    fn imports_without_overwriting_and_lists_attachments() {
        let (dir, vault) = vault();
        let source = dir.path().join("photo.png");
        fs::write(&source, b"one").unwrap();

        let first = vault.import_attachment(&source).unwrap();
        let second = vault.import_attachment(&source).unwrap();

        assert!(first.path.starts_with("attachments/"));
        assert_ne!(first.path, second.path);
        assert_eq!(vault.attachments().unwrap().len(), 2);
    }

    #[test]
    fn finds_usage_and_orphans() {
        let (dir, vault) = vault();
        let source = dir.path().join("scan.pdf");
        fs::write(&source, b"pdf").unwrap();
        let attachment = vault.import_attachment(&source).unwrap();

        vault
            .write_note("linked.md", &format!("[scan]({})", attachment.path))
            .unwrap();
        assert_eq!(
            vault.attachment_used_by(&attachment.path).unwrap(),
            vec!["linked.md"]
        );
        assert!(vault.orphan_attachments().unwrap().is_empty());

        vault.write_note("linked.md", "no attachment here").unwrap();
        assert_eq!(vault.orphan_attachments().unwrap().len(), 1);
    }

    #[test]
    fn finds_url_encoded_unicode_attachment_usage() {
        let (dir, vault) = vault();
        let source = dir.path().join("схема руху 1.png");
        fs::write(&source, b"png").unwrap();
        let attachment = vault.import_attachment(&source).unwrap();
        let encoded = markdown_url_path(&attachment.path);

        vault
            .write_note("linked.md", &format!("![scheme](</{encoded}>)"))
            .unwrap();

        assert_eq!(
            vault.attachment_used_by(&attachment.path).unwrap(),
            vec!["linked.md"]
        );
    }
}
