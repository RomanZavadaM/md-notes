import { useCallback, useEffect, useState } from "react";
import { emit, listen } from "@tauri-apps/api/event";
import { VAULT_CHANGED } from "./api";
import { KnowledgeGraphPanel } from "./components/KnowledgeGraphPanel";
import { RelationTreePanel } from "./components/RelationTreePanel";
import { useI18n } from "./i18n";
import { GRAPH_STRINGS } from "./i18n/graph";
import { RELATION_TREE_STRINGS } from "./i18n/relationTree";
import { GRAPH_ACTIVE_NOTE, GRAPH_OPEN_NOTE, type GraphActiveNote, type GraphWindowMode } from "./graphWindow";
import "./GraphWindow.css";

function initialMode(value: string | null, hasNote: boolean): GraphWindowMode {
  if (value === "local" || value === "global" || value === "tree") return value === "global" || hasNote ? value : "global";
  return "global";
}

/**
 * The knowledge graph in its own window: more room, can sit on a second monitor.
 * It reads the same open vault as the main window (one session in the core) and
 * follows the note the user is working on; a click on a node opens it in the main window.
 */
export function GraphWindow() {
  const { language } = useI18n();
  const graphStrings = GRAPH_STRINGS[language];
  const treeStrings = RELATION_TREE_STRINGS[language];
  const params = new URLSearchParams(window.location.search);
  const [notePath, setNotePath] = useState(params.get("note") ?? "");
  const [mode, setMode] = useState<GraphWindowMode>(() => initialMode(params.get("mode"), (params.get("note") ?? "") !== ""));
  const [refreshKey, setRefreshKey] = useState(0);

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
      if (event.payload.path) setNotePath(event.payload.path);
      if (event.payload.mode) setMode(event.payload.mode);
    }).then((fn) => {
      if (cancelled) fn();
      else unlisteners.push(fn);
    });
    return () => {
      cancelled = true;
      unlisteners.forEach((fn) => fn());
    };
  }, []);

  const onOpen = useCallback((path: string) => {
    void emit(GRAPH_OPEN_NOTE, path);
  }, []);

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
        {notePath !== "" && <span className="graph-window-note">{notePath}</span>}
      </div>
      <div className="graph-window-body">
        {needsNote ? (
          <p className="panel-note">{graphStrings.windowNoNote}</p>
        ) : mode === "tree" ? (
          <RelationTreePanel notePath={notePath} refreshKey={refreshKey} onOpen={onOpen} />
        ) : (
          <KnowledgeGraphPanel mode={mode} notePath={notePath} refreshKey={refreshKey} onOpen={onOpen} />
        )}
      </div>
    </div>
  );
}
