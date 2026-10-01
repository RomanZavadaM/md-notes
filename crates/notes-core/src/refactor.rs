//! Vault-wide edits that keep links consistent.

use std::collections::HashMap;

use serde::Serialize;

use crate::error::Result;
use crate::index::Index;
use crate::markdown::rewrite_wikilinks;
use crate::paths;
use crate::vault::Vault;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameOutcome {
    /// New path of the renamed entry.
    pub path: String,
    /// Notes whose links were rewritten (at their new paths).
    pub updated: Vec<String>,
}

/// Renames or moves a note or folder and rewrites every `[[link]]` that
/// pointed to a moved note, including links in properties.
///
/// Link style is preserved: `[[Name]]` stays a name and `[[dir/Name]]`
/// stays a path. A name that would become ambiguous is written as a path.
pub fn rename_with_links(
    vault: &Vault,
    index: &mut Index,
    from: &str,
    to: &str,
) -> Result<RenameOutcome> {
    let from = paths::normalize(from)?;
    let to = paths::normalize(to)?;
    index.sync(vault)?;
    let moved = moved_notes(vault, &from, &to)?;

    // Plan all edits while the index still describes the old layout.
    let mut sources: Vec<String> = Vec::new();
    for old in moved.keys() {
        for backlink in index.backlinks(old)? {
            if !sources.contains(&backlink.path) {
                sources.push(backlink.path);
            }
        }
    }
    sources.sort();
    let mut planned = Vec::new();
    for source in &sources {
        let note = vault.read_note(source)?;
        let content = rewrite_wikilinks(&note.content, |link| {
            let old = index.resolve(&link.target)?;
            let new = moved.get(&old)?;
            let mut text = link_text(&link.target, new);
            let clashes = index
                .resolve(&text)
                .is_some_and(|other| !moved.contains_key(&other));
            if clashes {
                text = strip_note_ext(new);
            }
            (text != link.target).then_some(text)
        });
        if content != note.content {
            planned.push((source.clone(), content));
        }
    }

    let path = vault.rename(&from, &to)?;
    let mut updated = Vec::new();
    for (source, content) in planned {
        let current = moved.get(&source).cloned().unwrap_or(source);
        vault.write_note(&current, &content)?;
        updated.push(current);
    }
    index.sync(vault)?;
    Ok(RenameOutcome { path, updated })
}

/// Old path -> new path for every note affected by moving `from` to `to`.
fn moved_notes(vault: &Vault, from: &str, to: &str) -> Result<HashMap<String, String>> {
    let mut moved = HashMap::new();
    if paths::is_note(from) {
        moved.insert(from.to_string(), to.to_string());
        return Ok(moved);
    }
    let prefix = format!("{from}/");
    for path in vault.note_paths()? {
        if let Some(rest) = path.strip_prefix(&prefix) {
            let new = paths::join(to, rest);
            moved.insert(path, new);
        }
    }
    Ok(moved)
}

/// Link text for `new_path` in the style of `old_target`.
fn link_text(old_target: &str, new_path: &str) -> String {
    let base = if old_target.contains('/') {
        new_path
    } else {
        paths::file_name(new_path)
    };
    if paths::is_note(old_target) {
        base.to_string()
    } else {
        strip_note_ext(base)
    }
}

fn strip_note_ext(path: &str) -> String {
    let lower = path.to_lowercase();
    for ext in [".md", ".markdown"] {
        if lower.ends_with(ext) {
            return path[..path.len() - ext.len()].to_string();
        }
    }
    path.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (tempfile::TempDir, Vault, Index) {
        let dir = tempfile::tempdir().unwrap();
        let vault = Vault::init(dir.path(), None).unwrap();
        let index = Index::open_in_memory().unwrap();
        (dir, vault, index)
    }

    fn read(vault: &Vault, path: &str) -> String {
        vault.read_note(path).unwrap().content
    }

    #[test]
    fn renaming_a_note_updates_links() {
        let (_dir, vault, mut index) = setup();
        vault.write_note("folder/Target.md", "# T").unwrap();
        let a = "[[Target]], [[target#H|alias]], `[[Target]]`, [[Other]]";
        vault.write_note("a.md", a).unwrap();
        let b = "---\nproject: \"[[Target]]\"\n---\nbody";
        vault.write_note("b.md", b).unwrap();
        vault.write_note("c.md", "[[folder/Target]]").unwrap();
        vault.write_note("d.md", "[[Target.md]]").unwrap();

        let outcome =
            rename_with_links(&vault, &mut index, "folder/Target.md", "other/New.md").unwrap();
        assert_eq!(outcome.path, "other/New.md");
        assert_eq!(outcome.updated, vec!["a.md", "b.md", "c.md", "d.md"]);
        assert_eq!(
            read(&vault, "a.md"),
            "[[New]], [[New#H|alias]], `[[Target]]`, [[Other]]"
        );
        assert_eq!(read(&vault, "b.md"), "---\nproject: \"[[New]]\"\n---\nbody");
        assert_eq!(read(&vault, "c.md"), "[[other/New]]");
        assert_eq!(read(&vault, "d.md"), "[[New.md]]");
        assert_eq!(index.backlinks("other/New.md").unwrap().len(), 4);
    }

    #[test]
    fn moving_a_folder_updates_path_links_only() {
        let (_dir, vault, mut index) = setup();
        vault.write_note("dir/x.md", "[[dir/y]]").unwrap();
        vault.write_note("dir/y.md", "y").unwrap();
        vault.write_note("a.md", "[[dir/x]] [[x]]").unwrap();

        let outcome = rename_with_links(&vault, &mut index, "dir", "moved").unwrap();
        assert_eq!(outcome.updated, vec!["a.md", "moved/x.md"]);
        assert_eq!(read(&vault, "a.md"), "[[moved/x]] [[x]]");
        assert_eq!(read(&vault, "moved/x.md"), "[[moved/y]]");
    }

    #[test]
    fn ambiguous_names_become_paths() {
        let (_dir, vault, mut index) = setup();
        vault.write_note("A/Target.md", "a").unwrap();
        vault.write_note("B/Other.md", "b").unwrap();
        vault.write_note("n.md", "[[Other]] [[Target]]").unwrap();

        rename_with_links(&vault, &mut index, "B/Other.md", "B/Target.md").unwrap();
        assert_eq!(read(&vault, "n.md"), "[[B/Target]] [[Target]]");
        assert_eq!(index.resolve("Target").as_deref(), Some("A/Target.md"));
    }

    #[test]
    fn renaming_without_backlinks_touches_nothing_else() {
        let (_dir, vault, mut index) = setup();
        vault.write_note("lonely.md", "x").unwrap();
        vault.write_note("other.md", "[[something]]").unwrap();
        let outcome = rename_with_links(&vault, &mut index, "lonely.md", "Alone.md").unwrap();
        assert_eq!(outcome.path, "Alone.md");
        assert!(outcome.updated.is_empty());
        assert_eq!(read(&vault, "other.md"), "[[something]]");
    }
}
