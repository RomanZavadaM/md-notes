use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::sync::atomic::AtomicBool;

use chrono::Utc;
use gix::bstr::{BString, ByteSlice};
use serde::Serialize;

use crate::error::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitRepositoryInfo {
    pub workdir: String,
    pub head: Option<String>,
}

/// Validate a remote before it reaches the Git transport.
///
/// v0.3 starts with HTTPS only. Credentials embedded in the URL are forbidden:
/// authentication will be injected from the platform secret store in a later
/// slice and must never be persisted in vault files or logs.
pub fn validate_git_remote(remote: &str) -> Result<()> {
    let rest = remote
        .strip_prefix("https://")
        .ok_or_else(|| Error::Git("only HTTPS Git remotes are supported".into()))?;
    let authority = rest.split('/').next().unwrap_or_default();
    if authority.is_empty() {
        return Err(Error::Git("Git remote host is missing".into()));
    }
    if authority.contains('@') {
        return Err(Error::Git(
            "credentials must not be embedded in the Git remote URL".into(),
        ));
    }
    if remote.chars().any(char::is_whitespace) || remote.contains('?') || remote.contains('#') {
        return Err(Error::Git(
            "Git remote URL must not contain whitespace, query parameters or fragments".into(),
        ));
    }
    Ok(())
}

pub fn open_git_repository(path: impl AsRef<Path>) -> Result<GitRepositoryInfo> {
    let repo = open_isolated(path.as_ref())?;
    repository_info(&repo)
}

/// Return whether the repository has tracked or untracked worktree changes.
///
/// This deliberately uses isolated repository config so system/user Git config
/// cannot silently alter status behavior or load credential helpers.
pub fn git_has_changes(path: impl AsRef<Path>) -> Result<bool> {
    let repo = open_isolated(path.as_ref())?;
    let mut items = status_items(&repo)?;

    match items.next() {
        Some(Ok(_)) => Ok(true),
        Some(Err(error)) => Err(git_err(error)),
        None => Ok(false),
    }
}

/// Commit all current worktree changes with an application-owned identity.
///
/// Returns the new commit id, or `None` when there is nothing to commit.
/// Existing staged/index-only changes are rejected instead of being silently
/// included or overwritten. MD Notes never depends on user/system Git identity.
pub fn commit_git_worktree(path: impl AsRef<Path>, message: &str) -> Result<Option<String>> {
    if message.trim().is_empty() {
        return Err(Error::Git("Git commit message must not be empty".into()));
    }

    let repo = open_isolated(path.as_ref())?;
    let changes = collect_worktree_changes(&repo)?;
    if changes.is_empty() {
        return Ok(None);
    }

    let base_tree = repo.head_tree_id_or_empty().map_err(git_err)?.detach();
    let mut editor = repo.edit_tree(base_tree).map_err(git_err)?;

    for rela_path in changes {
        let worktree_path = repo
            .workdir_path(rela_path.as_bstr())
            .ok_or_else(|| Error::Git("Git repository has no usable worktree".into()))?;

        match fs::symlink_metadata(&worktree_path) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(Error::Git(format!(
                    "symbolic links are not supported in MD Notes Git sync: {}",
                    rela_path
                )));
            }
            Ok(metadata) if metadata.is_file() => {
                let bytes = fs::read(&worktree_path)
                    .map_err(|error| Error::Git(format!("cannot read {}: {error}", rela_path)))?;
                let blob_id = repo.write_blob(bytes).map_err(git_err)?.detach();
                let kind = editor
                    .get(rela_path.as_bstr())
                    .map(|entry| entry.kind())
                    .filter(|kind| {
                        matches!(
                            kind,
                            gix::object::tree::EntryKind::Blob
                                | gix::object::tree::EntryKind::BlobExecutable
                        )
                    })
                    .unwrap_or(gix::object::tree::EntryKind::Blob);
                editor
                    .upsert(rela_path.as_bstr(), kind, blob_id)
                    .map_err(git_err)?;
            }
            Ok(metadata) if metadata.is_dir() => {
                return Err(Error::Git(format!(
                    "unexpected directory status entry: {}",
                    rela_path
                )));
            }
            Ok(_) => {
                return Err(Error::Git(format!(
                    "unsupported filesystem entry in Git sync: {}",
                    rela_path
                )));
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                editor.remove_leaf(rela_path.as_bstr()).map_err(git_err)?;
            }
            Err(error) => {
                return Err(Error::Git(format!("cannot inspect {}: {error}", rela_path)));
            }
        }
    }

    let tree_id = editor.write().map_err(git_err)?.detach();
    let parent = repo.head().map_err(git_err)?.id().map(|id| id.detach());
    let timestamp = format!("{} +0000", Utc::now().timestamp());
    let signature = gix::actor::SignatureRef {
        name: "MD Notes".into(),
        email: "md-notes@local".into(),
        time: timestamp.as_str(),
    };
    let commit_id = repo
        .commit_as(
            signature,
            signature,
            "HEAD",
            message.trim(),
            tree_id,
            parent,
        )
        .map_err(git_err)?;

    let mut index = repo.index_from_tree(&tree_id).map_err(git_err)?;
    index
        .write(gix::index::write::Options::default())
        .map_err(git_err)?;

    Ok(Some(commit_id.to_string()))
}

/// Fetch one configured public HTTPS remote without consulting credential helpers.
///
/// `remote_name == None` follows gix/Git remote selection rules (typically
/// `origin`). This operation only fetches objects/remote refs. It does not merge,
/// reset, checkout or otherwise modify the current worktree.
pub fn fetch_git_remote_public(path: impl AsRef<Path>, remote_name: Option<&str>) -> Result<()> {
    let repo = open_isolated(path.as_ref())?;
    let remote_name = remote_name.map(|name| name.as_bytes().as_bstr());
    let remote = repo.find_fetch_remote(remote_name).map_err(git_err)?;
    let url = remote
        .url(gix::remote::Direction::Fetch)
        .ok_or_else(|| Error::Git("Git fetch remote has no URL".into()))?;
    let raw_url = url.to_bstring();
    let raw_url = raw_url
        .to_str()
        .map_err(|_| Error::Git("Git remote URL must be valid UTF-8".into()))?;
    validate_git_remote(raw_url)?;

    let connection = remote
        .connect(gix::remote::Direction::Fetch)
        .map_err(git_err)?
        .with_credentials(|_action| Ok(None));
    let prepare = connection
        .prepare_fetch(
            gix::progress::Discard,
            gix::remote::ref_map::Options::default(),
        )
        .map_err(git_err)?;
    let interrupt = AtomicBool::new(false);
    prepare
        .receive(gix::progress::Discard, &interrupt)
        .map_err(git_err)?;
    Ok(())
}

/// Clone a public HTTPS repository and check out its main worktree.
///
/// Authentication is deliberately not accepted here. The isolated open options
/// prevent system/user Git credential helpers from being consulted implicitly.
/// A later credential boundary will provide secrets at connection time without
/// storing them in the vault or remote URL.
pub fn clone_git_repository(
    remote: &str,
    destination: impl AsRef<Path>,
) -> Result<GitRepositoryInfo> {
    validate_git_remote(remote)?;

    let mut prepare = gix::clone::PrepareFetch::new(
        remote,
        destination.as_ref(),
        gix::create::Kind::WithWorktree,
        gix::create::Options::default(),
        gix::open::Options::isolated(),
    )
    .map_err(git_err)?;
    let interrupt = AtomicBool::new(false);
    let (mut checkout, _) = prepare
        .fetch_then_checkout(gix::progress::Discard, &interrupt)
        .map_err(git_err)?;
    let (repo, _) = checkout
        .main_worktree(gix::progress::Discard, &interrupt)
        .map_err(git_err)?;
    repository_info(&repo)
}

fn status_items(repo: &gix::Repository) -> Result<gix::status::Iter> {
    repo.status(gix::progress::Discard)
        .map_err(git_err)?
        .untracked_files(gix::status::UntrackedFiles::Files)
        .index_worktree_rewrites(None)
        .into_iter(std::iter::empty::<BString>())
        .map_err(git_err)
}

fn collect_worktree_changes(repo: &gix::Repository) -> Result<BTreeSet<BString>> {
    let mut paths = BTreeSet::new();
    for item in status_items(repo)? {
        match item.map_err(git_err)? {
            gix::status::Item::TreeIndex(_) => {
                return Err(Error::Git(
                    "repository contains staged/index-only changes; commit or reset them before MD Notes sync"
                        .into(),
                ));
            }
            gix::status::Item::IndexWorktree(item) => {
                paths.insert(item.rela_path().to_owned());
            }
        }
    }
    Ok(paths)
}

fn open_isolated(path: &Path) -> Result<gix::Repository> {
    gix::open_opts(path, gix::open::Options::isolated()).map_err(git_err)
}

fn repository_info(repo: &gix::Repository) -> Result<GitRepositoryInfo> {
    let workdir = repo
        .workdir()
        .ok_or_else(|| Error::Git("bare Git repositories are not supported as vaults".into()))?;
    let head = repo.head().map_err(git_err)?.id().map(|id| id.to_string());
    Ok(GitRepositoryInfo {
        workdir: workdir.display().to_string(),
        head,
    })
}

fn git_err(error: impl std::fmt::Display) -> Error {
    Error::Git(error.to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn remote_validation_requires_https_without_embedded_credentials() {
        assert!(validate_git_remote("https://github.com/example/repo.git").is_ok());
        assert!(validate_git_remote("http://github.com/example/repo.git").is_err());
        assert!(validate_git_remote("ssh://git@github.com/example/repo.git").is_err());
        assert!(validate_git_remote("https://token@github.com/example/repo.git").is_err());
        assert!(validate_git_remote("https://github.com/example/my repo.git").is_err());
        assert!(validate_git_remote("https://github.com/example/repo.git?token=secret").is_err());
        assert!(validate_git_remote("https://github.com/example/repo.git#secret").is_err());
    }

    #[test]
    fn opens_isolated_local_repository() {
        let dir = tempfile::tempdir().unwrap();
        gix::init(dir.path()).unwrap();

        let info = open_git_repository(dir.path()).unwrap();
        assert_eq!(Path::new(&info.workdir), dir.path());
        assert!(info.head.is_none());
    }

    #[test]
    fn detects_untracked_changes() {
        let dir = tempfile::tempdir().unwrap();
        gix::init(dir.path()).unwrap();
        assert!(!git_has_changes(dir.path()).unwrap());

        fs::write(dir.path().join("note.md"), "# Note\n").unwrap();
        assert!(git_has_changes(dir.path()).unwrap());
    }

    #[test]
    fn commits_initial_modify_delete_and_noop_worktree_states() {
        let dir = tempfile::tempdir().unwrap();
        gix::init(dir.path()).unwrap();

        fs::create_dir_all(dir.path().join("Notes")).unwrap();
        fs::write(dir.path().join("Notes/Idea.md"), "# One\n").unwrap();
        let first = commit_git_worktree(dir.path(), "Initial vault").unwrap();
        assert!(first.is_some());
        assert!(!git_has_changes(dir.path()).unwrap());

        fs::write(dir.path().join("Notes/Idea.md"), "# Two\n").unwrap();
        let second = commit_git_worktree(dir.path(), "Update idea").unwrap();
        assert!(second.is_some());
        assert_ne!(first, second);
        assert!(!git_has_changes(dir.path()).unwrap());

        fs::remove_file(dir.path().join("Notes/Idea.md")).unwrap();
        let third = commit_git_worktree(dir.path(), "Remove idea").unwrap();
        assert!(third.is_some());
        assert!(!git_has_changes(dir.path()).unwrap());

        assert_eq!(commit_git_worktree(dir.path(), "No changes").unwrap(), None);
    }

    #[test]
    fn public_fetch_requires_a_configured_remote() {
        let dir = tempfile::tempdir().unwrap();
        gix::init(dir.path()).unwrap();
        assert!(matches!(
            fetch_git_remote_public(dir.path(), None),
            Err(Error::Git(_))
        ));
    }

    #[test]
    fn commit_rejects_empty_message() {
        let dir = tempfile::tempdir().unwrap();
        gix::init(dir.path()).unwrap();
        fs::write(dir.path().join("note.md"), "# Note\n").unwrap();
        assert!(matches!(
            commit_git_worktree(dir.path(), "   "),
            Err(Error::Git(_))
        ));
    }

    #[test]
    fn open_rejects_non_repository() {
        let dir = tempfile::tempdir().unwrap();
        assert!(matches!(
            open_git_repository(dir.path()),
            Err(Error::Git(_))
        ));
    }
}
