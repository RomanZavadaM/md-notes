//! Tauri shell of MD Notes: exposes `notes-core` to the web UI as commands.

use std::sync::Mutex;

use notes_core::{Note, TreeEntry, Vault};
use serde::Serialize;
use tauri::State;

#[derive(Default)]
struct AppState {
    vault: Mutex<Option<Vault>>,
}

type CmdResult<T> = Result<T, String>;

fn with_vault<T>(
    state: &State<'_, AppState>,
    f: impl FnOnce(&Vault) -> notes_core::Result<T>,
) -> CmdResult<T> {
    let guard = state.vault.lock().map_err(|e| e.to_string())?;
    let vault = guard
        .as_ref()
        .ok_or_else(|| "no vault is open".to_string())?;
    f(vault).map_err(|e| e.to_string())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct VaultInfo {
    root: String,
    name: String,
}

#[tauri::command]
fn open_vault(path: String, state: State<'_, AppState>) -> CmdResult<VaultInfo> {
    let vault = Vault::open(&path).map_err(|e| e.to_string())?;
    let info = VaultInfo {
        root: vault.root().display().to_string(),
        name: vault.display_name(),
    };
    *state.vault.lock().map_err(|e| e.to_string())? = Some(vault);
    Ok(info)
}

#[tauri::command]
fn get_tree(state: State<'_, AppState>) -> CmdResult<Vec<TreeEntry>> {
    with_vault(&state, |v| v.tree())
}

#[tauri::command]
fn read_note(path: String, state: State<'_, AppState>) -> CmdResult<Note> {
    with_vault(&state, |v| v.read_note(&path))
}

#[tauri::command]
fn save_note(path: String, content: String, state: State<'_, AppState>) -> CmdResult<Note> {
    with_vault(&state, |v| v.write_note(&path, &content))
}

#[tauri::command]
fn create_note(dir: String, title: String, state: State<'_, AppState>) -> CmdResult<Note> {
    with_vault(&state, |v| v.create_note(&dir, &title))
}

#[tauri::command]
fn create_folder(parent: String, name: String, state: State<'_, AppState>) -> CmdResult<String> {
    with_vault(&state, |v| v.create_folder(&parent, &name))
}

#[tauri::command]
fn rename_entry(from: String, to: String, state: State<'_, AppState>) -> CmdResult<String> {
    with_vault(&state, |v| v.rename(&from, &to))
}

#[tauri::command]
fn trash_entry(path: String, state: State<'_, AppState>) -> CmdResult<String> {
    with_vault(&state, |v| v.move_to_trash(&path))
}

#[tauri::command]
fn resolve_link(target: String, state: State<'_, AppState>) -> CmdResult<Option<String>> {
    with_vault(&state, |v| v.resolve_link(&target))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            open_vault,
            get_tree,
            read_note,
            save_note,
            create_note,
            create_folder,
            rename_entry,
            trash_entry,
            resolve_link
        ])
        .run(tauri::generate_context!())
        .expect("error while running MD Notes");
}
