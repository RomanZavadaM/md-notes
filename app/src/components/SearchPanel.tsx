import { useEffect, useState, type ReactNode } from "react";
import { api, MATCH_END, MATCH_START, type LookupHit, type SearchHit } from "../api";
import { useI18n } from "../i18n";
import { LOOKUP_STRINGS } from "../i18n/lookup";

interface Props {
  /** Changes whenever the index changes, so results are refreshed. */
  refreshKey: number;
  onOpen: (path: string) => void;
}

export function SearchPanel({ refreshKey, onOpen }: Props) {
  const { t, language } = useI18n();
  const lookupStrings = LOOKUP_STRINGS[language];
  const [lookup, setLookup] = useState<LookupHit[]>([]);
  const [query, setQuery] = useState("");
  const [hits, setHits] = useState<SearchHit[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!query.trim()) {
      setHits([]);
      setLookup([]);
      return;
    }
    let cancelled = false;
    const timer = window.setTimeout(() => {
      api
        .lookupSubject(query)
        .then((result) => {
          if (!cancelled) setLookup(result);
        })
        .catch(() => {
          if (!cancelled) setLookup([]);
        });
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

  const exact = lookup.filter((hit) => hit.exact);
  const candidates = lookup.filter((hit) => !hit.exact);
  const shown = new Set(lookup.map((hit) => hit.path));
  const textHits = hits.filter((hit) => !shown.has(hit.path));

  return (
    <div className="panel">
      <input
        className="panel-input"
        type="search"
        placeholder={t.searchPlaceholder}
        aria-label={t.tabSearch}
        value={query}
        autoFocus
        onChange={(e) => setQuery(e.target.value)}
      />
      {error && <p className="panel-note warn">{error}</p>}
      {query.trim() !== "" && hits.length === 0 && lookup.length === 0 && !error && (
        <p className="panel-note">{t.searchNothing}</p>
      )}
      {exact.length > 0 && (
        <section className="lookup-group">
          <h4>{lookupStrings.exact}</h4>
          <ul className="result-list">
            {exact.map((hit) => (
              <li key={`exact:${hit.path}`}>
                <button type="button" className="result" onClick={() => onOpen(hit.path)} title={hit.path}>
                  <span className="result-title">{hit.title}</span>
                  <span className="result-snippet">
                    <span className="lookup-badge">{hit.kind === "identifier" ? `${lookupStrings.codeBadge} ${hit.key ?? ""}` : lookupStrings.nameBadge}</span>
                    {hit.noteType ?? ""}
                  </span>
                </button>
              </li>
            ))}
          </ul>
        </section>
      )}
      {candidates.length > 0 && (
        <section className="lookup-group">
          <h4>{lookupStrings.candidates}</h4>
          <p className="panel-note">{lookupStrings.candidateNote}</p>
          <ul className="result-list">
            {candidates.map((hit) => (
              <li key={`candidate:${hit.path}`}>
                <button type="button" className="result" onClick={() => onOpen(hit.path)} title={hit.path}>
                  <span className="result-title">{hit.title}</span>
                  {hit.noteType && <span className="result-snippet">{hit.noteType}</span>}
                </button>
              </li>
            ))}
          </ul>
        </section>
      )}
      {textHits.length > 0 && (exact.length > 0 || candidates.length > 0) && <h4 className="lookup-heading">{lookupStrings.inText}</h4>}
      <ul className="result-list">
        {textHits.map((hit) => (
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
