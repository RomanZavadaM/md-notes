import { open } from "@tauri-apps/plugin-dialog";
import { openUrl } from "@tauri-apps/plugin-opener";
import { useEffect, useState, type ReactNode } from "react";
import { api, MATCH_END, MATCH_START, type LookupHit, type RegistryFacts, type SearchHit } from "../api";
import { useI18n } from "../i18n";
import { LOOKUP_STRINGS, REGISTRY_STRINGS, type RegistryStrings } from "../i18n/lookup";

const DIR_KEY = "mdnotes.registry.dir";

function readDir(): string {
  try {
    return localStorage.getItem(DIR_KEY) ?? "";
  } catch {
    return "";
  }
}

function money(value: number): string {
  return Math.round(value).toLocaleString("uk-UA");
}

interface Props {
  /** Changes whenever the index changes, so results are refreshed. */
  refreshKey: number;
  onOpen: (path: string) => void;
}

export function SearchPanel({ refreshKey, onOpen }: Props) {
  const { t, language } = useI18n();
  const lookupStrings = LOOKUP_STRINGS[language];
  const reg = REGISTRY_STRINGS[language];
  const [dataDir, setDataDir] = useState<string>(readDir);
  const [facts, setFacts] = useState<RegistryFacts | null>(null);
  const [factsError, setFactsError] = useState<string | null>(null);
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

  const code = query.trim();
  const isCode = /^\d{8}$/.test(code);

  useEffect(() => {
    setFacts(null);
    setFactsError(null);
    if (!isCode || dataDir === "") return;
    let cancelled = false;
    api
      .registryFacts(code, dataDir)
      .then((value) => {
        if (!cancelled) setFacts(value);
      })
      .catch((e) => {
        if (!cancelled) setFactsError(String(e));
      });
    return () => {
      cancelled = true;
    };
  }, [code, isCode, dataDir]);

  const pickDir = () => {
    void open({ directory: true, multiple: false }).then((chosen) => {
      if (typeof chosen !== "string") return;
      setDataDir(chosen);
      try {
        localStorage.setItem(DIR_KEY, chosen);
      } catch {
        // storage unavailable: the choice just is not remembered
      }
    });
  };

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
      {isCode && (
        <section className="lookup-group registry-facts">
          <h4>{reg.title}</h4>
          {dataDir === "" && <p className="panel-note">{reg.noDir}</p>}
          <p className="registry-dir">
            <button type="button" onClick={pickDir}>
              {dataDir === "" ? reg.pick : reg.change}
            </button>
            {dataDir !== "" && <small title={dataDir}> {dataDir}</small>}
          </p>
          {factsError && <p className="panel-note warn">{factsError}</p>}
          {facts && <RegistryView facts={facts} reg={reg} />}
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

/** Facts about one legal entity from the local open-data folder (exact code only). */
function RegistryView({ facts, reg }: { facts: RegistryFacts; reg: RegistryStrings }) {
  const { edr, amcu, tenders } = facts;
  const nothing = !edr && !amcu && tenders.asBuyer === 0 && tenders.asWinner === 0;
  if (nothing) return <p className="panel-note">{reg.none}</p>;
  return (
    <div className="registry-view">
      {facts.name && <p className="registry-name">{facts.name}</p>}
      {edr && (
        <ul className="registry-list">
          <li>
            {reg.registered}: {edr.registered}; {reg.state}: {edr.state}; {reg.form}: {edr.form}
          </li>
          {edr.founderCountries.length > 0 && (
            <li>
              {reg.founders}: {edr.founderCountries.join(", ")}
            </li>
          )}
          {edr.beneficiaryCountries.length > 0 && (
            <li>
              {reg.beneficiaries}: {edr.beneficiaryCountries.join(", ")}
            </li>
          )}
        </ul>
      )}
      {amcu && (
        <>
          <h5>
            {reg.amcu}
            {amcu.appealed ? ` (${reg.appealed})` : ""}
          </h5>
          <ul className="registry-list">
            {amcu.decisions.map((decision) => (
              <li key={`${decision.date}:${decision.no}`}>
                {decision.date}, {decision.body}, {decision.no}
              </li>
            ))}
          </ul>
        </>
      )}
      {(tenders.asBuyer > 0 || tenders.asWinner > 0) && (
        <>
          <h5>{reg.tenders}</h5>
          <ul className="registry-list">
            <li>
              {reg.asBuyer}: {tenders.asBuyer} ({money(tenders.buyerAmount)} UAH)
            </li>
            <li>
              {reg.asWinner}: {tenders.asWinner} ({money(tenders.winnerAmount)} UAH)
            </li>
          </ul>
          <p className="registry-latest">{reg.latest}:</p>
          <ul className="registry-list">
            {tenders.latest.map((line) => (
              <li key={`${line.id}:${line.role}`}>
                <button type="button" className="link" onClick={() => void openUrl(line.url)} title={line.url}>
                  {line.id}
                </button>{" "}
                {line.date.slice(0, 10)}, {line.role === "buyer" ? reg.buyer : reg.winner}, {money(line.amount)} UAH, {line.title}
              </li>
            ))}
          </ul>
        </>
      )}
      <p className="panel-note">{reg.note}</p>
    </div>
  );
}
