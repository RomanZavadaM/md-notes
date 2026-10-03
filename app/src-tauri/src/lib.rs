//! Tauri shell of MD Notes: exposes `notes-core` to the web UI as commands.

use std::collections::BTreeMap;
use std::sync::Mutex;

use notes_core::{
    create_vault_with_preset, knowledge_graph, AttachmentInfo, Backlink, Index, KnowledgeGraph,
    Note, NoteSummary, RenameOutcome, SchemaDocument, SearchHit, TagCount, TemplateInfo, TreeEntry,
    UnresolvedLink, Vault, VaultPreset,
};
use serde::Serialize;
use serde_json::Value;
use tauri::{AppHandle, Emitter, Manager, State};

mod mobile;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod watch;

struct Session {
    vault: Vault,
    index: Index,
}

#[derive(Default)]
struct AppState {
    session: Mutex<Option<Session>>,
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    watcher: Mutex<Option<watch::VaultWatcher>>,
}

type CmdResult<T> = Result<T, String>;

fn with_session<T>(
    state: &State<'_, AppState>,
    f: impl FnOnce(&mut Session) -> notes_core::Result<T>,
) -> CmdResult<T> {
    let mut guard = state.session.lock().map_err(|e| e.to_string())?;
    let session = guard
        .as_mut()
        .ok_or_else(|| "no vault is open".to_string())?;
    f(session).map_err(|e| e.to_string())
}

const VAULT_CHANGED: &str = "vault-changed";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct VaultInfo {
    root: String,
    name: String,
}

fn activate_vault(
    vault: Vault,
    app: AppHandle,
    state: State<'_, AppState>,
) -> CmdResult<VaultInfo> {
    let mut index = Index::open_for(&vault).map_err(|e| e.to_string())?;
    index.sync(&vault).map_err(|e| e.to_string())?;
    let info = VaultInfo {
        root: vault.root().display().to_string(),
        name: vault.display_name(),
    };
    app.asset_protocol_scope()
        .allow_directory(vault.root(), true)
        .map_err(|e| e.to_string())?;

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let handle = app.clone();
        let watcher = watch::watch(vault.root(), move |changed| {
            on_files_changed(&handle, changed)
        })
        .map_err(|e| e.to_string())?;
        *state.watcher.lock().map_err(|e| e.to_string())? = Some(watcher);
    }
    *state.session.lock().map_err(|e| e.to_string())? = Some(Session { vault, index });
    Ok(info)
}

#[tauri::command]
fn runtime_platform() -> &'static str {
    mobile::runtime_platform()
}

#[tauri::command]
fn open_mobile_sandbox_vault(app: AppHandle, state: State<'_, AppState>) -> CmdResult<VaultInfo> {
    let vault = mobile::open_or_create_sandbox_vault(&app)?;
    activate_vault(vault, app, state)
}

#[tauri::command]
fn open_vault(path: String, app: AppHandle, state: State<'_, AppState>) -> CmdResult<VaultInfo> {
    let vault = Vault::open(&path).map_err(|e| e.to_string())?;
    activate_vault(vault, app, state)
}

#[tauri::command]
fn create_vault(
    path: String,
    name: Option<String>,
    preset: VaultPreset,
    app: AppHandle,
    state: State<'_, AppState>,
) -> CmdResult<VaultInfo> {
    let vault = create_vault_with_preset(&path, name, preset).map_err(|e| e.to_string())?;
    activate_vault(vault, app, state)
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn on_files_changed(app: &AppHandle, changed: Vec<String>) {
    let state = app.state::<AppState>();
    if let Ok(mut guard) = state.session.lock() {
        if let Some(session) = guard.as_mut() {
            if let Err(err) = session.index.sync(&session.vault) {
                eprintln!("index sync failed: {err}");
            }
        }
    }
    let _ = app.emit(VAULT_CHANGED, changed);
}

#[tauri::command]
fn get_tree(state: State<'_, AppState>) -> CmdResult<Vec<TreeEntry>> {
    with_session(&state, |s| s.vault.tree())
}

#[tauri::command]
fn read_note(path: String, state: State<'_, AppState>) -> CmdResult<Note> {
    with_session(&state, |s| s.vault.read_note(&path))
}

#[tauri::command]
fn parse_note_content(path: String, content: String) -> Note {
    Note::parse(&path, content)
}

#[tauri::command]
fn save_note(path: String, content: String, state: State<'_, AppState>) -> CmdResult<Note> {
    with_session(&state, |s| {
        let note = s.vault.write_note(&path, &content)?;
        s.index.update_note(&s.vault, &note)?;
        Ok(note)
    })
}

#[tauri::command]
fn get_schema(state: State<'_, AppState>) -> CmdResult<SchemaDocument> {
    with_session(&state, |s| s.vault.schema())
}

#[tauri::command]
fn format_note_properties(
    content: String,
    patch: BTreeMap<String, Value>,
    state: State<'_, AppState>,
) -> CmdResult<String> {
    with_session(&state, |s| s.vault.format_note_properties(&content, &patch))
}

#[tauri::command]
fn update_note_properties(
    path: String,
    patch: BTreeMap<String, Value>,
    state: State<'_, AppState>,
) -> CmdResult<Note> {
    with_session(&state, |s| {
        let note = s.vault.update_note_properties(&path, &patch)?;
        s.index.update_note(&s.vault, &note)?;
        Ok(note)
    })
}

#[tauri::command]
fn create_note(
    dir: String,
    title: String,
    template: Option<String>,
    state: State<'_, AppState>,
) -> CmdResult<Note> {
    with_session(&state, |s| {
        let note = match template.as_deref() {
            Some(name) => s.vault.create_from_template(&dir, &title, name)?,
            None => s.vault.create_note(&dir, &title)?,
        };
        s.index.update_note(&s.vault, &note)?;
        Ok(note)
    })
}

#[tauri::command]
fn list_templates(state: State<'_, AppState>) -> CmdResult<Vec<TemplateInfo>> {
    with_session(&state, |s| s.vault.templates())
}

#[tauri::command]
fn open_daily(state: State<'_, AppState>) -> CmdResult<Note> {
    with_session(&state, |s| {
        let note = s.vault.open_daily(chrono::Local::now().date_naive())?;
        s.index.update_note(&s.vault, &note)?;
        Ok(note)
    })
}

#[tauri::command]
fn create_folder(parent: String, name: String, state: State<'_, AppState>) -> CmdResult<String> {
    with_session(&state, |s| s.vault.create_folder(&parent, &name))
}

#[tauri::command]
fn rename_entry(from: String, to: String, state: State<'_, AppState>) -> CmdResult<RenameOutcome> {
    with_session(&state, |s| {
        notes_core::rename_with_links(&s.vault, &mut s.index, &from, &to)
    })
}

#[tauri::command]
fn trash_entry(path: String, state: State<'_, AppState>) -> CmdResult<String> {
    with_session(&state, |s| {
        let trashed = s.vault.move_to_trash(&path)?;
        s.index.sync(&s.vault)?;
        Ok(trashed)
    })
}

#[tauri::command]
fn resolve_link(target: String, state: State<'_, AppState>) -> CmdResult<Option<String>> {
    with_session(&state, |s| Ok(s.index.resolve(&target)))
}

#[tauri::command]
fn list_notes(state: State<'_, AppState>) -> CmdResult<Vec<NoteSummary>> {
    with_session(&state, |s| s.index.notes())
}

#[tauri::command]
fn backlinks(path: String, state: State<'_, AppState>) -> CmdResult<Vec<Backlink>> {
    with_session(&state, |s| s.index.backlinks(&path))
}

#[tauri::command]
fn knowledge_graph_snapshot(
    focus: Option<String>,
    state: State<'_, AppState>,
) -> CmdResult<KnowledgeGraph> {
    with_session(&state, |s| knowledge_graph(&s.index, focus.as_deref()))
}

#[tauri::command]
fn unresolved_links(state: State<'_, AppState>) -> CmdResult<Vec<UnresolvedLink>> {
    with_session(&state, |s| s.index.unresolved_links())
}

#[tauri::command]
fn list_tags(state: State<'_, AppState>) -> CmdResult<Vec<TagCount>> {
    with_session(&state, |s| s.index.tags())
}

#[tauri::command]
fn search(query: String, limit: u32, state: State<'_, AppState>) -> CmdResult<Vec<SearchHit>> {
    with_session(&state, |s| s.index.search(&query, limit))
}

#[tauri::command]
fn import_attachment(source: String, state: State<'_, AppState>) -> CmdResult<AttachmentInfo> {
    with_session(&state, |s| s.vault.import_attachment(source))
}

#[tauri::command]
fn list_attachments(state: State<'_, AppState>) -> CmdResult<Vec<AttachmentInfo>> {
    with_session(&state, |s| s.vault.attachments())
}

#[tauri::command]
fn attachment_used_by(path: String, state: State<'_, AppState>) -> CmdResult<Vec<String>> {
    with_session(&state, |s| s.vault.attachment_used_by(&path))
}

#[tauri::command]
fn orphan_attachments(state: State<'_, AppState>) -> CmdResult<Vec<AttachmentInfo>> {
    with_session(&state, |s| s.vault.orphan_attachments())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            runtime_platform,
            open_mobile_sandbox_vault,
            open_vault,
            create_vault,
            get_tree,
            read_note,
            parse_note_content,
            save_note,
            get_schema,
            format_note_properties,
            update_note_properties,
            create_note,
            create_folder,
            list_templates,
            open_daily,
            rename_entry,
            trash_entry,
            resolve_link,
            list_notes,
            backlinks,
            knowledge_graph_snapshot,
            unresolved_links,
            list_tags,
            search,
            import_attachment,
            list_attachments,
            attachment_used_by,
            orphan_attachments
        ])
        .run(tauri::generate_context!())
        .expect("error while running MD Notes");
}
