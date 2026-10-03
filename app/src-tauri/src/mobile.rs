use std::path::PathBuf;

use notes_core::{create_vault_with_preset, Vault, VaultPreset};
use tauri::{AppHandle, Manager};

pub fn runtime_platform() -> &'static str {
    #[cfg(target_os = "android")]
    {
        return "android";
    }
    #[cfg(target_os = "ios")]
    {
        return "ios";
    }
    #[cfg(target_os = "windows")]
    {
        return "windows";
    }
    #[cfg(target_os = "macos")]
    {
        return "macos";
    }
    #[cfg(target_os = "linux")]
    {
        return "linux";
    }
    #[allow(unreachable_code)]
    "unknown"
}

pub fn sandbox_vault_root(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data = app.path().app_data_dir().map_err(|e| e.to_string())?;
    Ok(app_data.join("vault"))
}

pub fn open_or_create_sandbox_vault(app: &AppHandle) -> Result<Vault, String> {
    let root = sandbox_vault_root(app)?;
    std::fs::create_dir_all(&root).map_err(|e| e.to_string())?;

    if root.join(".mdnotes/config.json").is_file() {
        Vault::open(&root).map_err(|e| e.to_string())
    } else {
        create_vault_with_preset(&root, Some("MD Notes".into()), VaultPreset::Empty)
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_name_is_stable() {
        assert!(matches!(
            runtime_platform(),
            "android" | "ios" | "windows" | "macos" | "linux" | "unknown"
        ));
    }
}
