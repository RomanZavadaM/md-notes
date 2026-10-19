import { useEffect, useMemo, useRef, useState } from "react";
import Graph from "graphology";
import Sigma from "sigma";
import { api, type KnowledgeGraph } from "../api";
import { ForceLayout } from "../graph/layout";
import { attachOverlay } from "../graph/overlay";
import { gradeDigit, isDarkBackground, nodeSize, typeColor } from "../graph/palette";
import { useI18n } from "../i18n";
import { GRAPH_STRINGS } from "../i18n/graph";
import "./KnowledgeGraphPanel.css";

interface Props {
  mode: "local" | "global";
  notePath: string;
  refreshKey: number;
  onOpen: (path: string) => void;
}

interface Hover {
  title: string;
  type: string;
  degree: number;
  x: number;
  y: number;
}

interface ViewState {
  hovered: string | null;
  neighbours: Set<string>;
  search: string;
  hidden: Set<string>;
}

export function KnowledgeGraphPanel({ mode, notePath, refreshKey, onOpen }: Props) {
  const { language } = useI18n();
  const strings = GRAPH_STRINGS[language];
  const containerRef = useRef<HTMLDivElement | null>(null);
  const rendererRef = useRef<Sigma | null>(null);
  const viewRef = useRef<ViewState>({ hovered: null, neighbours: new Set(), search: "", hidden: new Set() });
  const [snapshot, setSnapshot] = useState<KnowledgeGraph | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [search, setSearch] = useState("");
  const [hiddenTypes, setHiddenTypes] = useState<Set<string>>(new Set());
  const [hover, setHover] = useState<Hover | null>(null);
  const [expanded, setExpanded] = useState(false);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    setError(null);
    api
      .knowledgeGraph(mode === "local" ? notePath : undefined)
      .then((result) => {
        if (!cancelled) setSnapshot(result);
      })
      .catch((reason) => {
        if (!cancelled) {
          setSnapshot(null);
          setError(String(reason));
        }
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => {
      cancelled = true;
    };
  }, [mode, notePath, refreshKey]);

  // Types present in the graph with counts, for the legend and its filter.
  const legend = useMemo(() => {
    const counts = new Map<string, number>();
    snapshot?.nodes.forEach((node) => {
      const key = node.noteType ?? "";
      counts.set(key, (counts.get(key) ?? 0) + 1);
    });
    return [...counts.entries()].sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0]));
  }, [snapshot]);

  useEffect(() => {
    const container = containerRef.current;
    if (!container || !snapshot || snapshot.nodes.length === 0) return;

    const dark = isDarkBackground();
    const labelColor = dark ? "#E5E7EB" : "#1D2430";
    const edgeColor = dark ? "#5B6475" : "#A3ABB9";
    const dimNode = dark ? "#2B313D" : "#DDE1E8";

    const graph = new Graph({ type: "directed", multi: false, allowSelfLoops: false });
    const indexOf = new Map<string, number>();
    const degree = new Map<string, number>();
    snapshot.nodes.forEach((node, index) => {
      indexOf.set(node.path, index);
      degree.set(node.path, 0);
    });

    // One line per pair of notes: a typed relation (styled by grade) wins over
    // a plain [[link]] between the same notes.
    interface PairEdge {
      source: string;
      target: string;
      grade?: string;
      kinds: string[];
      digit?: number;
    }
    const pairEdges = new Map<string, PairEdge>();
    const pairKey = (a: number, b: number) => (a < b ? `${a}:${b}` : `${b}:${a}`);
    const evidence = new Map<string, number>();
    (snapshot.relations ?? []).forEach((relation) => {
      const a = indexOf.get(relation.source);
      const b = indexOf.get(relation.target);
      if (a === undefined || b === undefined || a === b) return;
      const key = pairKey(a, b);
      const digit = gradeDigit(relation.grade);
      const current = pairEdges.get(key);
      if (!current) {
        pairEdges.set(key, {
          source: relation.source,
          target: relation.target,
          grade: relation.grade ?? undefined,
          kinds: relation.kind ? [relation.kind] : [],
          digit,
        });
      } else {
        if (relation.kind && !current.kinds.includes(relation.kind)) current.kinds.push(relation.kind);
        if (digit !== undefined && (current.digit === undefined || digit < current.digit)) {
          current.digit = digit;
          current.grade = relation.grade ?? undefined;
        }
      }
      if (digit !== undefined) {
        for (const path of [relation.source, relation.target]) {
          const best = evidence.get(path);
          if (best === undefined || digit < best) evidence.set(path, digit);
        }
      }
    });
    snapshot.edges.forEach((edge) => {
      const a = indexOf.get(edge.source);
      const b = indexOf.get(edge.target);
      if (a === undefined || b === undefined || a === b) return;
      const key = pairKey(a, b);
      if (!pairEdges.has(key)) pairEdges.set(key, { source: edge.source, target: edge.target, kinds: [] });
    });
    const pairs: Array<[number, number]> = [];
    pairEdges.forEach((edge) => {
      degree.set(edge.source, (degree.get(edge.source) ?? 0) + 1);
      degree.set(edge.target, (degree.get(edge.target) ?? 0) + 1);
      pairs.push([indexOf.get(edge.source) ?? 0, indexOf.get(edge.target) ?? 0]);
    });

    const layout = new ForceLayout(snapshot.nodes.length, pairs);
    snapshot.nodes.forEach((node, index) => {
      const isFocus = mode === "local" && node.path === notePath;
      const type = node.noteType ?? "";
      const size = nodeSize(degree.get(node.path) ?? 0);
      graph.addNode(node.path, {
        label: node.title,
        x: layout.x[index],
        y: layout.y[index],
        size: isFocus ? size + 4 : size,
        color: typeColor(type),
        typeKey: type,
        fullTitle: node.title,
        isFocus,
        evidence: evidence.get(node.path),
      });
    });
    pairEdges.forEach((edge, key) => {
      graph.addDirectedEdgeWithKey(`pair:${key}`, edge.source, edge.target, {
        size: 1,
        grade: edge.grade,
        kinds: edge.kinds.join("; "),
      });
    });

    const view = viewRef.current;
    view.hovered = null;
    view.neighbours = new Set();

    const renderer = new Sigma(graph, container, {
      renderEdgeLabels: false,
      labelDensity: 1.1,
      labelGridCellSize: 80,
      labelRenderedSizeThreshold: 5,
      labelColor: { color: labelColor },
      labelSize: 12,
      defaultEdgeColor: edgeColor,
      zIndex: true,
      nodeReducer: (node, data) => {
        const result = { ...data };
        if (view.hidden.has(String(data.typeKey))) {
          result.hidden = true;
          return result;
        }
        const searching = view.search.length > 0;
        const matches = searching && String(data.fullTitle).toLowerCase().includes(view.search);
        const focus = view.hovered;
        const related = focus !== null && (node === focus || view.neighbours.has(node));
        if (focus !== null && !related) {
          result.color = dimNode;
          result.label = "";
          result.zIndex = 0;
        } else if (searching && !matches) {
          result.color = dimNode;
          result.label = "";
          result.zIndex = 0;
        } else {
          result.zIndex = 1;
          if (matches || node === focus || data.isFocus === true) {
            result.forceLabel = true;
            result.zIndex = 2;
          }
        }
        return result;
      },
      // Lines are drawn by the overlay (dashes by grade); hide Sigma's own.
      edgeReducer: (_edge, data) => ({ ...data, hidden: true }),
    });
    rendererRef.current = renderer;
    const detachOverlay = attachOverlay(container, renderer, graph, view, dark);

    // Hover: highlight the node with its neighbours, show a small card.
    renderer.on("enterNode", ({ node }) => {
      view.hovered = node;
      view.neighbours = new Set(graph.neighbors(node));
      const attributes = graph.getNodeAttributes(node);
      const point = renderer.graphToViewport({ x: attributes.x, y: attributes.y });
      setHover({
        title: String(attributes.fullTitle),
        type: String(attributes.typeKey),
        degree: graph.degree(node),
        x: point.x,
        y: point.y,
      });
      renderer.refresh();
    });
    renderer.on("leaveNode", () => {
      view.hovered = null;
      view.neighbours = new Set();
      setHover(null);
      renderer.refresh();
    });

    // Let the layout settle visibly, a few iterations per frame.
    let frame = 0;
    const tick = () => {
      for (let i = 0; i < layout.perFrame && !layout.done; i += 1) layout.step();
      graph.updateEachNodeAttributes((node, attributes) => {
        const index = indexOf.get(node) ?? 0;
        return { ...attributes, x: layout.x[index], y: layout.y[index] };
      });
      if (!layout.done) frame = requestAnimationFrame(tick);
    };
    frame = requestAnimationFrame(tick);

    // Drag a node to move it; a click without movement opens the note.
    let dragged: string | null = null;
    let moved = false;
    renderer.on("downNode", (event) => {
      dragged = event.node;
      moved = false;
      cancelAnimationFrame(frame);
    });
    renderer.getMouseCaptor().on("mousedown", () => {
      if (!renderer.getCustomBBox()) renderer.setCustomBBox(renderer.getBBox());
    });
    renderer.getMouseCaptor().on("mousemovebody", (event) => {
      if (dragged === null) return;
      moved = true;
      const position = renderer.viewportToGraph(event);
      graph.setNodeAttribute(dragged, "x", position.x);
      graph.setNodeAttribute(dragged, "y", position.y);
      setHover(null);
      event.preventSigmaDefault();
      event.original.preventDefault();
      event.original.stopPropagation();
    });
    renderer.getMouseCaptor().on("mouseup", () => {
      dragged = null;
    });
    renderer.on("clickNode", ({ node }) => {
      if (!moved) onOpen(node);
      moved = false;
    });

    return () => {
      cancelAnimationFrame(frame);
      detachOverlay();
      renderer.kill();
      rendererRef.current = null;
    };
  }, [snapshot, mode, notePath, onOpen]);

  // Search and type filters only change what the reducers return.
  useEffect(() => {
    viewRef.current.search = search.trim().toLowerCase();
    viewRef.current.hidden = hiddenTypes;
    rendererRef.current?.refresh();
  }, [search, hiddenTypes, snapshot]);

  const toggleType = (type: string) => {
    setHiddenTypes((current) => {
      const next = new Set(current);
      if (next.has(type)) next.delete(type);
      else next.add(type);
      return next;
    });
  };

  const hasGraph = !!snapshot && snapshot.nodes.length > 0;

  return (
    <section
      className={`knowledge-graph${expanded ? " expanded" : ""}`}
      aria-label={mode === "local" ? strings.localTitle : strings.globalTitle}
    >
      <div className="knowledge-graph-meta">
        <div className="knowledge-graph-title">
          <strong>{mode === "local" ? strings.localTitle : strings.globalTitle}</strong>
          {hasGraph && (
            <button type="button" className="knowledge-graph-expand" onClick={() => setExpanded((value) => !value)}>
              {expanded ? strings.collapse : strings.expand}
            </button>
          )}
        </div>
        {snapshot && (
          <span>
            {strings.nodes(snapshot.nodes.length)} · {strings.edges(snapshot.edges.length)}
          </span>
        )}
      </div>
      {hasGraph && (
        <div className="knowledge-graph-tools">
          <input
            type="search"
            value={search}
            placeholder={strings.searchPlaceholder}
            onChange={(event) => setSearch(event.target.value)}
          />
          <ul className="knowledge-graph-legend" aria-label={strings.typesHeading}>
            {legend.map(([type, count]) => (
              <li key={type || "_none"}>
                <button
                  type="button"
                  className={hiddenTypes.has(type) ? "off" : ""}
                  aria-pressed={!hiddenTypes.has(type)}
                  onClick={() => toggleType(type)}
                >
                  <span className="dot" style={{ background: typeColor(type) }} />
                  {type || strings.noType} <small>{count}</small>
                </button>
              </li>
            ))}
          </ul>
        </div>
      )}
      {loading && <p className="panel-note">{strings.loading}</p>}
      {error && <p className="panel-note warn">{error}</p>}
      {!loading && !error && snapshot?.nodes.length === 0 && <p className="panel-note">{strings.empty}</p>}
      <div className="knowledge-graph-stage">
        <div ref={containerRef} className={`knowledge-graph-canvas ${hasGraph ? "has-graph" : ""}`} />
        {hover && (
          <div className="knowledge-graph-card" style={{ left: hover.x + 14, top: hover.y + 14 }}>
            <strong>{hover.title}</strong>
            <span>
              <i className="dot" style={{ background: typeColor(hover.type) }} />
              {hover.type || strings.noType} · {strings.connections(hover.degree)}
            </span>
          </div>
        )}
      </div>
      {hasGraph && <p className="knowledge-graph-hint">{strings.openHint}</p>}
    </section>
  );
}
