import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { api, type Note, type TreeEntry } from "./api";
import { Editor } from "./components/Editor";
import { Preview } from "./components/Preview";
import { openEditorWindow } from "./editorWindowApi";
import { useI18n } from "./i18n";
import { useStoredState } from "./storage";

type ViewMode = "edit" | "split" | "preview";
const AUTOSAVE_MS = 800;

function flattenFiles(entries: TreeEntry[]): string[] {
  const files: string[] = [];
  const walk = (items: TreeEntry[]) => {
    for (const item of items) {
      if (item.kind === "dir") walk(item.children ?? []);
      else files.push(item.path);
    }
  };
  walk(entries);
  return files;
}

export function EditorWindow() {
  const { t } = useI18n();
  const params = useMemo(() => new URLSearchParams(window.location.search), []);
  const vaultRoot = params.get("root") ?? "";
  const initialPath = params.get("note") ?? "";
  const [path, setPath] = useState(initialPath);
  const [note, setNote] = useState<Note | null>(null);
  const [draft, setDraft] = useState("");
  const [savedContent, setSavedContent] = useState("");
  const [files, setFiles] = useState<string[]>([]);
  const [encrypted, setEncrypted] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [mode, setMode] = useStoredState<ViewMode>("mdnotes.mode", "split");
  const draftRef = useRef("");
  const savedRef = useRef("");
  const noteRef = useRef<Note | null>(null);
  const closingRef = useRef(false);

  const loadNote = useCallback(async (nextPath: string) => {
    try {
      const opened = await api.readNote(nextPath);
      setPath(nextPath);
      setNote(opened);
      noteRef.current = opened;
      setDraft(opened.content);
      draftRef.current = opened.content;
      setSavedContent(opened.content);
      savedRef.current = opened.content;
      document.title = opened.title;
    } catch (err) {
      setError(String(err));
    }
  }, []);

  const save = useCallback(async (): Promise<boolean> => {
    const current = noteRef.current;
    const content = draftRef.current;
    if (!current || content === savedRef.current) return true;
    try {
      const disk = await api.readNote(current.path);
      if (disk.content !== savedRef.current) {
        setError(t.externalChange);
        return false;
      }
      const saved = await api.saveNote(current.path, content);
      noteRef.current = saved;
      setNote(saved);
      savedRef.current = content;
      setSavedContent(content);
      return true;
    } catch (err) {
      setError(String(err));
      return false;
    }
  }, [t.externalChange]);

  useEffect(() => {
    if (!initialPath) {
      setError("Missing note path");
      return;
    }
    void loadNote(initialPath);
    void api.getTree().then((tree) => setFiles(flattenFiles(tree))).catch(() => setFiles([]));
    void api.encryptionStatus(vaultRoot)
      .then((status) => setEncrypted(status.encrypted))
      .catch(() => setEncrypted(false));
  }, [initialPath, loadNote, vaultRoot]);

  useEffect(() => {
    if (draft === savedContent) return;
    const timer = window.setTimeout(() => void save(), AUTOSAVE_MS);
    return () => window.clearTimeout(timer);
  }, [draft, savedContent, save]);

  useEffect(() => {
    const flush = () => void save();
    window.addEventListener("beforeunload", flush);
    return () => window.removeEventListener("beforeunload", flush);
  }, [save]);

  useEffect(() => {
    const appWindow = getCurrentWindow();
    let disposed = false;
    let unlisten: (() => void) | undefined;
    void appWindow.onCloseRequested(async (event) => {
      if (closingRef.current || draftRef.current === savedRef.current) return;
      event.preventDefault();
      if (!(await save())) return;
      closingRef.current = true;
      await appWindow.destroy();
    }).then((fn) => {
      if (disposed) fn();
      else unlisten = fn;
    });
    return () => {
      disposed = true;
      unlisten?.();
    };
  }, [save]);

  const onChange = useCallback((value: string) => {
    draftRef.current = value;
    setDraft(value);
  }, []);

  const openPathInWindow = useCallback(async (nextPath: string) => {
    if (!(await save())) return;
    try {
      const target = await api.readNote(nextPath);
      await openEditorWindow(nextPath, vaultRoot, target.title);
    } catch (err) {
      setError(String(err));
    }
  }, [save, vaultRoot]);

  const openLink = useCallback(async (target: string) => {
    const name = target.split("#")[0].trim();
    if (!name) return;
    try {
      const resolved = await api.resolveLink(name);
      if (resolved) await openPathInWindow(resolved);
    } catch (err) {
      setError(String(err));
    }
  }, [openPathInWindow]);

  if (!note) {
    return <div className="empty">{error ?? t.workspaceEmpty}</div>;
  }

  const dirty = draft !== savedContent;

  return (
    <div className="app editor-window">
      <header className="toolbar">
        <span className="note-title" title={path}>{note.title}</span>
        <div className="segmented" role="group" aria-label={t.modeGroup}>
          {([
            ["edit", t.modeEdit],
            ["split", t.modeSplit],
            ["preview", t.modePreview],
          ] as const).map(([value, label]) => (
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
      </header>

      <main className={`workspace mode-${mode}`}>
        {mode !== "preview" && (
          <section className="pane pane-editor">
            <Editor docKey={`${note.path}#detached`} value={draft} onChange={onChange} onSave={() => void save()} />
          </section>
        )}
        {mode !== "edit" && (
          <section className="pane pane-preview">
            <Preview
              content={draft}
              notePath={note.path}
              vaultRoot={vaultRoot}
              encrypted={encrypted}
              files={files}
              onOpenLink={(target) => void openLink(target)}
              onOpenPath={(nextPath) => void openPathInWindow(nextPath)}
            />
          </section>
        )}
      </main>

      <footer className="statusbar">
        <span>{note.path}</span>
        {note.tags.length > 0 && <span>{note.tags.map((tag) => `#${tag}`).join(" ")}</span>}
        <span>{t.statusLinks(note.links.length)}</span>
        <span className="save-state">{dirty ? t.statusUnsaved : t.statusSaved}</span>
      </footer>

      {error && (
        <div className="toast" role="alert" onClick={() => setError(null)}>
          {error}
        </div>
      )}
    </div>
  );
}
