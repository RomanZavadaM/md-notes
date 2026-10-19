//! Tauri commands for encrypted vaults (age). The private key lives only in
//! the OS secret store; it is returned to the UI exactly once, when a vault is
//! created, so the user can back it up.
//!
//! Without the `encrypted-vault` feature the same commands exist but report
//! that encryption is unavailable, so the UI contract does not change.

#[cfg(feature = "encrypted-vault")]
use std::path::Path;

use notes_core::Vault;
use serde::Serialize;
use tauri::{AppHandle, State};

use crate::{activate_vault, AppState, CmdResult, VaultInfo};

/// Error text the UI maps to the "import your key" dialog.
#[cfg(feature = "encrypted-vault")]
pub(crate) const KEY_MISSING: &str = "KEY_MISSING";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EncryptionStatus {
    /// The folder contains an encrypted vault.
    encrypted: bool,
    /// This device holds the vault's private key.
    has_key: bool,
    vault_id: Option<String>,
    /// Number of public keys the vault is encrypted for.
    recipients: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedEncryptedVault {
    info: VaultInfo,
    /// Private key to back up. Shown once; not retrievable later.
    secret_key: String,
    public_key: String,
}

#[cfg(not(feature = "encrypted-vault"))]
const UNAVAILABLE: &str = "encrypted vaults are not available in this build";

#[cfg(feature = "encrypted-vault")]
use notes_core::encrypted_vault::{
    add_recipient, create_encrypted_vault, key_matches, open_encrypted_vault, EncryptionConfig,
};

#[cfg(feature = "encrypted-vault")]
use crate::mobile::secret_store;

/// Opens a vault folder: plain vaults as before, encrypted ones with the key
/// from the OS secret store (`KEY_MISSING` when this device has none).
pub(crate) fn open_any(path: &str) -> CmdResult<Vault> {
    #[cfg(feature = "encrypted-vault")]
    {
        let root = Path::new(path);
        if let Some(config) = EncryptionConfig::load(root).map_err(|e| e.to_string())? {
            let key = secret_store::load_vault_key(&config.vault_id)?
                .ok_or_else(|| KEY_MISSING.to_string())?;
            return open_encrypted_vault(root, &key).map_err(|e| e.to_string());
        }
    }
    Vault::open(path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn encryption_status(path: String) -> CmdResult<EncryptionStatus> {
    #[cfg(feature = "encrypted-vault")]
    {
        let config = EncryptionConfig::load(Path::new(&path)).map_err(|e| e.to_string())?;
        if let Some(config) = config {
            let has_key = secret_store::load_vault_key(&config.vault_id)?.is_some();
            return Ok(EncryptionStatus {
                encrypted: true,
                has_key,
                recipients: config.recipients.len(),
                vault_id: Some(config.vault_id),
            });
        }
    }
    #[cfg(not(feature = "encrypted-vault"))]
    let _ = path;
    Ok(EncryptionStatus {
        encrypted: false,
        has_key: false,
        vault_id: None,
        recipients: 0,
    })
}

#[tauri::command]
pub fn create_encrypted_vault_cmd(
    path: String,
    name: Option<String>,
    recovery_public_key: Option<String>,
    app: AppHandle,
    state: State<'_, AppState>,
) -> CmdResult<CreatedEncryptedVault> {
    #[cfg(feature = "encrypted-vault")]
    {
        let extra: Vec<String> = recovery_public_key
            .map(|key| key.trim().to_string())
            .filter(|key| !key.is_empty())
            .into_iter()
            .collect();
        let created =
            create_encrypted_vault(Path::new(&path), name, &extra).map_err(|e| e.to_string())?;
        // Keep the key before anything else can fail: losing it would make the
        // freshly created vault unreadable.
        secret_store::save_vault_key(&created.vault_id, &created.secret_key)?;
        let secret_key = created.secret_key;
        let public_key = created.public_key;
        let info = activate_vault(created.vault, app, state)?;
        Ok(CreatedEncryptedVault {
            info,
            secret_key,
            public_key,
        })
    }
    #[cfg(not(feature = "encrypted-vault"))]
    {
        let _ = (path, name, recovery_public_key, app, state);
        Err(UNAVAILABLE.into())
    }
}

/// Lists files of the open encrypted vault that would be pushed in plaintext.
/// Empty for plain vaults and for fully encrypted ones. A future push command
/// must call this first and refuse when the list is not empty.
#[tauri::command]
pub fn push_safety(state: State<'_, AppState>) -> CmdResult<serde_json::Value> {
    #[cfg(feature = "encrypted-vault")]
    {
        let guard = state.session.lock().map_err(|e| e.to_string())?;
        let session = guard
            .as_ref()
            .ok_or_else(|| "no vault is open".to_string())?;
        let findings = notes_core::push_guard::find_plaintext(session.vault.root())
            .map_err(|e| e.to_string())?;
        serde_json::to_value(findings).map_err(|e| e.to_string())
    }
    #[cfg(not(feature = "encrypted-vault"))]
    {
        let _ = state;
        Ok(serde_json::Value::Array(Vec::new()))
    }
}

/// Imports a private key (for example on a second device), checks that it
/// belongs to the vault in `path`, stores it and opens the vault.
#[tauri::command]
pub fn import_encryption_key(
    path: String,
    secret_key: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> CmdResult<VaultInfo> {
    #[cfg(feature = "encrypted-vault")]
    {
        let root = Path::new(&path);
        let config = EncryptionConfig::load(root)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "this folder is not an encrypted vault".to_string())?;
        let secret_key = secret_key.trim().to_string();
        if !key_matches(&config, &secret_key).map_err(|e| e.to_string())? {
            return Err("the key does not belong to this vault".into());
        }
        let vault = open_encrypted_vault(root, &secret_key).map_err(|e| e.to_string())?;
        secret_store::save_vault_key(&config.vault_id, &secret_key)?;
        activate_vault(vault, app, state)
    }
    #[cfg(not(feature = "encrypted-vault"))]
    {
        let _ = (path, secret_key, app, state);
        Err(UNAVAILABLE.into())
    }
}

/// Adds a public key (for example a recovery key) to the open vault and
/// re-encrypts its files. Returns the number of files rewritten.
#[tauri::command]
pub fn add_encryption_recipient(recipient: String, state: State<'_, AppState>) -> CmdResult<u32> {
    #[cfg(feature = "encrypted-vault")]
    {
        // Hold the session lock so no note is written while files are rewritten.
        let guard = state.session.lock().map_err(|e| e.to_string())?;
        let session = guard
            .as_ref()
            .ok_or_else(|| "no vault is open".to_string())?;
        let root = session.vault.root().to_path_buf();
        let config = EncryptionConfig::load(&root)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "the open vault is not encrypted".to_string())?;
        let key = secret_store::load_vault_key(&config.vault_id)?
            .ok_or_else(|| KEY_MISSING.to_string())?;
        add_recipient(&root, &key, &recipient).map_err(|e| e.to_string())
    }
    #[cfg(not(feature = "encrypted-vault"))]
    {
        let _ = (recipient, state);
        Err(UNAVAILABLE.into())
    }
}
