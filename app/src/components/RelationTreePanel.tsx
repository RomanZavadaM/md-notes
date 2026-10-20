import { useEffect, useMemo, useState } from "react";
import { api, type RelationPathResult, type RelationTreeResult } from "../api";
import { useI18n } from "../i18n";
import { RELATION_TREE_STRINGS } from "../i18n/relationTree";
import { chipColors, edgeStyle, evidenceStyle, gradeDigit, isDarkBackground, typeColor } from "../graph/palette";
import { NODE_H, NODE_W, edgePath, foldableKeys, layoutTree, shortTitle } from "../graph/treeLayout";
import "./RelationTreePanel.css";

interface Props {
  notePath: string;
  refreshKey: number;
  onOpen: (path: string) => void;
}

type Direction = "up" | "down";

interface NoteName {
  path: string;
  title: string;
}

const PAD = 14;

/**
 * Tree of typed relations (ownership, sources) above or below a note, with the
 * weakest-link grade of each chain, plus a path finder between two notes.
 * Everything is computed by the core from the notes' `relations` lists.
 */
export function RelationTreePanel({ notePath, refreshKey, onOpen }: Props) {
  const { language } = useI18n();
  const strings = RELATION_TREE_STRINGS[language];
  const [root, setRoot] = useState(notePath);
  const [direction, setDirection] = useState<Direction>("up");
  const [depth, setDepth] = useState(4);
  const [folded, setFolded] = useState<Set<string>>(new Set());
  const [result, setResult] = useState<RelationTreeResult | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [names, setNames] = useState<NoteName[]>([]);
  const [target, setTarget] = useState("");
  const [pathResult, setPathResult] = useState<RelationPathResult | null | undefined>(undefined);
  const dark = useMemo(() => isDarkBackground(), []);

  useEffect(() => {
    setRoot(notePath);
    setPathResult(undefined);
  }, [notePath]);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    setError(null);
    api
      .relationTree(root, direction, depth)
      .then((value) => {
        if (cancelled) return;
        setResult(value);
        setFolded(new Set());
      })
      .catch((reason) => {
        if (cancelled) return;
        setResult(null);
        setError(String(reason));
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => {
      cancelled = true;
    };
  }, [root, direction, depth, refreshKey]);

  useEffect(() => {
    let cancelled = false;
    api
      .knowledgeGraph()
      .then((graph) => {
        if (!cancelled) setNames(graph.nodes.map((node) => ({ path: node.path, title: node.title })));
      })
      .catch(() => {
        if (!cancelled) setNames([]);
      });
    return () => {
      cancelled = true;
    };
  }, [refreshKey]);

  const layout = useMemo(
    () => (result ? layoutTree(result.root, folded, direction === "up") : null),
    [result, folded, direction],
  );
  const byKey = useMemo(() => new Map((layout?.placed ?? []).map((item) => [item.key, item])), [layout]);
  const hasRelations = !!result && result.root.children.length > 0;

  const toggleFold = (key: string) => {
    setFolded((current) => {
      const next = new Set(current);
      if (next.has(key)) next.delete(key);
      else next.add(key);
      return next;
    });
  };

  const findPath = () => {
    const wanted = target.trim().toLowerCase();
    const match = names.find((name) => name.title.toLowerCase() === wanted || name.path.toLowerCase() === wanted);
    if (!match) {
      setPathResult(null);
      return;
    }
    api
      .relationPath(root, match.path)
      .then((value) => setPathResult(value))
      .catch(() => setPathResult(null));
  };

  return (
    <section className="relation-tree" aria-label={strings.tab}>
      <div className="relation-tree-tools">
        <div className="relation-tree-switch" role="group" aria-label={strings.tab}>
          {(
            [
              ["up", strings.up],
              ["down", strings.down],
            ] as const
          ).map(([value, label]) => (
            <button
              key={value}
              type="button"
              className={direction === value ? "on" : ""}
              aria-pressed={direction === value}
              onClick={() => setDirection(value)}
            >
              {label}
            </button>
          ))}
        </div>
        <label className="relation-tree-depth">
          {strings.depth}
          <select value={depth} onChange={(event) => setDepth(Number(event.target.value))}>
            {[1, 2, 3, 4, 5, 6, 8].map((value) => (
              <option key={value} value={value}>
                {value}
              </option>
            ))}
          </select>
        </label>
        {hasRelations && (
          <>
            <button type="button" onClick={() => setFolded(new Set())}>
              {strings.expandAll}
            </button>
            <button type="button" onClick={() => result && setFolded(new Set(foldableKeys(result.root)))}>
              {strings.collapseAll}
            </button>
          </>
        )}
      </div>

      {result && (
        <p className="relation-tree-meta">
          {strings.root}: <strong>{result.root.title}</strong> · {strings.nodes(result.nodes)}
          {root !== notePath && (
            <button type="button" className="link" onClick={() => setRoot(notePath)}>
              ⟲
            </button>
          )}
        </p>
      )}
      {loading && <p className="panel-note">{strings.loading}</p>}
      {error && <p className="panel-note warn">{error}</p>}
      {!loading && !error && result && !hasRelations && <p className="panel-note">{strings.empty}</p>}
      {result?.truncated && <p className="panel-note warn">{strings.truncated}</p>}

      {layout && hasRelations && (
        <div className="relation-tree-stage">
          <svg
            className="relation-tree-svg"
            width={layout.width + PAD * 2}
            height={layout.height + PAD * 2}
            role="img"
            aria-label={strings.tab}
          >
            <g transform={`translate(${PAD},${PAD})`}>
              {layout.placed.map((item) => {
                const parent = item.parentKey ? byKey.get(item.parentKey) : undefined;
                if (!parent) return null;
                const style = edgeStyle(item.node.grade ?? undefined, dark);
                return (
                  <path
                    key={`edge:${item.key}`}
                    d={edgePath(parent, item)}
                    fill="none"
                    stroke={style.color}
                    strokeWidth={style.width}
                    strokeDasharray={style.dash.join(" ")}
                  />
                );
              })}
              {layout.placed.map((item) => {
                const node = item.node;
                const isRoot = item.parentKey === null;
                const digit = gradeDigit(node.grade ?? undefined);
                const evidence = evidenceStyle(digit);
                const kinds = node.kinds.join("; ");
                const sub = [evidence.rumour ? strings.rumour : "", kinds].filter(Boolean).join(" · ");
                const chain = item.depth >= 2 && node.pathGrade && node.pathGrade !== node.grade ? node.pathGrade : null;
                const grade = chipColors(node.grade ?? undefined);
                const chainColors = chipColors(chain ?? undefined);
                const foldX = direction === "up" ? 0 : NODE_W;
                return (
                  <g key={item.key} transform={`translate(${item.x},${item.y})`}>
                    <g
                      className={`relation-tree-node${isRoot ? " root" : ""}${node.repeated ? " repeated" : ""}`}
                      role="button"
                      tabIndex={0}
                      onClick={() => onOpen(node.path)}
                      onKeyDown={(event) => {
                        if (event.key === "Enter" || event.key === " ") onOpen(node.path);
                      }}
                    >
                      <title>{[node.title, kinds, node.evidence, node.status].filter(Boolean).join("\n")}</title>
                      <rect
                        width={NODE_W}
                        height={NODE_H}
                        rx={7}
                        className="box"
                        strokeDasharray={evidence.dash.join(" ")}
                      />
                      <rect width={6} height={NODE_H} rx={3} fill={typeColor(node.noteType)} />
                      <text x={14} y={20} className="name">
                        {shortTitle(node.title, chain ? 20 : 24)}
                      </text>
                      <text x={14} y={37} className="sub">
                        {shortTitle(sub || (node.repeated ? strings.repeated : ""), 30)}
                      </text>
                      {node.grade && !isRoot && (
                        <g transform={`translate(${NODE_W - 38},5)`}>
                          <rect width={32} height={16} rx={8} fill={grade.background} />
                          <text x={16} y={12} textAnchor="middle" fill={grade.color} className="chip">
                            {node.grade}
                          </text>
                        </g>
                      )}
                      {chain && (
                        <g transform={`translate(${NODE_W - 38},26)`}>
                          <title>{`${strings.chain}: ${chain}`}</title>
                          <rect width={32} height={16} rx={8} fill="none" stroke={chainColors.background} strokeWidth={1.5} />
                          <text x={16} y={12} textAnchor="middle" className="chip chain">
                            {chain}
                          </text>
                        </g>
                      )}
                      {node.hasMore && !item.foldable && <text x={NODE_W - 20} y={NODE_H - 6} className="more">…</text>}
                    </g>
                    {!isRoot && (
                      <g
                        className="relation-tree-button"
                        role="button"
                        tabIndex={0}
                        transform={`translate(${direction === "up" ? NODE_W - 9 : 9},-7)`}
                        onClick={() => setRoot(node.path)}
                        onKeyDown={(event) => {
                          if (event.key === "Enter" || event.key === " ") setRoot(node.path);
                        }}
                      >
                        <title>{strings.reroot}</title>
                        <circle r={8} />
                        <text y={4} textAnchor="middle">
                          ◎
                        </text>
                      </g>
                    )}
                    {item.foldable && (
                      <g
                        className="relation-tree-button"
                        role="button"
                        tabIndex={0}
                        transform={`translate(${foldX},${NODE_H / 2})`}
                        onClick={() => toggleFold(item.key)}
                        onKeyDown={(event) => {
                          if (event.key === "Enter" || event.key === " ") toggleFold(item.key);
                        }}
                      >
                        <circle r={9} />
                        <text y={4} textAnchor="middle">
                          {item.folded ? "+" : "−"}
                        </text>
                      </g>
                    )}
                  </g>
                );
              })}
            </g>
          </svg>
        </div>
      )}
      {hasRelations && <p className="knowledge-graph-hint">{strings.hint}</p>}

      <section className="relation-tree-path">
        <h3>{strings.pathTitle}</h3>
        <div className="relation-tree-pathbar">
          <input
            list="relation-tree-names"
            value={target}
            placeholder={strings.pathTo}
            onChange={(event) => {
              setTarget(event.target.value);
              setPathResult(undefined);
            }}
            onKeyDown={(event) => {
              if (event.key === "Enter") findPath();
            }}
          />
          <datalist id="relation-tree-names">
            {names.map((name) => (
              <option key={name.path} value={name.title} />
            ))}
          </datalist>
          <button type="button" onClick={findPath} disabled={target.trim() === ""}>
            {strings.pathFind}
          </button>
        </div>
        {pathResult === null && <p className="panel-note">{strings.pathNone}</p>}
        {pathResult && pathResult.steps.length === 1 && <p className="panel-note">{strings.pathSame}</p>}
        {pathResult && pathResult.steps.length > 1 && (
          <>
            <ol className="relation-tree-steps">
              {pathResult.steps.map((step, index) => {
                const colors = chipColors(step.grade ?? undefined);
                return (
                  <li key={`${step.path}:${index}`}>
                    {index > 0 && (
                      <span className="via">
                        {step.forward ? "↓" : "↑"} {step.kind ?? ""}{" "}
                        {step.grade && (
                          <span className="relation-chip" style={{ background: colors.background, color: colors.color }}>
                            {step.grade}
                          </span>
                        )}
                      </span>
                    )}
                    <button type="button" className="link" onClick={() => onOpen(step.path)}>
                      {step.title}
                    </button>
                  </li>
                );
              })}
            </ol>
            {pathResult.weakest && (
              <p className="relation-tree-meta">
                {strings.pathWeakest}:{" "}
                <span
                  className="relation-chip"
                  style={{
                    background: chipColors(pathResult.weakest).background,
                    color: chipColors(pathResult.weakest).color,
                  }}
                >
                  {pathResult.weakest}
                </span>
              </p>
            )}
          </>
        )}
      </section>
    </section>
  );
}
