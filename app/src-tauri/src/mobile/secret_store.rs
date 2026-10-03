use std::collections::HashMap;
use std::sync::OnceLock;

use keyring_core::set_default_store;
use notes_core::GitHttpsCredentials;
use serde::{Deserialize, Serialize};

const GIT_CREDENTIAL_SERVICE: &str = "com.romanzavada.md-notes.git";
const GIT_CREDENTIAL_ACCOUNT: &str = "default-https";

static STORE_INIT: OnceLock<Result<(), String>> = OnceLock::new();

#[derive(Serialize, Deserialize)]
struct StoredGitCredential {
    username: String,
    token: String,
}

pub(crate) fn save_git_credentials(username: String, token: String) -> Result<(), String> {
    GitHttpsCredentials::new(username.clone(), token.clone()).map_err(|error| error.to_string())?;
    let payload = serde_json::to_string(&StoredGitCredential { username, token })
        .map_err(|error| format!("cannot encode Git credentials: {error}"))?;
    credential_entry()?
        .set_password(&payload)
        .map_err(store_error)
}

pub(crate) fn has_git_credentials() -> Result<bool, String> {
    match credential_entry()?.get_password() {
        Ok(secret) => {
            drop(secret);
            Ok(true)
        }
        Err(keyring_core::Error::NoEntry) => Ok(false),
        Err(error) => Err(store_error(error)),
    }
}

pub(crate) fn clear_git_credentials() -> Result<(), String> {
    match credential_entry()?.delete_credential() {
        Ok(()) | Err(keyring_core::Error::NoEntry) => Ok(()),
        Err(error) => Err(store_error(error)),
    }
}

pub(crate) fn load_git_credentials() -> Result<Option<GitHttpsCredentials>, String> {
    let payload = match credential_entry()?.get_password() {
        Ok(payload) => payload,
        Err(keyring_core::Error::NoEntry) => return Ok(None),
        Err(error) => return Err(store_error(error)),
    };
    let stored: StoredGitCredential = serde_json::from_str(&payload)
        .map_err(|error| format!("stored Git credentials are invalid: {error}"))?;
    GitHttpsCredentials::new(stored.username, stored.token)
        .map(Some)
        .map_err(|error| error.to_string())
}

fn credential_entry() -> Result<keyring_core::Entry, String> {
    ensure_store()?;
    keyring_core::Entry::new(GIT_CREDENTIAL_SERVICE, GIT_CREDENTIAL_ACCOUNT).map_err(store_error)
}

fn ensure_store() -> Result<(), String> {
    STORE_INIT.get_or_init(configure_native_store).clone()
}

fn configure_native_store() -> Result<(), String> {
    let config = HashMap::new();

    #[cfg(target_os = "android")]
    {
        use android_native_keyring_store::Store;
        let store = Store::new_with_configuration(&config).map_err(store_error)?;
        set_default_store(store);
        Ok(())
    }
    #[cfg(target_os = "ios")]
    {
        use apple_native_keyring_store::protected::Store;
        let store = Store::new_with_configuration(&config).map_err(store_error)?;
        set_default_store(store);
        Ok(())
    }
    #[cfg(target_os = "macos")]
    {
        use apple_native_keyring_store::keychain::Store;
        let store = Store::new_with_configuration(&config).map_err(store_error)?;
        set_default_store(store);
        Ok(())
    }
    #[cfg(target_os = "windows")]
    {
        use windows_native_keyring_store::Store;
        let store = Store::new_with_configuration(&config).map_err(store_error)?;
        set_default_store(store);
        Ok(())
    }
    #[cfg(target_os = "linux")]
    {
        use zbus_secret_service_keyring_store::Store;
        let store = Store::new_with_configuration(&config).map_err(store_error)?;
        set_default_store(store);
        Ok(())
    }
    #[cfg(not(any(
        target_os = "android",
        target_os = "ios",
        target_os = "macos",
        target_os = "windows",
        target_os = "linux"
    )))]
    {
        Err("system credential storage is not supported on this platform".into())
    }
}

fn store_error(error: impl std::fmt::Display) -> String {
    format!("system credential store error: {error}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stored_payload_round_trips_without_debug_surface() {
        let payload = serde_json::to_string(&StoredGitCredential {
            username: "roman".into(),
            token: "secret".into(),
        })
        .unwrap();
        let restored: StoredGitCredential = serde_json::from_str(&payload).unwrap();
        assert_eq!(restored.username, "roman");
        assert_eq!(restored.token, "secret");
    }
}
