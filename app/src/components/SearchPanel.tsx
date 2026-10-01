import { useEffect, useState, type ReactNode } from "react";
import { api, MATCH_END, MATCH_START, type SearchHit } from "../api";

interface Props {
  /** Changes whenever the index changes, so results are refreshed. */
  refreshKey: number;
  onOpen: (path: string) => void;
}

export function SearchPanel({ refreshKey, onOpen }: Props) {
  const [query, setQuery] = useState("");
  const [hits, setHits] = useState<SearchHit[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!query.trim()) {
      setHits([]);
      return;
    }
    let cancelled = false;
    const timer = window.setTimeout(() => {
      api
        .search(query)
        .then((result) => {
          if (cancelled) return;
          setHits(result);
          setError(null);
        })
        .catch((e) => {
          if (!cancelled) setError(String(e));
        });
    }, 200);
    return () => {
      cancelled = true;
      window.clearTimeout(timer);
    };
  }, [query, refreshKey]);

  return (
    <div className="panel">
      <input
        className="panel-input"
        type="search"
        placeholder="Пошук у нотатках…"
        aria-label="Пошук у нотатках"
        value={query}
        autoFocus
        onChange={(e) => setQuery(e.target.value)}
      />
      {error && <p className="panel-note warn">{error}</p>}
      {query.trim() !== "" && hits.length === 0 && !error && <p className="panel-note">Нічого не знайдено.</p>}
      <ul className="result-list">
        {hits.map((hit) => (
          <li key={hit.path}>
            <button type="button" className="result" onClick={() => onOpen(hit.path)} title={hit.path}>
              <span className="result-title">{hit.title}</span>
              {hit.snippet && (
                <span className="result-snippet">
                  <Highlighted text={hit.snippet} />
                </span>
              )}
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
}

/** Renders a snippet, turning MATCH_START…MATCH_END ranges into <mark>. */
export function Highlighted({ text }: { text: string }) {
  const nodes: ReactNode[] = [];
  text.split(MATCH_START).forEach((chunk, i) => {
    if (i === 0) {
      nodes.push(chunk);
      return;
    }
    const end = chunk.indexOf(MATCH_END);
    if (end < 0) {
      nodes.push(chunk);
      return;
    }
    nodes.push(<mark key={i}>{chunk.slice(0, end)}</mark>, chunk.slice(end + 1));
  });
  return <>{nodes}</>;
}
