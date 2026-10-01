import { useEffect, useState } from "react";
import { api, type Backlink, type Note } from "../api";
import { useI18n } from "../i18n";

interface Props {
  note: Note;
  refreshKey: number;
  onOpen: (path: string) => void;
  onOpenLink: (target: string) => void;
}

/** Right-hand panel: backlinks, outgoing links and tags of the open note. */
export function LinksPanel({ note, refreshKey, onOpen, onOpenLink }: Props) {
  const { t } = useI18n();
  const [backlinks, setBacklinks] = useState<Backlink[]>([]);

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

  return (
    <aside className="links-panel" aria-label={t.linksPanel}>
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
    </aside>
  );
}
