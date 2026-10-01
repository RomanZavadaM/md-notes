//! Watches the vault folder for changes made outside the app (another
//! editor, Git, sync clients) and reports them as vault-relative paths.

use std::path::Path;
use std::time::Duration;

use notify_debouncer_mini::notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{new_debouncer, DebounceEventResult, Debouncer};

/// Keeps the watcher alive; dropping it stops watching.
pub struct VaultWatcher {
    _debouncer: Debouncer<RecommendedWatcher>,
}

pub fn watch(
    root: &Path,
    on_change: impl Fn(Vec<String>) + Send + 'static,
) -> notify_debouncer_mini::notify::Result<VaultWatcher> {
    let base = root.to_path_buf();
    let canonical = root.canonicalize().unwrap_or_else(|_| base.clone());
    let handler = move |result: DebounceEventResult| {
        let Ok(events) = result else {
            return;
        };
        let mut changed: Vec<String> = events
            .iter()
            .filter_map(|e| relative(&base, &e.path).or_else(|| relative(&canonical, &e.path)))
            .collect();
        changed.sort();
        changed.dedup();
        if !changed.is_empty() {
            on_change(changed);
        }
    };
    let mut debouncer = new_debouncer(Duration::from_millis(400), handler)?;
    debouncer.watcher().watch(root, RecursiveMode::Recursive)?;
    Ok(VaultWatcher {
        _debouncer: debouncer,
    })
}

/// Vault-relative `/`-separated path, or `None` for hidden and temporary
/// files (`.mdnotes`, `.git`, atomic-write temp files).
fn relative(root: &Path, path: &Path) -> Option<String> {
    let rel = path.strip_prefix(root).ok()?;
    let parts: Vec<String> = rel
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect();
    let last = parts.last()?;
    if parts.iter().any(|p| p.starts_with('.')) || last.ends_with(".mdnotes-tmp") {
        return None;
    }
    Some(parts.join("/"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn relative_paths_skip_hidden_and_temp_files() {
        let root = PathBuf::from("vault");
        let rel = |p: &str| relative(&root, &root.join(p));
        assert_eq!(rel("a/b.md").as_deref(), Some("a/b.md"));
        assert_eq!(rel(".mdnotes/cache/index.db"), None);
        assert_eq!(rel(".git/HEAD"), None);
        assert_eq!(rel("a.md.mdnotes-tmp"), None);
        assert_eq!(relative(&root, Path::new("other/x.md")), None);
    }
}
