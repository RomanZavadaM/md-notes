//! Git push through the system `git` program (desktop only).
//!
//! `gix` cannot push yet, so this is the one place MD Notes runs the user's
//! Git. Everything around it is locked down:
//!
//! - the push guard runs first and refuses an encrypted vault that holds
//!   anything but ciphertext (ADR-0007);
//! - only HTTPS remotes without credentials in the URL are accepted;
//! - the token goes to Git through environment variables of the child
//!   process (`http.extraHeader`), never through arguments, the URL, the
//!   repository config or a credential helper (helpers are switched off);
//! - the user's global Git config is ignored, hooks are skipped
//!   (`--no-verify`), prompts are disabled and a stalled transfer is aborted;
//! - only the current branch is pushed, never with force: a rejected
//!   (non-fast-forward) push is an error.

use std::path::Path;
use std::process::{Command, Output};

use serde::Serialize;

use crate::error::{Error, Result};
use crate::git_sync::{validate_git_remote, GitHttpsCredentials};
use crate::push_guard::ensure_push_safe;

/// Result of a successful push.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GitPushOutcome {
    /// New commits were sent.
    Pushed,
    /// The remote already had everything.
    UpToDate,
}

/// Pushes the current branch to `remote_name` (default `origin`) over HTTPS
/// with credentials supplied in memory. Never forces.
pub fn push_git_branch(
    path: impl AsRef<Path>,
    remote_name: Option<&str>,
    credentials: &GitHttpsCredentials,
) -> Result<GitPushOutcome> {
    push_inner(
        path.as_ref(),
        remote_name.unwrap_or("origin"),
        Some(credentials),
        true,
    )
}

fn push_inner(
    root: &Path,
    remote: &str,
    credentials: Option<&GitHttpsCredentials>,
    https_only: bool,
) -> Result<GitPushOutcome> {
    validate_remote_name(remote)?;
    ensure_push_safe(root)?;
    let branch = current_branch(root)?;
    let url = capture(root, &["remote", "get-url", "--push", "--", remote], None)?;
    if https_only {
        validate_git_remote(url.trim())?;
    }
    let refspec = format!("HEAD:refs/heads/{branch}");
    let output = run(
        root,
        &[
            "push",
            "--porcelain",
            "--no-verify",
            "--no-recurse-submodules",
            "--",
            remote,
            &refspec,
        ],
        credentials,
    )?;
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let detail: &str = if stderr.trim().is_empty() {
            stdout.as_str()
        } else {
            &stderr
        };
        return Err(Error::Git(format!(
            "git push failed: {}",
            sanitize(detail, credentials)
        )));
    }
    Ok(parse_porcelain(&stdout))
}

/// `--porcelain` lines look like `<flag>\t<from>:<to>\t<summary>`; `=` means
/// the ref was already up to date.
fn parse_porcelain(stdout: &str) -> GitPushOutcome {
    let mut up_to_date = false;
    let mut sent = false;
    for line in stdout.lines() {
        match line.chars().next() {
            Some('=') => up_to_date = true,
            Some(' ') | Some('*') | Some('+') | Some('-') => sent = true,
            _ => {}
        }
    }
    if up_to_date && !sent {
        GitPushOutcome::UpToDate
    } else {
        GitPushOutcome::Pushed
    }
}

fn validate_remote_name(name: &str) -> Result<()> {
    let ok = !name.is_empty()
        && !name.starts_with('-')
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'));
    if ok {
        Ok(())
    } else {
        Err(Error::Git("Git remote name is not valid".into()))
    }
}

fn current_branch(root: &Path) -> Result<String> {
    let branch = capture(root, &["symbolic-ref", "--short", "-q", "HEAD"], None)
        .map_err(|_| Error::Git("push needs a checked-out branch (HEAD is detached)".into()))?;
    let branch = branch.trim().to_string();
    let ok = !branch.is_empty()
        && !branch.starts_with('-')
        && !branch.chars().any(|c| c.is_whitespace() || c.is_control());
    if ok {
        Ok(branch)
    } else {
        Err(Error::Git(
            "current Git branch name is not supported".into(),
        ))
    }
}

/// Runs a read-only Git command and returns its trimmed-by-caller stdout.
pub(crate) fn capture(
    root: &Path,
    args: &[&str],
    credentials: Option<&GitHttpsCredentials>,
) -> Result<String> {
    let output = run(root, args, credentials)?;
    if !output.status.success() {
        return Err(Error::Git(format!(
            "git {} failed: {}",
            args.first().copied().unwrap_or(""),
            sanitize(&String::from_utf8_lossy(&output.stderr), credentials)
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

pub(crate) fn run(
    root: &Path,
    args: &[&str],
    credentials: Option<&GitHttpsCredentials>,
) -> Result<Output> {
    let mut command = Command::new("git");
    command.arg("-C").arg(root).args(args);
    command.env("GIT_TERMINAL_PROMPT", "0");
    command.env("GIT_CONFIG_GLOBAL", null_device());

    // Command-scope config through the environment keeps the token out of the
    // process arguments. An empty `credential.helper` resets all helpers.
    let mut pairs: Vec<(&str, String)> = vec![
        ("credential.helper", String::new()),
        ("http.lowSpeedLimit", "1000".into()),
        ("http.lowSpeedTime", "60".into()),
    ];
    if let Some(credentials) = credentials {
        pairs.push(("http.extraHeader", authorization_header(credentials)));
    }
    command.env("GIT_CONFIG_COUNT", pairs.len().to_string());
    for (index, (key, value)) in pairs.iter().enumerate() {
        command.env(format!("GIT_CONFIG_KEY_{index}"), key);
        command.env(format!("GIT_CONFIG_VALUE_{index}"), value);
    }

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        // CREATE_NO_WINDOW: no console flash from the GUI app.
        command.creation_flags(0x0800_0000);
    }

    command.output().map_err(|err| {
        if err.kind() == std::io::ErrorKind::NotFound {
            Error::Git("system Git is required for push but was not found in PATH".into())
        } else {
            Error::Git(format!("cannot run git: {err}"))
        }
    })
}

fn null_device() -> &'static str {
    if cfg!(windows) {
        "NUL"
    } else {
        "/dev/null"
    }
}

fn authorization_header(credentials: &GitHttpsCredentials) -> String {
    let pair = format!("{}:{}", credentials.username, credentials.token);
    format!("Authorization: Basic {}", base64_encode(pair.as_bytes()))
}

fn sanitize(text: &str, credentials: Option<&GitHttpsCredentials>) -> String {
    let mut out = text.to_string();
    if let Some(credentials) = credentials {
        let header = authorization_header(credentials);
        let encoded = header.rsplit(' ').next().unwrap_or_default().to_string();
        for secret in [
            credentials.token.as_str(),
            header.as_str(),
            encoded.as_str(),
        ] {
            if !secret.is_empty() {
                out = out.replace(secret, "***");
            }
        }
    }
    out.chars()
        .filter(|c| !c.is_control() || *c == '\n')
        .take(600)
        .collect::<String>()
        .trim()
        .to_string()
}

fn base64_encode(input: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(input.len().div_ceil(3) * 4);
    for chunk in input.chunks(3) {
        let b = [
            chunk[0],
            chunk.get(1).copied().unwrap_or(0),
            chunk.get(2).copied().unwrap_or(0),
        ];
        out.push(TABLE[(b[0] >> 2) as usize] as char);
        out.push(TABLE[(((b[0] & 0x03) << 4) | (b[1] >> 4)) as usize] as char);
        if chunk.len() > 1 {
            out.push(TABLE[(((b[1] & 0x0f) << 2) | (b[2] >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(TABLE[(b[2] & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn git_available() -> bool {
        Command::new("git")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Runs git with a fixed identity and without user config; panics on failure.
    fn git(dir: &Path, args: &[&str]) -> String {
        let output = Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(["-c", "user.name=Test", "-c", "user.email=test@example.com"])
            .args(["-c", "commit.gpgsign=false", "-c", "core.autocrlf=false"])
            .args(args)
            .env("GIT_CONFIG_GLOBAL", null_device())
            .output()
            .expect("git runs");
        assert!(
            output.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    /// A work repository on branch `main` with a bare `origin` next to it.
    fn repo_with_remote() -> (tempfile::TempDir, std::path::PathBuf, std::path::PathBuf) {
        let tmp = tempfile::tempdir().unwrap();
        let work = tmp.path().join("work");
        let bare = tmp.path().join("remote.git");
        fs::create_dir_all(&work).unwrap();
        fs::create_dir_all(&bare).unwrap();
        git(&bare, &["init", "--bare"]);
        git(&work, &["init"]);
        git(&work, &["symbolic-ref", "HEAD", "refs/heads/main"]);
        git(&work, &["remote", "add", "origin", bare.to_str().unwrap()]);
        (tmp, work, bare)
    }

    #[test]
    fn base64_matches_known_vectors() {
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
        assert_eq!(base64_encode(b"user:pass"), "dXNlcjpwYXNz");
    }

    #[test]
    fn header_and_error_text_never_leak_the_token() {
        let credentials = GitHttpsCredentials::new("roman", "tok-123").unwrap();
        let header = authorization_header(&credentials);
        assert!(header.starts_with("Authorization: Basic "));
        let cleaned = sanitize(
            &format!("fatal: auth failed for tok-123 and {header}\u{7}"),
            Some(&credentials),
        );
        assert!(!cleaned.contains("tok-123"));
        assert!(!cleaned.contains("Basic cm9tYW46"));
        assert!(cleaned.contains("***"));
    }

    #[test]
    fn remote_names_cannot_inject_options() {
        assert!(validate_remote_name("origin").is_ok());
        assert!(validate_remote_name("my-remote.2").is_ok());
        assert!(validate_remote_name("").is_err());
        assert!(validate_remote_name("-x").is_err());
        assert!(validate_remote_name("a b").is_err());
        assert!(validate_remote_name("--upload-pack=evil").is_err());
    }

    #[test]
    fn plain_https_requirement_applies_to_the_public_entry_point() {
        if !git_available() {
            return;
        }
        let (_tmp, work, _bare) = repo_with_remote();
        fs::write(work.join("a.md"), "# a").unwrap();
        git(&work, &["add", "-A"]);
        git(&work, &["commit", "-m", "init"]);
        let credentials = GitHttpsCredentials::new("roman", "tok").unwrap();
        let err = push_git_branch(&work, None, &credentials)
            .unwrap_err()
            .to_string();
        assert!(err.contains("HTTPS"), "{err}");
    }

    #[test]
    fn pushes_then_reports_up_to_date() {
        if !git_available() {
            return;
        }
        let (_tmp, work, bare) = repo_with_remote();
        fs::write(work.join("a.md"), "# a").unwrap();
        git(&work, &["add", "-A"]);
        git(&work, &["commit", "-m", "init"]);
        assert_eq!(
            push_inner(&work, "origin", None, false).unwrap(),
            GitPushOutcome::Pushed
        );
        assert_eq!(
            git(&bare, &["rev-parse", "refs/heads/main"]),
            git(&work, &["rev-parse", "HEAD"])
        );
        assert_eq!(
            push_inner(&work, "origin", None, false).unwrap(),
            GitPushOutcome::UpToDate
        );
    }

    #[test]
    fn rewritten_history_is_rejected_not_forced() {
        if !git_available() {
            return;
        }
        let (_tmp, work, bare) = repo_with_remote();
        fs::write(work.join("a.md"), "# a").unwrap();
        git(&work, &["add", "-A"]);
        git(&work, &["commit", "-m", "init"]);
        push_inner(&work, "origin", None, false).unwrap();
        let remote_head = git(&bare, &["rev-parse", "refs/heads/main"]);
        git(
            &work,
            &["commit", "--amend", "-m", "rewritten", "--allow-empty"],
        );
        let err = push_inner(&work, "origin", None, false)
            .unwrap_err()
            .to_string();
        assert!(err.contains("git push failed"), "{err}");
        assert_eq!(git(&bare, &["rev-parse", "refs/heads/main"]), remote_head);
    }

    #[test]
    fn detached_head_is_refused() {
        if !git_available() {
            return;
        }
        let (_tmp, work, _bare) = repo_with_remote();
        fs::write(work.join("a.md"), "# a").unwrap();
        git(&work, &["add", "-A"]);
        git(&work, &["commit", "-m", "init"]);
        git(&work, &["checkout", "--detach"]);
        let err = push_inner(&work, "origin", None, false)
            .unwrap_err()
            .to_string();
        assert!(err.contains("detached"), "{err}");
    }

    #[cfg(feature = "encrypted-vault")]
    mod encrypted {
        use super::*;
        use crate::encrypted_vault::create_encrypted_vault;

        fn encrypted_repo() -> (tempfile::TempDir, std::path::PathBuf, std::path::PathBuf) {
            let (tmp, work, bare) = repo_with_remote();
            let created = create_encrypted_vault(&work, None, &[]).unwrap();
            created
                .vault
                .write_note("secret.md", "# Secret\n\nbody")
                .unwrap();
            (tmp, work, bare)
        }

        #[test]
        fn ciphertext_only_vault_is_pushed() {
            if !git_available() {
                return;
            }
            let (_tmp, work, bare) = encrypted_repo();
            git(&work, &["add", "-A"]);
            git(&work, &["commit", "-m", "notes"]);
            assert_eq!(
                push_inner(&work, "origin", None, false).unwrap(),
                GitPushOutcome::Pushed
            );
            let tree = git(&bare, &["ls-tree", "-r", "--name-only", "main"]);
            for name in tree.lines() {
                assert!(
                    name.ends_with(".age") || name.starts_with(".mdnotes/"),
                    "unexpected file on the remote: {name}"
                );
            }
        }

        #[test]
        fn plaintext_in_an_encrypted_vault_blocks_the_push() {
            if !git_available() {
                return;
            }
            let (_tmp, work, bare) = encrypted_repo();
            fs::write(work.join("leak.md"), "# leaked").unwrap();
            git(&work, &["add", "-A"]);
            git(&work, &["commit", "-m", "notes"]);
            let err = push_inner(&work, "origin", None, false)
                .unwrap_err()
                .to_string();
            assert!(err.contains("push refused"), "{err}");
            let refs = git(&bare, &["for-each-ref"]);
            assert!(refs.is_empty(), "the remote must stay empty: {refs}");
        }
    }
}
