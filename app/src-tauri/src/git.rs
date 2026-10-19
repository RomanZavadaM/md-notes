//! Tauri commands of the Git panel. Credentials stay in the OS secret store;
//! encrypted vaults go through the push guard before any commit.

use std::path::PathBuf;

use tauri::State;

use crate::{AppState, CmdResult};

fn vault_root(state: &State<'_, AppState>) -> CmdResult<PathBuf> {
    let guard = state.session.lock().map_err(|e| e.to_string())?;
    let session = guard
        .as_ref()
        .ok_or_else(|| "no vault is open".to_string())?;
    Ok(session.vault.root().to_path_buf())
}

#[cfg(any(target_os = "android", target_os = "ios"))]
const DESKTOP_ONLY: &str = "the Git panel is available on desktop only";

fn remote_or_origin(remote_name: Option<String>) -> String {
    remote_name
        .filter(|name| !name.trim().is_empty())
        .unwrap_or_else(|| "origin".to_string())
}

/// Repository status of the open vault: branch, remote, changes, relation to
/// the last fetched remote branch.
#[tauri::command]
pub fn git_status_cmd(
    remote_name: Option<String>,
    state: State<'_, AppState>,
) -> CmdResult<serde_json::Value> {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let _ = (remote_name, state);
        Err(DESKTOP_ONLY.into())
    }
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let root = vault_root(&state)?;
        let status = notes_core::git_status(&root, &remote_or_origin(remote_name))
            .map_err(|e| e.to_string())?;
        serde_json::to_value(status).map_err(|e| e.to_string())
    }
}

/// Creates a repository (branch `main`) in the vault folder.
#[tauri::command]
pub fn git_init_repo(state: State<'_, AppState>) -> CmdResult<()> {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let _ = state;
        Err(DESKTOP_ONLY.into())
    }
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        notes_core::git_init(vault_root(&state)?).map_err(|e| e.to_string())
    }
}

/// Sets the HTTPS remote of the vault repository.
#[tauri::command]
pub fn git_set_remote_url(
    remote_name: Option<String>,
    url: String,
    state: State<'_, AppState>,
) -> CmdResult<()> {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let _ = (remote_name, url, state);
        Err(DESKTOP_ONLY.into())
    }
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let root = vault_root(&state)?;
        notes_core::git_set_remote(&root, &remote_or_origin(remote_name), &url)
            .map_err(|e| e.to_string())
    }
}

/// Commits all changes with the application identity; returns the new commit
/// id or `None` when there was nothing to commit. In an encrypted vault
/// nothing but ciphertext may be committed.
#[tauri::command]
pub fn git_commit_all(message: String, state: State<'_, AppState>) -> CmdResult<Option<String>> {
    let root = vault_root(&state)?;
    notes_core::push_guard::ensure_push_safe(&root).map_err(|e| e.to_string())?;
    notes_core::commit_git_worktree(&root, &message).map_err(|e| e.to_string())
}

/// Fetches, then fast-forwards when that is safe (clean worktree, remote
/// ahead). Returns `{relation, worktreeClean, applied}`.
#[tauri::command]
pub fn git_pull_with_stored_credentials(
    remote_name: Option<String>,
    state: State<'_, AppState>,
) -> CmdResult<serde_json::Value> {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let _ = (remote_name, state);
        Err(DESKTOP_ONLY.into())
    }
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let root = vault_root(&state)?;
        let remote = remote_or_origin(remote_name);
        let credentials = crate::mobile::secret_store::load_git_credentials()?
            .ok_or_else(|| "Git HTTPS credentials are not configured".to_string())?;
        notes_core::fetch_git_remote_authenticated(&root, Some(&remote), &credentials)
            .map_err(|e| e.to_string())?;
        let tracking = notes_core::tracking_reference(&root, &remote).map_err(|e| e.to_string())?;
        let preflight =
            notes_core::plan_git_pull_preflight(&root, &tracking).map_err(|e| e.to_string())?;
        let applied = if preflight.can_fast_forward {
            notes_core::apply_git_fast_forward(&root, &tracking).map_err(|e| e.to_string())?;
            true
        } else {
            false
        };
        Ok(serde_json::json!({
            "relation": preflight.relation,
            "worktreeClean": preflight.worktree_clean,
            "applied": applied,
        }))
    }
}
