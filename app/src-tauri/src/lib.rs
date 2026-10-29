//! Tauri shell of MD Notes: exposes `notes-core` to the web UI as commands.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
#[cfg(any(target_os = "windows", target_os = "linux"))]
use std::process::Command;
use std::sync::{Arc, Mutex};

use notes_core::{
    create_vault_backup, create_vault_with_preset, fetch_git_remote_authenticated, knowledge_graph,
    restore_vault_backup, AttachmentInfo, Backlink, BackupSummary, Index, KnowledgeGraph,
    LocalFsProvider, Note, NoteSummary, RenameOutcome, RestoreSummary, SchemaDocument, SearchHit,
    TagCount, TemplateInfo, TreeEntry, UnresolvedLink, Vault, VaultPreset, VaultStorage,
};
use serde::Serialize;
use serde_json::Value;
use tauri::{AppHandle, Emitter, Manager, State};

mod encryption;
mod git;
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
    #[cfg(target_os = "macos")]
    opened_markdown: Mutex<Vec<String>>,
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StandaloneMarkdown {
    path: String,
    directory: String,
    name: String,
    note: Note,
}

fn standalone_markdown_path(path: &str) -> CmdResult<(PathBuf, PathBuf, String)> {
    let path = Path::new(path);
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    if !matches!(extension.to_ascii_lowercase().as_str(), "md" | "markdown") {
        return Err("only .md and .markdown files are supported".into());
    }
    let absolute = path.canonicalize().map_err(|error| error.to_string())?;
    if !absolute.is_file() {
        return Err("Markdown path is not a file".into());
    }
    let directory = absolute
        .parent()
        .ok_or_else(|| "Markdown file has no parent directory".to_string())?
        .to_path_buf();
    let name = absolute
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "Markdown filename must be valid UTF-8".to_string())?
        .to_string();
    Ok((absolute, directory, name))
}

fn load_standalone_markdown(path: &str) -> CmdResult<StandaloneMarkdown> {
    let (absolute, directory, name) = standalone_markdown_path(path)?;
    let provider = Arc::new(LocalFsProvider::new(&directory).map_err(|error| error.to_string())?);
    let storage = VaultStorage::open(provider).map_err(|error| error.to_string())?;
    let note = storage
        .read_note(&name)
        .map_err(|error| error.to_string())?;
    Ok(StandaloneMarkdown {
        path: absolute.display().to_string(),
        directory: directory.display().to_string(),
        name,
        note,
    })
}

#[tauri::command]
fn open_markdown_file(path: String) -> CmdResult<StandaloneMarkdown> {
    load_standalone_markdown(&path)
}

#[tauri::command]
fn save_markdown_file(path: String, content: String) -> CmdResult<StandaloneMarkdown> {
    let (_, directory, name) = standalone_markdown_path(&path)?;
    let provider = Arc::new(LocalFsProvider::new(&directory).map_err(|error| error.to_string())?);
    let storage = VaultStorage::open(provider).map_err(|error| error.to_string())?;
    storage
        .write_note(&name, &content)
        .map_err(|error| error.to_string())?;
    load_standalone_markdown(&path)
}

#[tauri::command]
fn startup_markdown_files(_state: State<'_, AppState>) -> Vec<String> {
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    {
        std::env::args()
            .skip(1)
            .filter(|arg| {
                let path = Path::new(arg);
                path.is_file()
                    && path
                        .extension()
                        .and_then(|value| value.to_str())
                        .is_some_and(|ext| {
                            matches!(ext.to_ascii_lowercase().as_str(), "md" | "markdown")
                        })
            })
            .collect()
    }

    #[cfg(target_os = "macos")]
    {
        let state = _state;
        state
            .opened_markdown
            .lock()
            .map(|mut paths| std::mem::take(&mut *paths))
            .unwrap_or_default()
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        let _ = _state;
        Vec::new()
    }
}

#[cfg(target_os = "macos")]
fn set_macos_markdown_default() -> CmdResult<()> {
    use std::ffi::{c_char, c_void, CString};

    type CFStringRef = *const c_void;
    const UTF8_ENCODING: u32 = 0x0800_0100;
    const LS_ROLES_ALL: u32 = 0xffff_ffff;

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        fn CFStringCreateWithCString(
            allocator: *const c_void,
            c_str: *const c_char,
            encoding: u32,
        ) -> CFStringRef;
        fn CFRelease(value: *const c_void);
    }

    #[link(name = "CoreServices", kind = "framework")]
    extern "C" {
        fn LSSetDefaultRoleHandlerForContentType(
            content_type: CFStringRef,
            role: u32,
            handler_bundle_id: CFStringRef,
        ) -> i32;
    }

    fn cf_string(value: &str) -> CmdResult<CFStringRef> {
        let value = CString::new(value).map_err(|error| error.to_string())?;
        let string =
            unsafe { CFStringCreateWithCString(std::ptr::null(), value.as_ptr(), UTF8_ENCODING) };
        if string.is_null() {
            return Err("macOS could not create Launch Services string".into());
        }
        Ok(string)
    }

    let content_type = cf_string("net.daringfireball.markdown")?;
    let bundle_id = cf_string("io.github.romanzavadam.mdnotes")?;
    let status =
        unsafe { LSSetDefaultRoleHandlerForContentType(content_type, LS_ROLES_ALL, bundle_id) };
    unsafe {
        CFRelease(content_type);
        CFRelease(bundle_id);
    }
    if status == 0 {
        Ok(())
    } else {
        Err(format!("macOS Launch Services returned OSStatus {status}"))
    }
}

#[tauri::command]
fn configure_markdown_association() -> CmdResult<String> {
    #[cfg(target_os = "windows")]
    {
        let exe = std::env::current_exe().map_err(|error| error.to_string())?;
        let exe_text = exe.display().to_string();
        let command = format!("\"{exe_text}\" \"%1\"");
        let icon = format!("\"{exe_text}\",0");
        let exe_name = exe
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| "MD Notes executable name must be valid UTF-8".to_string())?;
        let application_key = format!(r"HKCU\Software\Classes\Applications\{exe_name}");
        let application_command_key = format!(r"{application_key}\shell\open\command");
        let application_types_key = format!(r"{application_key}\SupportedTypes");
        let registrations = [
            vec![
                "add",
                r"HKCU\Software\Classes\MDNotes.Markdown",
                "/ve",
                "/d",
                "Markdown Document",
                "/f",
            ],
            vec![
                "add",
                r"HKCU\Software\Classes\MDNotes.Markdown\DefaultIcon",
                "/ve",
                "/d",
                &icon,
                "/f",
            ],
            vec![
                "add",
                r"HKCU\Software\Classes\MDNotes.Markdown\shell\open\command",
                "/ve",
                "/d",
                &command,
                "/f",
            ],
            vec![
                "add",
                r"HKCU\Software\Classes\.md\OpenWithProgids",
                "/v",
                "MDNotes.Markdown",
                "/t",
                "REG_NONE",
                "/f",
            ],
            vec![
                "add",
                r"HKCU\Software\Classes\.markdown\OpenWithProgids",
                "/v",
                "MDNotes.Markdown",
                "/t",
                "REG_NONE",
                "/f",
            ],
            vec!["add", &application_key, "/ve", "/d", "MD Notes", "/f"],
            vec!["add", &application_command_key, "/ve", "/d", &command, "/f"],
            vec![
                "add",
                &application_types_key,
                "/v",
                ".md",
                "/t",
                "REG_SZ",
                "/d",
                "",
                "/f",
            ],
            vec![
                "add",
                &application_types_key,
                "/v",
                ".markdown",
                "/t",
                "REG_SZ",
                "/d",
                "",
                "/f",
            ],
        ];
        for args in registrations {
            let status = Command::new("reg.exe")
                .args(args)
                .status()
                .map_err(|error| error.to_string())?;
            if !status.success() {
                return Err("Windows could not register MD Notes for Markdown files".into());
            }
        }
        let _ = Command::new("explorer.exe")
            .arg("ms-settings:defaultapps")
            .spawn();
        Ok("systemChoice".into())
    }

    #[cfg(target_os = "linux")]
    {
        let exe = std::env::current_exe().map_err(|error| error.to_string())?;
        let home = std::env::var_os("HOME").ok_or_else(|| "HOME is not available".to_string())?;
        let applications = PathBuf::from(home).join(".local/share/applications");
        std::fs::create_dir_all(&applications).map_err(|error| error.to_string())?;
        let desktop_path = applications.join("md-notes.desktop");
        let exe_text = exe.display().to_string().replace('"', "\\\"");
        let desktop = format!(
            "[Desktop Entry]\nType=Application\nName=MD Notes\nExec=\"{exe_text}\" %f\nMimeType=text/markdown;text/x-markdown;\nNoDisplay=true\nTerminal=false\n"
        );
        std::fs::write(&desktop_path, desktop).map_err(|error| error.to_string())?;
        let _ = Command::new("update-desktop-database")
            .arg(&applications)
            .status();
        for mime in ["text/markdown", "text/x-markdown"] {
            let status = Command::new("xdg-mime")
                .args(["default", "md-notes.desktop", mime])
                .status()
                .map_err(|error| error.to_string())?;
            if !status.success() {
                return Err(format!("xdg-mime could not set MD Notes for {mime}"));
            }
        }
        Ok("defaultSet".into())
    }

    #[cfg(target_os = "macos")]
    {
        set_macos_markdown_default()?;
        Ok("defaultSet".into())
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    Ok("unsupported".into())
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
    let vault = encryption::open_any(&path)?;
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

#[tauri::command]
fn save_git_credentials(username: String, token: String) -> CmdResult<()> {
    mobile::secret_store::save_git_credentials(username, token)
}

#[tauri::command]
fn has_git_credentials() -> CmdResult<bool> {
    mobile::secret_store::has_git_credentials()
}

#[tauri::command]
fn clear_git_credentials() -> CmdResult<()> {
    mobile::secret_store::clear_git_credentials()
}

#[tauri::command]
fn git_fetch_with_stored_credentials(
    remote_name: Option<String>,
    state: State<'_, AppState>,
) -> CmdResult<()> {
    let vault_root = {
        let guard = state.session.lock().map_err(|e| e.to_string())?;
        let session = guard
            .as_ref()
            .ok_or_else(|| "no vault is open".to_string())?;
        session.vault.root().to_path_buf()
    };
    let credentials = mobile::secret_store::load_git_credentials()?
        .ok_or_else(|| "Git HTTPS credentials are not configured".to_string())?;
    fetch_git_remote_authenticated(&vault_root, remote_name.as_deref(), &credentials)
        .map_err(|e| e.to_string())
}

/// Pushes the current branch of the open vault with the stored HTTPS
/// credentials (`"pushed"` or `"upToDate"`). Uses the system `git`, so it is
/// desktop only; encrypted vaults are checked by the push guard first.
/// Imports a plain folder into the open vault (encrypted on the way in when
/// the vault is encrypted). Existing files are never overwritten.
#[tauri::command]
fn import_folder(
    source: String,
    overwrite: bool,
    state: State<'_, AppState>,
) -> CmdResult<notes_core::ImportReport> {
    with_session(&state, |s| {
        s.vault.import_folder(Path::new(&source), overwrite)
    })
}

/// Lists groups of notes that probably describe the same subject (a read-only
/// hint; nothing is changed, ADR-0008).
#[tauri::command]
fn possible_duplicates(state: State<'_, AppState>) -> CmdResult<Vec<notes_core::DuplicateGroup>> {
    with_session(&state, |s| s.vault.possible_duplicates())
}

/// Closes the application (the File menu's Quit).
#[tauri::command]
fn quit_app(app: AppHandle) {
    app.exit(0);
}

#[tauri::command]
fn git_push_with_stored_credentials(
    remote_name: Option<String>,
    state: State<'_, AppState>,
) -> CmdResult<&'static str> {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let _ = (remote_name, state);
        Err("Git push is available on desktop only".into())
    }
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let vault_root = {
            let guard = state.session.lock().map_err(|e| e.to_string())?;
            let session = guard
                .as_ref()
                .ok_or_else(|| "no vault is open".to_string())?;
            session.vault.root().to_path_buf()
        };
        let credentials = mobile::secret_store::load_git_credentials()?
            .ok_or_else(|| "Git HTTPS credentials are not configured".to_string())?;
        match notes_core::push_git_branch(&vault_root, remote_name.as_deref(), &credentials)
            .map_err(|e| e.to_string())?
        {
            notes_core::GitPushOutcome::Pushed => Ok("pushed"),
            notes_core::GitPushOutcome::UpToDate => Ok("upToDate"),
        }
    }
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
    // Encrypted vaults store notes as `<name>.age`; the UI works with logical names.
    let changed: Vec<String> = changed
        .into_iter()
        .map(|path| match path.strip_suffix(".age") {
            Some(logical) => logical.to_string(),
            None => path,
        })
        .collect();
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
    with_session(&state, |s| {
        let graph = knowledge_graph(&s.index, focus.as_deref())?;
        notes_core::with_indexed_relations(&s.index, graph)
    })
}

/// Chain of typed relations above ("up": owners, sources) or below ("down": what
/// the note owns) a note, with the weakest-link grade of every chain.
#[tauri::command]
fn relation_tree_for(
    root: String,
    direction: String,
    max_depth: u8,
    state: State<'_, AppState>,
) -> CmdResult<Option<notes_core::RelationTree>> {
    let direction = notes_core::TreeDirection::parse(&direction)
        .ok_or_else(|| "direction must be \"up\" or \"down\"".to_string())?;
    with_session(&state, |s| {
        let graph = knowledge_graph(&s.index, None)?;
        let graph = notes_core::with_indexed_relations(&s.index, graph)?;
        Ok(notes_core::relation_tree(
            &graph, &root, direction, max_depth,
        ))
    })
}

/// Chain of typed relations between two notes (direction ignored): the shortest one,
/// or with `most_reliable` the one whose weakest line is the best.
#[tauri::command]
fn relation_path_between(
    from: String,
    to: String,
    most_reliable: Option<bool>,
    state: State<'_, AppState>,
) -> CmdResult<Option<notes_core::RelationPath>> {
    with_session(&state, |s| {
        let graph = knowledge_graph(&s.index, None)?;
        let graph = notes_core::with_indexed_relations(&s.index, graph)?;
        Ok(notes_core::relation_path_by(
            &graph,
            &from,
            &to,
            8,
            most_reliable.unwrap_or(false),
        ))
    })
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

/// Facts about a legal entity (8-digit code) from a local copy of the open-data folder of
/// the "З-під Ляди" site; read-only, nothing leaves the computer.
#[tauri::command]
async fn registry_facts(code: String, data_dir: String) -> CmdResult<notes_core::RegistryFacts> {
    tauri::async_runtime::spawn_blocking(move || {
        notes_core::registry_facts(std::path::Path::new(&data_dir), &code)
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())
}

/// Exact lookup by code (8 or 10 digits) or by name (candidates), for the search panel.
#[tauri::command]
fn lookup_subject(
    query: String,
    limit: u32,
    state: State<'_, AppState>,
) -> CmdResult<Vec<notes_core::LookupHit>> {
    with_session(&state, |s| s.index.lookup(&query, limit))
}

#[tauri::command]
fn create_backup(destination: String, state: State<'_, AppState>) -> CmdResult<BackupSummary> {
    with_session(&state, |session| {
        create_vault_backup(session.vault.root(), Path::new(&destination))
    })
}

#[tauri::command]
fn restore_backup(archive: String, destination_parent: String) -> CmdResult<RestoreSummary> {
    restore_vault_backup(Path::new(&archive), Path::new(&destination_parent))
        .map_err(|error| error.to_string())
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
            open_markdown_file,
            save_markdown_file,
            startup_markdown_files,
            configure_markdown_association,
            open_vault,
            create_vault,
            encryption::encryption_status,
            encryption::create_encrypted_vault_cmd,
            encryption::import_encryption_key,
            encryption::add_encryption_recipient,
            encryption::push_safety,
            save_git_credentials,
            has_git_credentials,
            clear_git_credentials,
            git_fetch_with_stored_credentials,
            git_push_with_stored_credentials,
            import_folder,
            possible_duplicates,
            lookup_subject,
            registry_facts,
            relation_tree_for,
            relation_path_between,
            quit_app,
            git::git_status_cmd,
            git::git_init_repo,
            git::git_set_remote_url,
            git::git_commit_all,
            git::git_pull_with_stored_credentials,
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
            create_backup,
            restore_backup,
            import_attachment,
            list_attachments,
            attachment_used_by,
            orphan_attachments
        ])
        .build(tauri::generate_context!())
        .expect("error while building MD Notes")
        .run(|_app, _event| {
            #[cfg(target_os = "macos")]
            let (app, event) = (_app, _event);
            #[cfg(target_os = "macos")]
            if let tauri::RunEvent::Opened { urls } = event {
                let paths = urls
                    .into_iter()
                    .filter_map(|url| url.to_file_path().ok())
                    .filter(|path| {
                        path.extension()
                            .and_then(|value| value.to_str())
                            .is_some_and(|ext| {
                                matches!(ext.to_ascii_lowercase().as_str(), "md" | "markdown")
                            })
                    })
                    .map(|path| path.display().to_string())
                    .collect::<Vec<_>>();
                if paths.is_empty() {
                    return;
                }
                if let Ok(mut pending) = app.state::<AppState>().opened_markdown.lock() {
                    pending.extend(paths.clone());
                }
                let _ = app.emit("opened-markdown-files", paths);
            }
        });
}
