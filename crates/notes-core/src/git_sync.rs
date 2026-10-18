use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

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

/// Safe pull-planning result. This type never mutates refs, index or worktree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GitPullRelation {
    UpToDate,
    FastForward,
    LocalAhead,
    Diverged,
    Unrelated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitPullPreflight {
    pub relation: GitPullRelation,
    pub worktree_clean: bool,
    pub can_fast_forward: bool,
}

/// Classify local and fetched remote heads without changing repository state.
pub fn classify_git_pull_relation(
    local_head: &str,
    remote_head: &str,
    merge_base: Option<&str>,
) -> GitPullRelation {
    if local_head == remote_head {
        return GitPullRelation::UpToDate;
    }
    match merge_base {
        Some(base) if base == local_head => GitPullRelation::FastForward,
        Some(base) if base == remote_head => GitPullRelation::LocalAhead,
        Some(_) => GitPullRelation::Diverged,
        None => GitPullRelation::Unrelated,
    }
}

/// Combine repository relation planning with the clean-worktree safety gate.
pub fn plan_git_pull_preflight(
    path: impl AsRef<Path>,
    remote_revision: &str,
) -> Result<GitPullPreflight> {
    let path = path.as_ref();
    let worktree_clean = !git_has_changes(path)?;
    let relation = plan_git_pull(path, remote_revision)?;
    Ok(GitPullPreflight {
        relation,
        worktree_clean,
        can_fast_forward: worktree_clean && relation == GitPullRelation::FastForward,
    })
}

/// Plan a pull against an already-fetched revision without mutating repository state.
///
/// The caller is responsible for fetching first. `remote_revision` is typically
/// a remote-tracking ref such as `refs/remotes/origin/main`.
pub fn plan_git_pull(path: impl AsRef<Path>, remote_revision: &str) -> Result<GitPullRelation> {
    if remote_revision.trim().is_empty() {
        return Err(Error::Git("Git remote revision must not be empty".into()));
    }

    let repo = open_isolated(path.as_ref())?;
    let local = repo
        .head()
        .map_err(git_err)?
        .id()
        .ok_or_else(|| Error::Git("cannot plan pull from an unborn local HEAD".into()))?
        .detach();
    let remote = repo
        .rev_parse_single(remote_revision.as_bytes().as_bstr())
        .map_err(git_err)?
        .detach();
    let merge_bases = repo.merge_bases_many(local, &[remote]).map_err(git_err)?;
    let merge_base = merge_bases.first().map(|id| id.detach());

    let local_text = local.to_string();
    let remote_text = remote.to_string();
    let base_text = merge_base.map(|id| id.to_string());
    Ok(classify_git_pull_relation(
        &local_text,
        &remote_text,
        base_text.as_deref(),
    ))
}

/// Apply an already-planned clean fast-forward without merge commits.
///
/// The remote revision must already be fetched. This function refuses dirty,
/// detached, unborn, non-fast-forward, diverged and unrelated states.
pub fn apply_git_fast_forward(
    path: impl AsRef<Path>,
    remote_revision: &str,
) -> Result<GitRepositoryInfo> {
    let path = path.as_ref();
    let preflight = plan_git_pull_preflight(path, remote_revision)?;
    if !preflight.can_fast_forward {
        return Err(Error::Git(format!(
            "Git fast-forward is not safe: relation={:?}, worktree_clean={}",
            preflight.relation, preflight.worktree_clean
        )));
    }

    let repo = open_isolated(path)?;
    // Re-check immediately before mutation so stale planning cannot authorize an apply.
    if git_has_changes(path)? {
        return Err(Error::Git(
            "Git worktree changed after pull planning; fast-forward aborted".into(),
        ));
    }

    let local = repo.head_id().map_err(git_err)?.detach();
    let head_name = repo
        .head_name()
        .map_err(git_err)?
        .ok_or_else(|| Error::Git("detached HEAD cannot be fast-forwarded automatically".into()))?;
    let remote = repo
        .rev_parse_single(remote_revision.as_bytes().as_bstr())
        .map_err(git_err)?
        .detach();

    let local_text = local.to_string();
    let remote_text = remote.to_string();
    let merge_bases = repo.merge_bases_many(local, &[remote]).map_err(git_err)?;
    let merge_base = merge_bases.first().map(|id| id.to_string());
    if classify_git_pull_relation(&local_text, &remote_text, merge_base.as_deref())
        != GitPullRelation::FastForward
    {
        return Err(Error::Git(
            "Git relation changed after pull planning; fast-forward aborted".into(),
        ));
    }

    let old_tree = repo.head_tree_id().map_err(git_err)?.detach();
    let target_tree = repo
        .find_commit(remote)
        .map_err(git_err)?
        .tree_id()
        .map_err(git_err)?
        .detach();

    checkout_tree(&repo, target_tree, false)?;

    let update = repo.reference(
        head_name,
        remote,
        gix::refs::transaction::PreviousValue::MustExistAndMatch(gix::refs::Target::Object(local)),
        "pull: fast-forward",
    );
    if let Err(error) = update {
        let rollback = checkout_tree(&repo, old_tree, true);
        return match rollback {
            Ok(()) => Err(Error::Git(format!(
                "Git HEAD changed during fast-forward; worktree restored: {error}"
            ))),
            Err(rollback_error) => Err(Error::Git(format!(
                "Git HEAD changed during fast-forward ({error}); rollback also failed ({rollback_error})"
            ))),
        };
    }

    Ok(GitRepositoryInfo {
        workdir: repo
            .workdir()
            .ok_or_else(|| Error::Git("bare Git repositories are not supported as vaults".into()))?
            .display()
            .to_string(),
        head: Some(remote.to_string()),
    })
}

fn checkout_tree(
    repo: &gix::Repository,
    tree_id: gix::ObjectId,
    overwrite_existing: bool,
) -> Result<()> {
    let workdir = repo
        .workdir()
        .ok_or_else(|| Error::Git("bare Git repositories are not supported as vaults".into()))?;
    let mut index = repo.index_from_tree(&tree_id).map_err(git_err)?;
    let mut options = repo
        .checkout_options(gix::worktree::stack::state::attributes::Source::IdMapping)
        .map_err(git_err)?;
    options.overwrite_existing = overwrite_existing;
    options.destination_is_initially_empty = false;
    let interrupt = AtomicBool::new(false);
    let objects = repo.objects.clone().into_arc().map_err(git_err)?;
    gix::worktree::state::checkout(
        &mut index,
        workdir,
        objects,
        &gix::progress::Discard,
        &gix::progress::Discard,
        &interrupt,
        options,
    )
    .map_err(git_err)?;
    if interrupt.load(Ordering::Relaxed) {
        return Err(Error::Git("Git checkout was interrupted".into()));
    }
    index
        .write(gix::index::write::Options::default())
        .map_err(git_err)?;
    Ok(())
}

/// HTTPS credentials supplied by the application at call time.
///
/// Intentionally does not implement `Debug` or `Serialize` so tokens are harder
/// to leak into logs, diagnostics or persisted vault state by accident.
#[derive(Clone, PartialEq, Eq)]
pub struct GitHttpsCredentials {
    username: String,
    token: String,
}

impl GitHttpsCredentials {
    pub fn new(username: impl Into<String>, token: impl Into<String>) -> Result<Self> {
        let username = username.into();
        let token = token.into();
        if username.trim().is_empty() {
            return Err(Error::Git("Git HTTPS username must not be empty".into()));
        }
        if token.is_empty() {
            return Err(Error::Git("Git HTTPS token must not be empty".into()));
        }
        Ok(Self { username, token })
    }
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
    fetch_git_remote(path, remote_name, None)
}

/// Fetch one configured HTTPS remote using credentials supplied only in memory.
///
/// MD Notes does not invoke system Git credential helpers and does not persist
/// the supplied token in the remote URL, vault, repository config or credential
/// helper storage. `Store`/`Erase` callback actions are deliberately no-ops.
pub fn fetch_git_remote_authenticated(
    path: impl AsRef<Path>,
    remote_name: Option<&str>,
    credentials: &GitHttpsCredentials,
) -> Result<()> {
    fetch_git_remote(path, remote_name, Some(credentials))
}

fn fetch_git_remote(
    path: impl AsRef<Path>,
    remote_name: Option<&str>,
    credentials: Option<&GitHttpsCredentials>,
) -> Result<()> {
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
        .with_credentials(|action| credential_response(action, credentials));
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

fn credential_response(
    action: gix::credentials::helper::Action,
    credentials: Option<&GitHttpsCredentials>,
) -> gix::ExnResult<Option<gix::credentials::protocol::Outcome>> {
    match action {
        gix::credentials::helper::Action::Get(context) => match credentials {
            Some(credentials) => Ok(Some(gix::credentials::protocol::Outcome {
                identity: gix::sec::identity::Account {
                    username: credentials.username.clone(),
                    password: credentials.token.clone(),
                    oauth_refresh_token: None,
                },
                next: context.into(),
            })),
            None => Ok(None),
        },
        gix::credentials::helper::Action::Store(_) | gix::credentials::helper::Action::Erase(_) => {
            Ok(None)
        }
    }
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
    fn classifies_pull_relations_without_mutation() {
        assert_eq!(
            classify_git_pull_relation("a", "a", Some("a")),
            GitPullRelation::UpToDate
        );
        assert_eq!(
            classify_git_pull_relation("a", "b", Some("a")),
            GitPullRelation::FastForward
        );
        assert_eq!(
            classify_git_pull_relation("b", "a", Some("a")),
            GitPullRelation::LocalAhead
        );
        assert_eq!(
            classify_git_pull_relation("b", "c", Some("a")),
            GitPullRelation::Diverged
        );
        assert_eq!(
            classify_git_pull_relation("b", "c", None),
            GitPullRelation::Unrelated
        );
    }

    #[test]
    fn pull_preflight_blocks_apply_when_worktree_is_dirty() {
        let dir = tempfile::tempdir().unwrap();
        gix::init(dir.path()).unwrap();
        fs::write(dir.path().join("note.md"), "# One\n").unwrap();
        commit_git_worktree(dir.path(), "Initial").unwrap();

        let clean = plan_git_pull_preflight(dir.path(), "HEAD").unwrap();
        assert!(clean.worktree_clean);
        assert_eq!(clean.relation, GitPullRelation::UpToDate);
        assert!(!clean.can_fast_forward);

        fs::write(dir.path().join("draft.md"), "# Draft\n").unwrap();
        let dirty = plan_git_pull_preflight(dir.path(), "HEAD").unwrap();
        assert!(!dirty.worktree_clean);
        assert_eq!(dirty.relation, GitPullRelation::UpToDate);
        assert!(!dirty.can_fast_forward);
    }

    #[test]
    fn fast_forward_apply_updates_head_worktree_and_index() {
        let dir = tempfile::tempdir().unwrap();
        gix::init(dir.path()).unwrap();
        fs::write(dir.path().join("note.md"), "# One\n").unwrap();
        let first = commit_git_worktree(dir.path(), "Initial").unwrap().unwrap();

        fs::write(dir.path().join("note.md"), "# Two\n").unwrap();
        let second = commit_git_worktree(dir.path(), "Remote update")
            .unwrap()
            .unwrap();

        let repo = open_isolated(dir.path()).unwrap();
        let first_id = repo
            .rev_parse_single(first.as_bytes().as_bstr())
            .unwrap()
            .detach();
        let second_id = repo
            .rev_parse_single(second.as_bytes().as_bstr())
            .unwrap()
            .detach();
        let head_name = repo.head_name().unwrap().unwrap();

        repo.reference(
            "refs/remotes/origin/main",
            second_id,
            gix::refs::transaction::PreviousValue::MustNotExist,
            "test: remote",
        )
        .unwrap();
        repo.reference(
            head_name,
            first_id,
            gix::refs::transaction::PreviousValue::MustExistAndMatch(gix::refs::Target::Object(
                second_id,
            )),
            "test: rewind local",
        )
        .unwrap();

        fs::write(dir.path().join("note.md"), "# One\n").unwrap();
        let first_tree = repo
            .find_commit(first_id)
            .unwrap()
            .tree_id()
            .unwrap()
            .detach();
        let mut index = repo.index_from_tree(&first_tree).unwrap();
        index.write(gix::index::write::Options::default()).unwrap();

        assert!(!git_has_changes(dir.path()).unwrap());
        let preflight = plan_git_pull_preflight(dir.path(), "refs/remotes/origin/main").unwrap();
        assert_eq!(preflight.relation, GitPullRelation::FastForward);
        assert!(preflight.can_fast_forward);

        let info = apply_git_fast_forward(dir.path(), "refs/remotes/origin/main").unwrap();
        assert_eq!(info.head.as_deref(), Some(second.as_str()));
        assert_eq!(
            fs::read_to_string(dir.path().join("note.md")).unwrap(),
            "# Two\n"
        );
        assert!(!git_has_changes(dir.path()).unwrap());
        assert_eq!(
            plan_git_pull(dir.path(), "refs/remotes/origin/main").unwrap(),
            GitPullRelation::UpToDate
        );
    }

    #[test]
    fn fast_forward_apply_refuses_dirty_worktree() {
        let dir = tempfile::tempdir().unwrap();
        gix::init(dir.path()).unwrap();
        fs::write(dir.path().join("note.md"), "# One\n").unwrap();
        commit_git_worktree(dir.path(), "Initial").unwrap();
        fs::write(dir.path().join("draft.md"), "# Draft\n").unwrap();

        assert!(apply_git_fast_forward(dir.path(), "HEAD").is_err());
        assert_eq!(
            fs::read_to_string(dir.path().join("draft.md")).unwrap(),
            "# Draft\n"
        );
    }

    #[test]
    fn repository_pull_plan_is_read_only_for_equal_heads() {
        let dir = tempfile::tempdir().unwrap();
        gix::init(dir.path()).unwrap();
        fs::write(dir.path().join("note.md"), "# Note\n").unwrap();
        commit_git_worktree(dir.path(), "Initial").unwrap();

        let before = open_git_repository(dir.path()).unwrap().head;
        assert_eq!(
            plan_git_pull(dir.path(), "HEAD").unwrap(),
            GitPullRelation::UpToDate
        );
        let after = open_git_repository(dir.path()).unwrap().head;
        assert_eq!(before, after);
    }

    #[test]
    fn repository_pull_plan_rejects_missing_remote_revision() {
        let dir = tempfile::tempdir().unwrap();
        gix::init(dir.path()).unwrap();
        fs::write(dir.path().join("note.md"), "# Note\n").unwrap();
        commit_git_worktree(dir.path(), "Initial").unwrap();

        assert!(matches!(
            plan_git_pull(dir.path(), "refs/remotes/origin/main"),
            Err(Error::Git(_))
        ));
    }

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
    fn https_credentials_require_non_empty_values() {
        assert!(GitHttpsCredentials::new("roman", "token").is_ok());
        assert!(GitHttpsCredentials::new(" ", "token").is_err());
        assert!(GitHttpsCredentials::new("roman", "").is_err());
    }

    #[test]
    fn credential_callback_returns_only_in_memory_identity() {
        let credentials = GitHttpsCredentials::new("roman", "top-secret").unwrap();
        let action =
            gix::credentials::helper::Action::get_for_url("https://github.com/example/repo.git");
        let outcome = credential_response(action, Some(&credentials))
            .unwrap()
            .unwrap();
        assert_eq!(outcome.identity.username, "roman");
        assert_eq!(outcome.identity.password, "top-secret");

        let public_action =
            gix::credentials::helper::Action::get_for_url("https://github.com/example/repo.git");
        assert!(credential_response(public_action, None).unwrap().is_none());
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
