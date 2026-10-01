import { useCallback, useEffect, useRef, useState } from "react";
import { ask, open } from "@tauri-apps/plugin-dialog";
import { api, joinPath, parentPath, type Note, type TreeEntry, type VaultInfo } from "./api";
import { Editor } from "./components/Editor";
import { FileTree } from "./components/FileTree";
import { NameDialog } from "./components/NameDialog";
import { Preview } from "./components/Preview";
import { storage, useStoredState } from "./storage";

type ViewMode = "edit" | "split" | "preview";
type Theme = "system" | "light" | "dark";
type DialogState =
  | { kind: "note"; dir: string }
  | { kind: "folder"; dir: string }
  | { kind: "rename"; entry: TreeEntry };

const AUTOSAVE_MS = 800;
const LAST_VAULT_KEY = "mdnotes.lastVault";
const isNarrow = () => window.matchMedia("(max-width: 720px)").matches;

export default function App() {
  const [vault, setVault] = useState<VaultInfo | null>(null);
  const [tree, setTree] = useState<TreeEntry[]>([]);
  const [note, setNote] = useState<Note | null>(null);
  const [draft, setDraft] = useState("");
  const [selected, setSelected] = useState<TreeEntry | null>(null);
  const [dialog, setDialog] = useState<DialogState | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [sidebarOpen, setSidebarOpen] = useState(() => !isNarrow());
  const [mode, setMode] = useStoredState<ViewMode>("mdnotes.mode", isNarrow() ? "preview" : "split");
  const [theme, setTheme] = useStoredState<Theme>("mdnotes.theme", "system");

  // Refs give async callbacks the latest values without re-subscribing.
  const noteRef = useRef<Note | null>(null);
  const draftRef = useRef("");
  const savedRef = useRef("");
  const [savedContent, setSavedContent] = useState("");
  const dirty = note !== null && draft !== savedContent;

  const report = useCallback((e: unknown) => setError(String(e)), []);

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
    } catch (e) {
      report(e);
    }
  }, [report]);

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
      } catch (e) {
        storage.remove(LAST_VAULT_KEY);
        report(e);
      }
    },
    [save, showNote, report],
  );

  useEffect(() => {
    const last = storage.get(LAST_VAULT_KEY);
    if (last) void openVaultAt(last);
    // Only on startup.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const pickVault = async () => {
    const dir = await open({ directory: true, multiple: false, title: "Виберіть папку сховища" });
    if (typeof dir === "string") await openVaultAt(dir);
  };

  const openNote = useCallback(
    async (path: string) => {
      await save();
      try {
        showNote(await api.readNote(path));
        if (isNarrow()) setSidebarOpen(false);
      } catch (e) {
        report(e);
      }
    },
    [save, showNote, report],
  );

  const openLink = useCallback(
    async (target: string) => {
      const name = target.split("#")[0].trim();
      if (!name) return;
      try {
        const path = await api.resolveLink(name);
        if (path) {
          await openNote(path);
          return;
        }
        // Like in most wiki tools, following a link to a missing note creates it.
        const created = await api.createNote("", name);
        await refreshTree();
        await openNote(created.path);
      } catch (e) {
        report(e);
      }
    },
    [openNote, refreshTree, report],
  );

  // Autosave shortly after the user stops typing.
  useEffect(() => {
    if (!dirty) return;
    const timer = window.setTimeout(() => void save(), AUTOSAVE_MS);
    return () => window.clearTimeout(timer);
  }, [draft, dirty, save]);

  // Do not lose edits when the window is closed or reloaded.
  useEffect(() => {
    const flush = () => void save();
    window.addEventListener("beforeunload", flush);
    return () => window.removeEventListener("beforeunload", flush);
  }, [save]);

  const onChange = useCallback((value: string) => {
    draftRef.current = value;
    setDraft(value);
  }, []);

  const targetDir = () => {
    if (!selected) return "";
    return selected.kind === "dir" ? selected.path : parentPath(selected.path);
  };

  const submitDialog = async (value: string) => {
    if (!dialog) return;
    setDialog(null);
    try {
      if (dialog.kind === "note") {
        const created = await api.createNote(dialog.dir, value);
        await refreshTree();
        await openNote(created.path);
      } else if (dialog.kind === "folder") {
        await api.createFolder(dialog.dir, value);
        await refreshTree();
      } else {
        const { entry } = dialog;
        let name = value;
        if (entry.kind === "note" && !/\.(md|markdown)$/i.test(name)) name += ".md";
        await save();
        const moved = await api.renameEntry(entry.path, joinPath(parentPath(entry.path), name));
        await refreshTree();
        setSelected(null);
        const current = noteRef.current;
        if (current && current.path === entry.path) await openNote(moved);
        else if (current && current.path.startsWith(`${entry.path}/`)) {
          await openNote(moved + current.path.slice(entry.path.length));
        }
      }
    } catch (e) {
      report(e);
    }
  };

  const trashSelected = async () => {
    if (!selected) return;
    const confirmed = await ask(`Перемістити «${selected.name}» у кошик сховища (.mdnotes/trash)?`, {
      title: "Видалення",
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
    } catch (e) {
      report(e);
    }
  };

  if (!vault) {
    return (
      <div className="welcome">
        <h1>MD Notes</h1>
        <p>Особиста база знань у звичайних Markdown-файлах.</p>
        <button type="button" className="primary" onClick={() => void pickVault()}>
          Відкрити папку
        </button>
        {error && <p className="welcome-error">{error}</p>}
      </div>
    );
  }

  return (
    <div className="app">
      <header className="toolbar">
        <button type="button" className="icon" title="Бічна панель" onClick={() => setSidebarOpen((v) => !v)}>
          ☰
        </button>
        <span className="vault-name" title={vault.root}>
          {vault.name}
        </span>
        <span className="note-title">
          {note ? note.title : ""}
          {dirty && <span className="dirty" title="Є незбережені зміни" />}
        </span>
        <div className="segmented" role="group" aria-label="Режим">
          {(
            [
              ["edit", "Редактор"],
              ["split", "Поруч"],
              ["preview", "Перегляд"],
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
        <select value={theme} onChange={(e) => setTheme(e.target.value as Theme)} aria-label="Тема">
          <option value="system">Системна</option>
          <option value="light">Світла</option>
          <option value="dark">Темна</option>
        </select>
      </header>

      <div className="body">
        {sidebarOpen && (
          <aside className="sidebar">
            <div className="sidebar-actions">
              <button type="button" title="Нова нотатка" onClick={() => setDialog({ kind: "note", dir: targetDir() })}>
                + Нотатка
              </button>
              <button type="button" title="Нова папка" onClick={() => setDialog({ kind: "folder", dir: targetDir() })}>
                + Папка
              </button>
              <button
                type="button"
                title="Перейменувати"
                disabled={!selected}
                onClick={() => selected && setDialog({ kind: "rename", entry: selected })}
              >
                ✎
              </button>
              <button type="button" title="У кошик" disabled={!selected} onClick={() => void trashSelected()}>
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
            <div className="sidebar-footer">
              <button type="button" onClick={() => void pickVault()}>
                Інше сховище…
              </button>
            </div>
          </aside>
        )}

        <main className={`workspace mode-${mode}`}>
          {note ? (
            <>
              {mode !== "preview" && (
                <section className="pane pane-editor">
                  <Editor docKey={note.path} value={note.content} onChange={onChange} onSave={() => void save()} />
                </section>
              )}
              {mode !== "edit" && (
                <section className="pane pane-preview">
                  <Preview content={draft} onOpenLink={(target) => void openLink(target)} />
                </section>
              )}
            </>
          ) : (
            <div className="empty">Виберіть нотатку зліва або створіть нову.</div>
          )}
        </main>
      </div>

      {note && (
        <footer className="statusbar">
          <span>{note.path}</span>
          {note.tags.length > 0 && <span>{note.tags.map((tag) => `#${tag}`).join(" ")}</span>}
          <span>Посилань: {note.links.length}</span>
          {note.frontMatterError && <span className="warn">Помилка у властивостях: {note.frontMatterError}</span>}
          <span className="save-state">{dirty ? "Не збережено" : "Збережено"}</span>
        </footer>
      )}

      {dialog && (
        <NameDialog
          title={dialog.kind === "note" ? "Нова нотатка" : dialog.kind === "folder" ? "Нова папка" : "Перейменувати"}
          label="Назва"
          initial={dialog.kind === "rename" ? dialog.entry.name.replace(/\.(md|markdown)$/i, "") : ""}
          submitText={dialog.kind === "rename" ? "Перейменувати" : "Створити"}
          onSubmit={(value) => void submitDialog(value)}
          onCancel={() => setDialog(null)}
        />
      )}

      {error && (
        <div className="toast" role="alert" onClick={() => setError(null)}>
          {error}
        </div>
      )}
    </div>
  );
}
