//! Vault-relative paths.
//!
//! Inside the core every path is relative to the vault root and uses `/` as
//! the separator on all platforms, e.g. `Projects/MD Notes.md`. The empty
//! string means the vault root itself.

use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

/// Normalizes a vault-relative path and rejects anything that could escape
/// the vault: absolute paths, drive prefixes and `..` components.
pub fn normalize(rel: &str) -> Result<String> {
    let unified = rel.replace('\\', "/");
    if unified.starts_with('/') || has_drive_prefix(&unified) {
        return Err(Error::InvalidPath(rel.to_string()));
    }
    let mut parts = Vec::new();
    for part in unified.split('/') {
        match part {
            "" | "." => continue,
            ".." => return Err(Error::InvalidPath(rel.to_string())),
            p => parts.push(p),
        }
    }
    Ok(parts.join("/"))
}

fn has_drive_prefix(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() >= 2 && b[0].is_ascii_alphabetic() && b[1] == b':'
}

/// Converts a vault-relative path to a filesystem path under `root`.
pub fn resolve(root: &Path, rel: &str) -> Result<PathBuf> {
    let norm = normalize(rel)?;
    let mut path = root.to_path_buf();
    for part in norm.split('/').filter(|p| !p.is_empty()) {
        path.push(part);
    }
    Ok(path)
}

/// Whether the path points to a Markdown note.
pub fn is_note(path: &str) -> bool {
    let lower = path.to_lowercase();
    lower.ends_with(".md") || lower.ends_with(".markdown")
}

/// Last component of the path.
pub fn file_name(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

/// File name without its extension: `a/Note.md` -> `Note`.
pub fn file_stem(path: &str) -> &str {
    let name = file_name(path);
    match name.rfind('.') {
        Some(i) if i > 0 => &name[..i],
        _ => name,
    }
}

/// Parent directory: `a/b/c.md` -> `a/b`, `c.md` -> ``.
pub fn parent(path: &str) -> &str {
    match path.rfind('/') {
        Some(i) => &path[..i],
        None => "",
    }
}

/// Joins a directory and a name.
pub fn join(dir: &str, name: &str) -> String {
    if dir.is_empty() {
        name.to_string()
    } else {
        format!("{dir}/{name}")
    }
}

const FORBIDDEN_CHARS: &[char] = &['/', '\\', ':', '*', '?', '"', '<', '>', '|'];

/// Removes characters that are not allowed in file names on any supported
/// platform, so a vault stays portable between Windows, macOS and mobile.
pub fn sanitize_file_name(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .filter(|c| !c.is_control() && !FORBIDDEN_CHARS.contains(c))
        .collect();
    cleaned
        .trim()
        .trim_start_matches('.')
        .trim_end_matches(['.', ' '])
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_separators_and_dots() {
        assert_eq!(normalize(r"a\b/./c.md").unwrap(), "a/b/c.md");
        assert_eq!(normalize("").unwrap(), "");
        assert_eq!(normalize("a//b/").unwrap(), "a/b");
    }

    #[test]
    fn rejects_escaping_paths() {
        assert!(normalize("../secret.md").is_err());
        assert!(normalize("a/../../b").is_err());
        assert!(normalize("/etc/passwd").is_err());
        assert!(normalize(r"C:\Windows").is_err());
    }

    #[test]
    fn splits_names() {
        assert_eq!(file_stem("a/Note.md"), "Note");
        assert_eq!(file_stem(".hidden"), ".hidden");
        assert_eq!(parent("a/b/c.md"), "a/b");
        assert_eq!(parent("c.md"), "");
        assert_eq!(join("", "x.md"), "x.md");
        assert_eq!(join("a", "x.md"), "a/x.md");
    }

    #[test]
    fn sanitizes_file_names() {
        assert_eq!(sanitize_file_name("  Що: нове?  "), "Що нове");
        assert_eq!(sanitize_file_name("../evil"), "evil");
        assert_eq!(sanitize_file_name("name. "), "name");
    }
}
