import { useEffect, useState } from "react";
import { api, type Backlink, type Note } from "../api";
import { useI18n } from "../i18n";
import { GRAPH_STRINGS } from "../i18n/graph";
import { KnowledgeGraphPanel } from "./KnowledgeGraphPanel";
import { RelationTreePanel } from "./RelationTreePanel";
import { openGraphWindow } from "../graphWindowApi";
import { RELATION_TREE_STRINGS } from "../i18n/relationTree";

interface Props {
  note: Note;
  refreshKey: number;
  onOpen: (path: string) => void;
  onOpenLink: (target: string) => void;
}

type PanelMode = "links" | "local" | "global" | "tree";

/** Right-hand panel: textual links plus local/global knowledge graph views. */
export function LinksPanel({ note, refreshKey, onOpen, onOpenLink }: Props) {
  const { t, language } = useI18n();
  const graphStrings = GRAPH_STRINGS[language];
  const [backlinks, setBacklinks] = useState<Backlink[]>([]);
  const [mode, setMode] = useState<PanelMode>("links");
  const [desktop, setDesktop] = useState(false);

  useEffect(() => {
    api
      .runtimePlatform()
      .then((platform) => setDesktop(platform === "windows" || platform === "linux" || platform === "macos"))
      .catch(() => setDesktop(false));
  }, []);

  useEffect(() => {
    let cancelled = false;
    api
      .backlinks(note.path)
      .then((result) => {
        if (!cancelled) setBacklinks(result);
      })
      .catch(() => {
        if (!cancelled) setBacklinks([]);
      });
    return () => {
      cancelled = true;
    };
  }, [note.path, refreshKey]);

  const outgoing = [...new Set(note.links.filter((l) => l.target).map((l) => l.target))];

  const detachGraph = (graphMode: "local" | "global" | "tree") => {
    void openGraphWindow(graphMode, note.path, graphStrings.windowTitle);
  };

  return (
    <aside className="links-panel" aria-label={t.linksPanel}>
      <div className="tabs graph-tabs" role="tablist">
        {(
          [
            ["links", graphStrings.linksMode],
            ["local", graphStrings.localMode],
            ["global", graphStrings.globalMode],
            ["tree", RELATION_TREE_STRINGS[language].tab],
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
        {desktop && mode !== "links" && (
          <button
            type="button"
            className="graph-detach"
            title={graphStrings.detach}
            onClick={() => detachGraph(mode)}
          >
            ↗
          </button>
        )}
      </div>

      {mode === "links" ? (
        <>
          <section>
            <h3>
              {t.backlinks} <span className="count">{backlinks.length}</span>
            </h3>
            {backlinks.length === 0 && <p className="panel-note">{t.noBacklinks}</p>}
            <ul className="result-list">
              {backlinks.map((b, i) => (
                <li key={`${b.path}-${i}`}>
                  <button type="button" className="result" onClick={() => onOpen(b.path)} title={b.path}>
                    <span className="result-title">{b.title}</span>
                    {b.context && <span className="result-snippet">{b.context}</span>}
                  </button>
                </li>
              ))}
            </ul>
          </section>
          <section>
            <h3>
              {t.outgoingLinks} <span className="count">{outgoing.length}</span>
            </h3>
            <ul className="result-list">
              {outgoing.map((target) => (
                <li key={target}>
                  <button type="button" className="result" onClick={() => onOpenLink(target)}>
                    <span className="result-title">{target}</span>
                  </button>
                </li>
              ))}
            </ul>
          </section>
          {note.tags.length > 0 && (
            <section>
              <h3>{t.tags}</h3>
              <p className="tag-line">{note.tags.map((tag) => `#${tag}`).join(" ")}</p>
            </section>
          )}
        </>
      ) : mode === "tree" ? (
        <RelationTreePanel notePath={note.path} refreshKey={refreshKey} onOpen={onOpen} />
      ) : (
        <KnowledgeGraphPanel
          mode={mode}
          notePath={note.path}
          refreshKey={refreshKey}
          onOpen={onOpen}
          onExpand={
            desktop
              ? () => detachGraph(mode)
              : undefined
          }
        />
      )}
    </aside>
  );
}
