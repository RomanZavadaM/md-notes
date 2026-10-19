//! Repository status and setup for the Git panel (desktop only).
//!
//! Status and the pull relation use `gix` (read-only); creating a repository
//! and setting the remote use the system `git` through the same locked-down
//! runner as `git_push`. Only HTTPS remotes without credentials in the URL
//! are accepted.

use std::path::Path;

use serde::Serialize;

use crate::error::{Error, Result};
use crate::git_push::{capture, run};
use crate::git_sync::{plan_git_pull, validate_git_remote, GitPullRelation};

/// What the Git panel shows about a vault folder.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitStatus {
    /// The vault folder itself holds a Git repository.
    pub is_repo: bool,
    pub branch: Option<String>,
    pub remote_url: Option<String>,
    pub has_changes: bool,
    /// Local branch compared with the last fetched remote branch; `None`
    /// when nothing was fetched yet or the local branch has no commits.
    pub relation: Option<GitPullRelation>,
}

fn valid_remote_name(name: &str) -> bool {
    !name.is_empty()
        && !name.starts_with('-')
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
}

/// Reads the status of the repository in `path` against `remote_name`.
pub fn git_status(path: impl AsRef<Path>, remote_name: &str) -> Result<GitStatus> {
    let path = path.as_ref();
    if !valid_remote_name(remote_name) {
        return Err(Error::Git("Git remote name is not valid".into()));
    }
    if !path.join(".git").exists() {
        return Ok(GitStatus {
            is_repo: false,
            branch: None,
            remote_url: None,
            has_changes: false,
            relation: None,
        });
    }
    let branch = capture(path, &["symbolic-ref", "--short", "-q", "HEAD"], None)
        .ok()
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty());
    let remote_url = capture(path, &["remote", "get-url", "--", remote_name], None)
        .ok()
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty());
    let has_changes = capture(path, &["status", "--porcelain"], None)
        .map(|text| !text.trim().is_empty())
        .unwrap_or(false);
    let relation = match (&branch, &remote_url) {
        (Some(branch), Some(_)) => {
            let tracking = format!("refs/remotes/{remote_name}/{branch}");
            let fetched = capture(path, &["rev-parse", "--verify", "-q", &tracking], None).is_ok();
            if fetched {
                plan_git_pull(path, &tracking).ok()
            } else {
                None
            }
        }
        _ => None,
    };
    Ok(GitStatus {
        is_repo: true,
        branch,
        remote_url,
        has_changes,
        relation,
    })
}

/// Name of the remote-tracking reference of the current branch, for
/// planning a pull after a fetch.
pub fn tracking_reference(path: impl AsRef<Path>, remote_name: &str) -> Result<String> {
    let path = path.as_ref();
    if !valid_remote_name(remote_name) {
        return Err(Error::Git("Git remote name is not valid".into()));
    }
    let branch = capture(path, &["symbolic-ref", "--short", "-q", "HEAD"], None)
        .map_err(|_| Error::Git("pull needs a checked-out branch".into()))?;
    Ok(format!("refs/remotes/{remote_name}/{}", branch.trim()))
}

/// Creates a repository in the vault folder (branch `main`). Does nothing
/// to an existing repository.
pub fn git_init(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    if path.join(".git").exists() {
        return Ok(());
    }
    check(run(path, &["init"], None)?, "init")?;
    check(
        run(path, &["symbolic-ref", "HEAD", "refs/heads/main"], None)?,
        "symbolic-ref",
    )
}

/// Sets (or adds) a remote. Only HTTPS URLs without embedded credentials.
pub fn git_set_remote(path: impl AsRef<Path>, remote_name: &str, url: &str) -> Result<()> {
    let path = path.as_ref();
    if !valid_remote_name(remote_name) {
        return Err(Error::Git("Git remote name is not valid".into()));
    }
    let url = url.trim();
    validate_git_remote(url)?;
    let exists = capture(path, &["remote", "get-url", "--", remote_name], None).is_ok();
    let verb = if exists { "set-url" } else { "add" };
    check(
        run(path, &["remote", verb, "--", remote_name, url], None)?,
        "remote",
    )
}

fn check(output: std::process::Output, what: &str) -> Result<()> {
    if output.status.success() {
        Ok(())
    } else {
        Err(Error::Git(format!(
            "git {what} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::Command;

    use super::*;

    fn git_available() -> bool {
        Command::new("git")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn git(dir: &Path, args: &[&str]) -> String {
        let output = Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(["-c", "user.name=Test", "-c", "user.email=test@example.com"])
            .args(["-c", "commit.gpgsign=false", "-c", "core.autocrlf=false"])
            .args(args)
            .output()
            .expect("git runs");
        assert!(
            output.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    #[test]
    fn folder_without_repository_is_reported() {
        let dir = tempfile::tempdir().unwrap();
        let status = git_status(dir.path(), "origin").unwrap();
        assert!(!status.is_repo);
        assert_eq!(status.branch, None);
        assert!(git_status(dir.path(), "-x").is_err());
    }

    #[test]
    fn init_and_remote_setup() {
        if !git_available() {
            return;
        }
        let dir = tempfile::tempdir().unwrap();
        git_init(dir.path()).unwrap();
        git_init(dir.path()).unwrap();
        let status = git_status(dir.path(), "origin").unwrap();
        assert!(status.is_repo);
        assert_eq!(status.branch.as_deref(), Some("main"));
        assert_eq!(status.remote_url, None);
        assert_eq!(status.relation, None);

        git_set_remote(dir.path(), "origin", "https://github.com/user/repo.git").unwrap();
        git_set_remote(dir.path(), "origin", "https://github.com/user/other.git").unwrap();
        let status = git_status(dir.path(), "origin").unwrap();
        assert_eq!(
            status.remote_url.as_deref(),
            Some("https://github.com/user/other.git")
        );

        assert!(git_set_remote(dir.path(), "origin", "git@github.com:user/repo.git").is_err());
        assert!(git_set_remote(dir.path(), "origin", "https://user:tok@github.com/x.git").is_err());
        assert!(git_set_remote(dir.path(), "-x", "https://github.com/x.git").is_err());
    }

    #[test]
    fn changes_and_relation_to_the_fetched_remote() {
        if !git_available() {
            return;
        }
        let tmp = tempfile::tempdir().unwrap();
        let work = tmp.path().join("work");
        let bare = tmp.path().join("remote.git");
        fs::create_dir_all(&work).unwrap();
        fs::create_dir_all(&bare).unwrap();
        git(&bare, &["init", "--bare"]);
        git_init(&work).unwrap();
        git(&work, &["remote", "add", "origin", bare.to_str().unwrap()]);

        fs::write(work.join("a.md"), "# a").unwrap();
        assert!(git_status(&work, "origin").unwrap().has_changes);
        git(&work, &["add", "-A"]);
        git(&work, &["commit", "-m", "one"]);
        git(&work, &["push", "origin", "main"]);
        git(&work, &["fetch", "origin"]);
        let status = git_status(&work, "origin").unwrap();
        assert!(!status.has_changes);
        assert_eq!(status.relation, Some(GitPullRelation::UpToDate));
        assert_eq!(
            tracking_reference(&work, "origin").unwrap(),
            "refs/remotes/origin/main"
        );

        fs::write(work.join("b.md"), "# b").unwrap();
        git(&work, &["add", "-A"]);
        git(&work, &["commit", "-m", "two"]);
        let status = git_status(&work, "origin").unwrap();
        assert_eq!(status.relation, Some(GitPullRelation::LocalAhead));
    }
}
