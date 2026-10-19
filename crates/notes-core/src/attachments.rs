use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use serde::Serialize;

use crate::error::{io_err, Error, Result};
use crate::paths;
use crate::storage::{StorageEntryKind, StorageProvider};
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
    /// The file is written through the vault's storage provider, so an
    /// encrypted vault stores the attachment encrypted.
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

        let provider = self.provider();
        let rel = unique_attachment_path(provider.as_ref(), &dir, &clean_name)?;
        let bytes = fs::read(source).map_err(io_err(source))?;
        provider.write(&rel, &bytes)?;
        attachment_info(provider.as_ref(), &rel)
    }

    /// Lists all files under the configured attachments directory.
    pub fn attachments(&self) -> Result<Vec<AttachmentInfo>> {
        let base = paths::normalize(&self.config().attachments_dir)?;
        let provider = self.provider();
        if provider.metadata(&base)?.is_none() {
            return Ok(Vec::new());
        }
        let mut out = Vec::new();
        collect_attachments(provider.as_ref(), &base, &mut out)?;
        out.sort_by_key(|a| a.path.to_lowercase());
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

fn attachment_info(provider: &dyn StorageProvider, rel: &str) -> Result<AttachmentInfo> {
    let entry = provider
        .metadata(rel)?
        .ok_or_else(|| Error::NotFound(rel.to_string()))?;
    Ok(AttachmentInfo {
        name: paths::file_name(rel).to_string(),
        path: rel.to_string(),
        size: entry.size,
    })
}

fn collect_attachments(
    provider: &dyn StorageProvider,
    dir: &str,
    out: &mut Vec<AttachmentInfo>,
) -> Result<()> {
    for entry in provider.list(dir)? {
        if paths::file_name(&entry.path).starts_with('.') {
            continue;
        }
        match entry.kind {
            StorageEntryKind::Dir => collect_attachments(provider, &entry.path, out)?,
            StorageEntryKind::File => out.push(attachment_info(provider, &entry.path)?),
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

fn unique_attachment_path(
    provider: &dyn StorageProvider,
    dir: &str,
    file_name: &str,
) -> Result<String> {
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
        if provider.metadata(&rel)?.is_none() {
            return Ok(rel);
        }
    }
    unreachable!()
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
