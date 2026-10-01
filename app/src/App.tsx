import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { ask, open } from "@tauri-apps/plugin-dialog";
import {
  api,
  joinPath,
  parentPath,
  VAULT_CHANGED,
  type AttachmentInfo,
  type Note,
  type TemplateInfo,
  type TreeEntry,
  type VaultInfo,
} from "./api";
import { Editor } from "./components/Editor";
import { FileTree } from "./components/FileTree";
import { LinksPanel } from "./components/LinksPanel";
import { NameDialog } from "./components/NameDialog";
import { Preview } from "./components/Preview";
import { QuickSwitcher } from "./components/QuickSwitcher";
import { SearchPanel } from "./components/SearchPanel";
import { TagsPanel } from "./components/TagsPanel";
import { AttachmentsPanel } from "./components/AttachmentsPanel";
import { AboutDialog, COPYRIGHT } from "./components/AboutDialog";
import { LANGUAGES, useI18n, type LanguageCode } from "./i18n";
import { storage, useStoredState } from "./storage";

type ViewMode = "edit" | "split" | "preview";
type Theme = "system" | "light" | "dark";
type SidebarTab = "files" | "search" | "tags" | "attachments";
type DialogState =
  | { kind: "note"; dir: string }
  | { kind: "folder"; dir: string }
  | { kind: "rename"; entry: TreeEntry };

const AUTOSAVE_MS = 800;
const LAST_VAULT_KEY = "mdnotes.lastVault";
const IMAGE_ATTACHMENT = /\.(png|jpe?g|gif|webp|svg|bmp|avif)$/i;
const isNarrow = () => window.matchMedia("(max-width: 720px)").matches;

export default function App() {
  const { t, language, setLanguage } = useI18n();
  const [aboutOpen, setAboutOpen] = useState(false);
  const [vault, setVault] = useState<VaultInfo | null>(null);
  const [tree, setTree] = useState<TreeEntry[]>([]);
  const [note, setNote] = useState<Note | null>(null);
  const [draft, setDraft] = useState("");
  const [selected, setSelected] = useState<TreeEntry | null>(null);
  const [dialog, setDialog] = useState<DialogState | null>(null);
  const [switcherOpen, setSwitcherOpen] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [sidebarOpen, setSidebarOpen] = useState(() => !isNarrow());
  const [sidebarTab, setSidebarTab] = useState<SidebarTab>("files");
  const [mode, setMode] = useStoredState<ViewMode>("mdnotes.mode", isNarrow() ? "preview" : "split");
  const [theme, setTheme] = useStoredState<Theme>("mdnotes.theme", "system");
  const [linksPanel, setLinksPanel] = useStoredState<"on" | "off">("mdnotes.linksPanel", isNarrow() ? "off" : "on");
  const [refreshKey, setRefreshKey] = useState(0);
  const [editorVersion, setEditorVersion] = useState(0);

  const noteRef = useRef<Note | null>(null);
  const draftRef = useRef("");
  const savedRef = useRef("");
  const [savedContent, setSavedContent] = useState("");
  const dirty = note !== null && draft !== savedContent;

  const files = useMemo(() => {
    const out: string[] = [];
    const walk = (entries: TreeEntry[]) => {
      for (const entry of entries) {
        if (entry.kind === "dir") walk(entry.children ?? []);
        else out.push(entry.path);
      }
    };
    walk(tree);
    return out;
  }, [tree]);

  const report = useCallback((e: unknown) => setError(String(e)), []);
  const bump = useCallback(() => setRefreshKey((k) => k + 1), []);

  useEffect(() => {
    document.documentElement.dataset.theme = theme === "system" ? "" : theme;
  }, [theme]);

  const showNote = useCallback((n: Note | null) => {
    noteRef.current = n;
    draftRef.current = n?.content ?? "";
    savedRef.current = n?.content ?? "";
    setNote(n);
    setDraft(n?.content ?? "");
    setSavedContent(n?.content ?? "");
  }, []);

  const refreshTree = useCallback(async () => {
    try {
      setTree(await api.getTree());
    } catch (e) {
      report(e);
    }
  }, [report]);

  const save = useCallback(async () => {
    const current = noteRef.current;
    const content = draftRef.current;
    if (!current || content === savedRef.current) return;
    try {
      const saved = await api.saveNote(current.path, content);
      savedRef.current = content;
      setSavedContent(content);
      if (noteRef.current?.path === saved.path) {
        noteRef.current = saved;
        setNote(saved);
      }
      bump();
    } catch (e) {
      report(e);
    }
  }, [report, bump]);

  const openVaultAt = useCallback(
    async (path: string) => {
      try {
        await save();
        const info = await api.openVault(path);
        setVault(info);
        setSelected(null);
        showNote(null);
        storage.set(LAST_VAULT_KEY, path);
        setTree(await api.getTree());
        bump();
      } catch (e) {
        storage.remove(LAST_VAULT_KEY);
        report(e);
      }
    },
    [save, showNote, report, bump],
  );

  useEffect(() => {
    const last = storage.get(LAST_VAULT_KEY);
    if (last) void openVaultAt(last);
    // Only on startup.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const pickVault = async () => {
    const dir = await open({ directory: true, multiple: false, title: t.pickFolderTitle });
    if (typeof dir === "string") await openVaultAt(dir);
  };

  const openNote = useCallback(
    async (path: string) => {
      await save();
      try {
        showNote(await api.readNote(path));
        setEditorVersion((v) => v + 1);
        if (isNarrow()) setSidebarOpen(false);
      } catch (e) {
        report(e);
      }
    },
    [save, showNote, report],
  );

  const createAndOpen = useCallback(
    async (dir: string, title: string, template?: string) => {
      try {
        const created = await api.createNote(dir, title, template);
        await refreshTree();
        bump();
        await openNote(created.path);
      } catch (e) {
        report(e);
      }
    },
    [openNote, refreshTree, bump, report],
  );

  const openToday = useCallback(async () => {
    try {
      const daily = await api.openDaily();
      await refreshTree();
      bump();
      await openNote(daily.path);
    } catch (e) {
      report(e);
    }
  }, [openNote, refreshTree, bump, report]);

  const [templates, setTemplates] = useState<TemplateInfo[]>([]);
  useEffect(() => {
    if (!vault) return;
    api
      .listTemplates()
      .then(setTemplates)
      .catch(() => setTemplates([]));
  }, [vault, refreshKey]);
  const templateChoices = [
    { value: "", label: t.emptyTemplate },
    ...templates.filter((tpl) => tpl.name !== "daily").map((tpl) => ({ value: tpl.name, label: tpl.label })),
  ];

  const openLink = useCallback(
    async (target: string) => {
      const name = target.split("#")[0].trim();
      if (!name) return;
      try {
        const path = await api.resolveLink(name);
        if (path) await openNote(path);
        else await createAndOpen("", name);
      } catch (e) {
        report(e);
      }
    },
    [openNote, createAndOpen, report],
  );

  const onExternalChange = useCallback(
    async (paths: string[]) => {
      await refreshTree();
      bump();
      const current = noteRef.current;
      if (!current || !paths.includes(current.path)) return;
      try {
        const fresh = await api.readNote(current.path);
        if (fresh.content === savedRef.current) return;
        if (draftRef.current !== savedRef.current) {
          setError(t.externalChange);
          return;
        }
        showNote(fresh);
        setEditorVersion((v) => v + 1);
      } catch {
        showNote(null);
      }
    },
    [refreshTree, bump, showNote, t],
  );

  useEffect(() => {
    if (!vault) return;
    let disposed = false;
    let unlisten: (() => void) | undefined;
    void listen<string[]>(VAULT_CHANGED, (event) => void onExternalChange(event.payload)).then((fn) => {
      if (disposed) fn();
      else unlisten = fn;
    });
    return () => {
      disposed = true;
      unlisten?.();
    };
  }, [vault, onExternalChange]);

  useEffect(() => {
    if (!dirty) return;
    const timer = window.setTimeout(() => void save(), AUTOSAVE_MS);
    return () => window.clearTimeout(timer);
  }, [draft, dirty, save]);

  useEffect(() => {
    const flush = () => void save();
    window.addEventListener("beforeunload", flush);
    return () => window.removeEventListener("beforeunload", flush);
  }, [save]);

  useEffect(() => {
    if (!vault) return;
    const onKey = (e: KeyboardEvent) => {
      const mod = e.ctrlKey || e.metaKey;
      if (!mod) return;
      const key = e.key.toLowerCase();
      if (!e.shiftKey && (key === "o" || key === "p")) {
        e.preventDefault();
        setSwitcherOpen(true);
      } else if (e.shiftKey && key === "f") {
        e.preventDefault();
        setSidebarOpen(true);
        setSidebarTab("search");
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [vault]);

  const onChange = useCallback((value: string) => {
    draftRef.current = value;
    setDraft(value);
  }, []);

  const insertAttachment = useCallback(
    (attachment: AttachmentInfo) => {
      if (!noteRef.current) return;
      const encodedPath = `/${attachment.path.split("/").map(encodeURIComponent).join("/")}`;
      const link = IMAGE_ATTACHMENT.test(attachment.name)
        ? `![${attachment.name}](<${encodedPath}>)`
        : `[${attachment.name}](<${encodedPath}>)`;
      const current = draftRef.current;
      const separator = current.length === 0 || current.endsWith("\n") ? "" : "\n";
      const next = `${current}${separator}${link}\n`;
      onChange(next);
      setError(t.attachmentInserted(attachment.name));
    },
    [onChange, t],
  );

  const targetDir = () => {
    if (!selected) return "";
    return selected.kind === "dir" ? selected.path : parentPath(selected.path);
  };

  const submitDialog = async (value: string, template: string) => {
    if (!dialog) return;
    setDialog(null);
    try {
      if (dialog.kind === "note") {
        await createAndOpen(dialog.dir, value, template || undefined);
      } else if (dialog.kind === "folder") {
        await api.createFolder(dialog.dir, value);
        await refreshTree();
      } else {
        const { entry } = dialog;
        let name = value;
        if (entry.kind === "note" && !/\.(md|markdown)$/i.test(name)) name += ".md";
        await save();
        const outcome = await api.renameEntry(entry.path, joinPath(parentPath(entry.path), name));
        const moved = outcome.path;
        await refreshTree();
        bump();
        setSelected(null);
        const current = noteRef.current;
        if (current && current.path === entry.path) await openNote(moved);
        else if (current && current.path.startsWith(`${entry.path}/`)) {
          await openNote(moved + current.path.slice(entry.path.length));
        } else if (current && outcome.updated.includes(current.path)) {
          await openNote(current.path);
        }
        if (outcome.updated.length > 0) {
          setError(t.linksUpdated(outcome.updated.length));
        }
      }
    } catch (e) {
      report(e);
    }
  };

  const trashSelected = async () => {
    if (!selected) return;
    const confirmed = await ask(t.trashConfirm(selected.name), {
      title: t.trashTitle,
      kind: "warning",
    });
    if (!confirmed) return;
    try {
      await api.trashEntry(selected.path);
      const current = noteRef.current;
      if (current && (current.path === selected.path || current.path.startsWith(`${selected.path}/`))) {
        showNote(null);
      }
      setSelected(null);
      await refreshTree();
      bump();
    } catch (e) {
      report(e);
    }
  };

  if (!vault) {
    return (
      <div className="welcome">
        <h1>MD Notes</h1>
        <p>{t.appTagline}</p>
        <button type="button" className="primary" onClick={() => void pickVault()}>
          {t.openFolder}
        </button>
        {error && <p className="welcome-error">{error}</p>}
        <select
          className="welcome-language"
          value={language}
          onChange={(e) => setLanguage(e.target.value as LanguageCode)}
          aria-label={t.language}
        >
          {LANGUAGES.map((lang) => (
            <option key={lang.code} value={lang.code} title={lang.ukrainianDescription}>
              {lang.flag} {lang.nativeName}
            </option>
          ))}
        </select>
        <button type="button" className="link-button" onClick={() => setAboutOpen(true)}>
          {t.about}
        </button>
        <p className="welcome-copyright">{COPYRIGHT}</p>
        {aboutOpen && <AboutDialog onClose={() => setAboutOpen(false)} />}
      </div>
    );
  }

  return (
    <div className="app">
      <header className="toolbar">
        <button type="button" className="icon" title={t.sidebarToggle} onClick={() => setSidebarOpen((v) => !v)}>
          ☰
        </button>
        <span className="vault-name" title={vault.root}>
          {vault.name}
        </span>
        <button type="button" className="search-button" title={t.goToNote} onClick={() => setSwitcherOpen(true)}>
          <span className="note-title">
            {note ? note.title : t.goToNotePlaceholder}
            {dirty && <span className="dirty" title={t.unsavedChanges} />}
          </span>
        </button>
        <div className="segmented" role="group" aria-label={t.modeGroup}>
          {(
            [
              ["edit", t.modeEdit],
              ["split", t.modeSplit],
              ["preview", t.modePreview],
            ] as const
          ).map(([value, label]) => (
            <button
              key={value}
              type="button"
              className={`${mode === value ? "on" : ""} mode-${value}`}
              onClick={() => setMode(value)}
            >
              {label}
            </button>
          ))}
        </div>
        <button
          type="button"
          className={`icon ${linksPanel === "on" ? "on" : ""}`}
          title={t.linksPanelToggle}
          onClick={() => setLinksPanel(linksPanel === "on" ? "off" : "on")}
        >
          ⇆
        </button>
        <select value={theme} onChange={(e) => setTheme(e.target.value as Theme)} aria-label={t.theme}>
          <option value="system">{t.themeSystem}</option>
          <option value="light">{t.themeLight}</option>
          <option value="dark">{t.themeDark}</option>
        </select>
        <select
          value={language}
          onChange={(e) => setLanguage(e.target.value as LanguageCode)}
          aria-label={t.language}
          title={t.language}
        >
          {LANGUAGES.map((lang) => (
            <option key={lang.code} value={lang.code} title={lang.ukrainianDescription}>
              {lang.flag} {lang.nativeName}
            </option>
          ))}
        </select>
        <button type="button" className="icon" title={t.about} onClick={() => setAboutOpen(true)}>
          ⓘ
        </button>
      </header>

      <div className="body">
        {sidebarOpen && (
          <aside className="sidebar">
            <div className="tabs" role="tablist">
              {(
                [
                  ["files", t.tabFiles],
                  ["search", t.tabSearch],
                  ["tags", t.tabTags],
                  ["attachments", t.tabAttachments],
                ] as const
              ).map(([value, label]) => (
                <button
                  key={value}
                  type="button"
                  role="tab"
                  aria-selected={sidebarTab === value}
                  className={sidebarTab === value ? "on" : ""}
                  onClick={() => setSidebarTab(value)}
                >
                  {label}
                </button>
              ))}
            </div>

            {sidebarTab === "files" && (
              <>
                <div className="sidebar-actions">
                  <button type="button" title={t.newNote} onClick={() => setDialog({ kind: "note", dir: targetDir() })}>
                    {t.newNoteButton}
                  </button>
                  <button type="button" title={t.todayTitle} onClick={() => void openToday()}>
                    {t.today}
                  </button>
                  <button type="button" title={t.newFolder} onClick={() => setDialog({ kind: "folder", dir: targetDir() })}>
                    {t.newFolderButton}
                  </button>
                  <button
                    type="button"
                    title={t.rename}
                    disabled={!selected}
                    onClick={() => selected && setDialog({ kind: "rename", entry: selected })}
                  >
                    ✎
                  </button>
                  <button type="button" title={t.moveToTrash} disabled={!selected} onClick={() => void trashSelected()}>
                    🗑
                  </button>
                </div>
                <FileTree
                  entries={tree}
                  selectedPath={selected?.path ?? null}
                  activePath={note?.path ?? null}
                  onSelect={setSelected}
                  onOpen={(entry) => {
                    if (entry.kind === "note") void openNote(entry.path);
                  }}
                />
              </>
            )}
            {sidebarTab === "search" && <SearchPanel refreshKey={refreshKey} onOpen={(p) => void openNote(p)} />}
            {sidebarTab === "tags" && <TagsPanel refreshKey={refreshKey} onOpen={(p) => void openNote(p)} />}
            {sidebarTab === "attachments" && (
              <AttachmentsPanel
                vaultRoot={vault.root}
                refreshKey={refreshKey}
                canInsert={note !== null}
                onInserted={insertAttachment}
                onChanged={() => {
                  void refreshTree();
                  bump();
                }}
                onError={report}
              />
            )}

            <div className="sidebar-footer">
              <button type="button" onClick={() => void pickVault()}>
                {t.otherVault}
              </button>
            </div>
          </aside>
        )}

        <main className={`workspace mode-${mode}`}>
          {note ? (
            <>
              {mode !== "preview" && (
                <section className="pane pane-editor">
                  <Editor
                    docKey={`${note.path}#${editorVersion}`}
                    value={note.content}
                    onChange={onChange}
                    onSave={() => void save()}
                  />
                </section>
              )}
              {mode !== "edit" && (
                <section className="pane pane-preview">
                  <Preview
                    content={draft}
                    notePath={note.path}
                    vaultRoot={vault.root}
                    files={files}
                    onOpenLink={(target) => void openLink(target)}
                    onOpenPath={(path) => void openNote(path)}
                  />
                </section>
              )}
            </>
          ) : (
            <div className="empty">{t.workspaceEmpty}</div>
          )}
        </main>

        {linksPanel === "on" && note && (
          <LinksPanel
            note={note}
            refreshKey={refreshKey}
            onOpen={(p) => void openNote(p)}
            onOpenLink={(target) => void openLink(target)}
          />
        )}
      </div>

      {note && (
        <footer className="statusbar">
          <span>{note.path}</span>
          {note.tags.length > 0 && <span>{note.tags.map((tag) => `#${tag}`).join(" ")}</span>}
          <span>{t.statusLinks(note.links.length)}</span>
          {note.frontMatterError && <span className="warn">{t.statusPropertyError(note.frontMatterError)}</span>}
          <span className="save-state">{dirty ? t.statusUnsaved : t.statusSaved}</span>
        </footer>
      )}

      {dialog && (
        <NameDialog
          title={dialog.kind === "note" ? t.newNote : dialog.kind === "folder" ? t.newFolder : t.rename}
          label={t.dialogName}
          initial={dialog.kind === "rename" ? dialog.entry.name.replace(/\.(md|markdown)$/i, "") : ""}
          submitText={dialog.kind === "rename" ? t.rename : t.create}
          choices={dialog.kind === "note" ? templateChoices : undefined}
          choiceLabel={t.dialogTemplate}
          onSubmit={(value, choice) => void submitDialog(value, choice)}
          onCancel={() => setDialog(null)}
        />
      )}

      {switcherOpen && (
        <QuickSwitcher
          onOpen={(path) => {
            setSwitcherOpen(false);
            void openNote(path);
          }}
          onCreate={(title) => {
            setSwitcherOpen(false);
            void createAndOpen("", title);
          }}
          onClose={() => setSwitcherOpen(false)}
        />
      )}

      {aboutOpen && <AboutDialog onClose={() => setAboutOpen(false)} />}

      {error && (
        <div className="toast" role="alert" onClick={() => setError(null)}>
          {error}
        </div>
      )}
    </div>
  );
}
