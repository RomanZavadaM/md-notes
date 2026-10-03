use std::path::Path;
use std::sync::atomic::AtomicBool;

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
    let repo = gix::open_opts(path.as_ref(), gix::open::Options::isolated()).map_err(git_err)?;
    repository_info(&repo)
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
    fn open_rejects_non_repository() {
        let dir = tempfile::tempdir().unwrap();
        assert!(matches!(
            open_git_repository(dir.path()),
            Err(Error::Git(_))
        ));
    }
}
