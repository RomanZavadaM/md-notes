import { useEffect, useMemo, useState } from "react";
import { api, type NoteSummary } from "../api";
import { useI18n } from "../i18n";

interface Props {
  onOpen: (path: string) => void;
  onCreate: (title: string) => void;
  onClose: () => void;
}

const LIMIT = 50;

/** Ranks a note for the query: lower is better, `null` means no match. */
function score(note: NoteSummary, q: string): number | null {
  const title = note.title.toLowerCase();
  if (title.startsWith(q)) return 0;
  if (title.includes(q)) return 1;
  if (note.aliases.some((a) => a.toLowerCase().includes(q))) return 2;
  if (note.path.toLowerCase().includes(q)) return 3;
  return null;
}

/** Ctrl/Cmd+O: jump to a note by title, alias or path, or create it. */
export function QuickSwitcher({ onOpen, onCreate, onClose }: Props) {
  const { t } = useI18n();
  const [notes, setNotes] = useState<NoteSummary[]>([]);
  const [query, setQuery] = useState("");
  const [active, setActive] = useState(0);

  useEffect(() => {
    api
      .listNotes()
      .then(setNotes)
      .catch(() => setNotes([]));
  }, []);

  const results = useMemo(() => {
    const q = query.trim().toLowerCase();
    if (!q) return notes.slice(0, LIMIT);
    return notes
      .map((note) => ({ note, rank: score(note, q) }))
      .filter((r): r is { note: NoteSummary; rank: number } => r.rank !== null)
      .sort((a, b) => a.rank - b.rank || a.note.title.localeCompare(b.note.title))
      .slice(0, LIMIT)
      .map((r) => r.note);
  }, [notes, query]);

  useEffect(() => setActive(0), [query]);

  const choose = (index: number) => {
    const note = results[index];
    if (note) onOpen(note.path);
    else if (query.trim()) onCreate(query.trim());
  };

  return (
    <div className="dialog-backdrop" onMouseDown={onClose}>
      <div className="dialog switcher" role="dialog" aria-label={t.switcherLabel} onMouseDown={(e) => e.stopPropagation()}>
        <input
          autoFocus
          placeholder={t.switcherPlaceholder}
          aria-label={t.switcherPlaceholder}
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Escape") onClose();
            else if (e.key === "ArrowDown") {
              e.preventDefault();
              setActive((i) => Math.min(i + 1, Math.max(results.length - 1, 0)));
            } else if (e.key === "ArrowUp") {
              e.preventDefault();
              setActive((i) => Math.max(i - 1, 0));
            } else if (e.key === "Enter") {
              e.preventDefault();
              choose(active);
            }
          }}
        />
        <ul className="result-list switcher-list">
          {results.map((note, i) => (
            <li key={note.path}>
              <button
                type="button"
                className={`result ${i === active ? "active" : ""}`}
                onMouseEnter={() => setActive(i)}
                onClick={() => choose(i)}
              >
                <span className="result-title">{note.title}</span>
                <span className="result-path">{note.path}</span>
              </button>
            </li>
          ))}
          {results.length === 0 && query.trim() && (
            <li>
              <button type="button" className="result active" onClick={() => choose(0)}>
                <span className="result-title">{t.switcherCreate(query.trim())}</span>
              </button>
            </li>
          )}
        </ul>
      </div>
    </div>
  );
}
