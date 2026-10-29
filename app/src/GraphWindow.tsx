import { useCallback, useEffect, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { api, VAULT_CHANGED, type Note } from "./api";
import Markdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { KnowledgeGraphPanel } from "./components/KnowledgeGraphPanel";
import { RelationTreePanel } from "./components/RelationTreePanel";
import { useI18n } from "./i18n";
import { GRAPH_STRINGS } from "./i18n/graph";
import { RELATION_TREE_STRINGS } from "./i18n/relationTree";
import { GRAPH_ACTIVE_NOTE, openGraphNoteInMain, type GraphActiveNote, type GraphWindowMode } from "./graphWindowApi";
import "./GraphWindow.css";

function initialMode(value: string | null, hasNote: boolean): GraphWindowMode {
  if (value === "local" || value === "global" || value === "tree") return value === "global" || hasNote ? value : "global";
  return "global";
}

/**
 * The knowledge graph in its own window: more room, can sit on a second monitor.
 * It reads the same open vault as the main window (one session in the core) and
 * follows the note the user is working on; node selection previews it here.
 */
export function GraphWindow() {
  const { language } = useI18n();
  const graphStrings = GRAPH_STRINGS[language];
  const treeStrings = RELATION_TREE_STRINGS[language];
  const params = new URLSearchParams(window.location.search);
  const [notePath, setNotePath] = useState(params.get("note") ?? "");
  const [mode, setMode] = useState<GraphWindowMode>(() => initialMode(params.get("mode"), (params.get("note") ?? "") !== ""));
  const [refreshKey, setRefreshKey] = useState(0);
  const [previewPath, setPreviewPath] = useState<string | null>(null);
  const [previewNote, setPreviewNote] = useState<Note | null>(null);
  const [previewError, setPreviewError] = useState<string | null>(null);
  // Історія активних нотаток цього вікна: дозволяє повернутися назад після переходу по вузлах.
  const historyRef = useRef<string[]>([]);
  const lastPathRef = useRef(params.get("note") ?? "");
  const [canBack, setCanBack] = useState(false);

  const followNote = useCallback((path: string) => {
    if (path === "" || path === lastPathRef.current) return;
    if (lastPathRef.current !== "") {
      historyRef.current = [...historyRef.current.slice(-49), lastPathRef.current];
      setCanBack(true);
    }
    lastPathRef.current = path;
    setNotePath(path);
  }, []);

  const goBack = useCallback(() => {
    const prev = historyRef.current[historyRef.current.length - 1];
    if (prev === undefined) return;
    historyRef.current = historyRef.current.slice(0, -1);
    setCanBack(historyRef.current.length > 0);
    lastPathRef.current = prev;
    setNotePath(prev);
    setPreviewPath(prev);
  }, []);

  useEffect(() => {
    document.title = graphStrings.windowTitle;
  }, [graphStrings.windowTitle]);

  useEffect(() => {
    let cancelled = false;
    const unlisteners: Array<() => void> = [];
    void listen<string[]>(VAULT_CHANGED, () => setRefreshKey((value) => value + 1)).then((fn) => {
      if (cancelled) fn();
      else unlisteners.push(fn);
    });
    void listen<GraphActiveNote>(GRAPH_ACTIVE_NOTE, (event) => {
      if (event.payload.path) followNote(event.payload.path);
      if (event.payload.mode) setMode(event.payload.mode);
    }).then((fn) => {
      if (cancelled) fn();
      else unlisteners.push(fn);
    });
    return () => {
      cancelled = true;
      unlisteners.forEach((fn) => fn());
    };
  }, [followNote]);

  const onOpen = useCallback((path: string) => {
    followNote(path);
    setPreviewPath(path);
    void openGraphNoteInMain(path);
  }, [followNote]);

  useEffect(() => {
    if (!previewPath) { setPreviewNote(null); setPreviewError(null); return; }
    let cancelled = false;
    setPreviewNote(null);
    setPreviewError(null);
    void api.readNote(previewPath).then((note) => {
      if (!cancelled) setPreviewNote(note);
    }).catch((error: unknown) => {
      if (!cancelled) setPreviewError(String(error));
    });
    return () => { cancelled = true; };
  }, [previewPath, refreshKey]);

  const needsNote = (mode === "local" || mode === "tree") && notePath === "";

  return (
    <div className="graph-window">
      <div className="tabs graph-tabs" role="tablist">
        {(
          [
            ["global", graphStrings.globalMode],
            ["local", graphStrings.localMode],
            ["tree", treeStrings.tab],
          ] as const
        ).map(([value, label]) => (
          <button
            key={value}
            type="button"
            role="tab"
            aria-selected={mode === value}
            className={mode === value ? "on" : ""}
            onClick={() => setMode(value)}
          >
            {label}
          </button>
        ))}
        {canBack && (
          <button type="button" className="graph-window-back" onClick={goBack} title={graphStrings.windowBack} aria-label={graphStrings.windowBack}>
            ←
          </button>
        )}
        {notePath !== "" && <span className="graph-window-note">{notePath}</span>}
      </div>
      <div className="graph-window-content">
      <div className="graph-window-body">
        {needsNote ? (
          <p className="panel-note">{graphStrings.windowNoNote}</p>
        ) : mode === "tree" ? (
          <RelationTreePanel notePath={notePath} refreshKey={refreshKey} onOpen={onOpen} />
        ) : (
          <KnowledgeGraphPanel mode={mode} notePath={notePath} refreshKey={refreshKey} onOpen={onOpen} />
        )}
      </div>
      {previewPath && (
        <aside className="graph-window-preview" aria-label={graphStrings.previewTitle}>
          <div className="graph-window-preview-header">
            <strong>{previewNote?.title ?? previewPath}</strong>
            <button type="button" onClick={() => void openGraphNoteInMain(previewPath)}>{graphStrings.previewEdit}</button>
            <button type="button" onClick={() => setPreviewPath(null)} aria-label={graphStrings.previewClose}>×</button>
          </div>
          {previewError ? <p role="alert">{previewError}</p> :
            previewNote ? <div className="graph-window-preview-body"><Markdown remarkPlugins={[remarkGfm]}>{previewNote.content}</Markdown></div> :
            <p>{graphStrings.loading}</p>}
        </aside>
      )}
      </div>
    </div>
  );
}
