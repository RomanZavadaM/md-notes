import { useEffect, useMemo, useRef, useState } from "react";
import Graph from "graphology";
import Sigma from "sigma";
import { api, type KnowledgeGraph } from "../api";
import { ForceLayout } from "../graph/layout";
import { attachOverlay, type EdgeHit } from "../graph/overlay";
import { chipColors, gradeDigit, isDarkBackground, nodeSize, typeColor } from "../graph/palette";
import { useI18n } from "../i18n";
import { GRAPH_STRINGS } from "../i18n/graph";
import { RelationCard, type RelationCardData, type RelationItem } from "./RelationCard";
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
  trace: { nodes: Set<string>; edges: Set<string> } | null;
}

interface TraceNode {
  path: string;
  title: string;
  type: string;
  grade?: string;
  kinds: string;
  evidence?: string;
  repeated: boolean;
  children: TraceNode[];
}

/** Upstream chain of `root`: who owns it, who owns them, plus the materials around the chain. */
function buildTrace(graph: Graph, root: string) {
  const nodes = new Set<string>([root]);
  const edges = new Set<string>();
  const materials = new Set<string>();
  const expanded = new Set<string>();
  const visit = (node: string, depth: number): TraceNode => {
    const owners: TraceNode[] = [];
    const repeated = expanded.has(node);
    expanded.add(node);
    if (!repeated && depth < 8) {
      graph.forEachInEdge(node, (edge, attributes, source) => {
        if (attributes.hasItems !== true) return;
        edges.add(edge);
        nodes.add(source);
        const items = attributes.items as RelationItem[];
        const child = visit(source, depth + 1);
        child.grade = attributes.grade as string | undefined;
        child.kinds = items.map((item) => item.kind).filter(Boolean).join('; ');
        child.evidence = items.find((item) => item.evidence)?.evidence;
        owners.push(child);
      });
    }
    graph.forEachEdge(node, (edge, attributes, source, target) => {
      const other = source === node ? target : source;
      if (graph.getNodeAttribute(other, 'typeKey') === 'material' && attributes.hasItems !== true) {
        materials.add(other);
        edges.add(edge);
      }
    });
    return {
      path: node,
      title: String(graph.getNodeAttribute(node, 'fullTitle')),
      type: String(graph.getNodeAttribute(node, 'typeKey')),
      kinds: '',
      repeated,
      children: owners,
    };
  };
  const tree = visit(root, 0);
  materials.forEach((material) => nodes.add(material));
  return { tree, nodes, edges, materials: [...materials] };
}

export function KnowledgeGraphPanel({ mode, notePath, refreshKey, onOpen }: Props) {
  const { language } = useI18n();
  const strings = GRAPH_STRINGS[language];
  const containerRef = useRef<HTMLDivElement | null>(null);
  const rendererRef = useRef<Sigma | null>(null);
  const viewRef = useRef<ViewState>({ hovered: null, neighbours: new Set(), search: "", hidden: new Set(), trace: null });
  const graphRef = useRef<Graph | null>(null);
  const traceModeRef = useRef(false);
  const [snapshot, setSnapshot] = useState<KnowledgeGraph | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [search, setSearch] = useState("");
  const [hiddenTypes, setHiddenTypes] = useState<Set<string>>(new Set());
  const [hover, setHover] = useState<Hover | null>(null);
  const [edgeCard, setEdgeCard] = useState<RelationCardData | null>(null);
  const [expanded, setExpanded] = useState(false);
  const [traceMode, setTraceMode] = useState(false);
  const [traceRoot, setTraceRoot] = useState<string | null>(null);
  const [traceResult, setTraceResult] = useState<ReturnType<typeof buildTrace> | null>(null);

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

    // One line per pair of notes: typed relations (styled by grade, with an
    // arrow and a card) win over a plain [[link]] between the same notes.
    const titleOf = new Map(snapshot.nodes.map((node) => [node.path, node.title]));
    interface PairEdge {
      source: string;
      target: string;
      grade?: string;
      items: RelationItem[];
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
      const item: RelationItem = {
        kind: relation.kind,
        grade: relation.grade ?? undefined,
        origin: relation.origin ?? undefined,
        status: relation.status ?? undefined,
        evidence: relation.evidence ?? undefined,
        from: titleOf.get(relation.source) ?? relation.source,
        to: titleOf.get(relation.target) ?? relation.target,
      };
      const current = pairEdges.get(key);
      if (!current) {
        pairEdges.set(key, {
          source: relation.source,
          target: relation.target,
          grade: relation.grade ?? undefined,
          items: [item],
          digit,
        });
      } else {
        current.items.push(item);
        if (digit !== undefined && (current.digit === undefined || digit < current.digit)) {
          current.digit = digit;
          current.grade = relation.grade ?? undefined;
          current.source = relation.source;
          current.target = relation.target;
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
      if (!pairEdges.has(key)) pairEdges.set(key, { source: edge.source, target: edge.target, items: [] });
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
        arrow: edge.items.length > 0,
        hasItems: edge.items.length > 0,
        items: edge.items,
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
        if (view.trace !== null) {
          if (view.trace.nodes.has(node)) {
            result.zIndex = 2;
            result.forceLabel = true;
          } else {
            result.color = dimNode;
            result.label = "";
            result.zIndex = 0;
          }
        } else if (focus !== null && !related) {
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
    graphRef.current = graph;
    const detachOverlay = attachOverlay(container, renderer, graph, view, dark, (hit: EdgeHit | null) => {
      if (!hit || !graph.hasEdge(hit.edge)) {
        setEdgeCard(null);
        return;
      }
      const [source, target] = graph.extremities(hit.edge);
      const items = graph.getEdgeAttribute(hit.edge, "items") as RelationItem[];
      setEdgeCard({
        x: hit.x,
        y: hit.y,
        from: String(graph.getNodeAttribute(source, "fullTitle")),
        to: String(graph.getNodeAttribute(target, "fullTitle")),
        items,
      });
    });

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
      if (!moved) {
        if (traceModeRef.current) setTraceRoot(node);
        else onOpen(node);
      }
      moved = false;
    });

    return () => {
      cancelAnimationFrame(frame);
      detachOverlay();
      renderer.kill();
      rendererRef.current = null;
      graphRef.current = null;
    };
  }, [snapshot, mode, notePath, onOpen]);

  // Search and type filters only change what the reducers return.
  useEffect(() => {
    viewRef.current.search = search.trim().toLowerCase();
    viewRef.current.hidden = hiddenTypes;
    rendererRef.current?.refresh();
  }, [search, hiddenTypes, snapshot]);

  // Provenance trace: recompute when the root changes, keep the graph repainted.
  useEffect(() => {
    traceModeRef.current = traceMode;
    const graph = graphRef.current;
    if (!traceMode || traceRoot === null || !graph || !graph.hasNode(traceRoot)) {
      viewRef.current.trace = null;
      setTraceResult(null);
    } else {
      const result = buildTrace(graph, traceRoot);
      viewRef.current.trace = { nodes: result.nodes, edges: result.edges };
      setTraceResult(result);
    }
    rendererRef.current?.refresh();
  }, [traceMode, traceRoot, snapshot]);

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
          <button
            type="button"
            className={`knowledge-graph-trace-toggle${traceMode ? " on" : ""}`}
            aria-pressed={traceMode}
            onClick={() => {
              setTraceMode((value) => !value);
              setTraceRoot(null);
            }}
          >
            {strings.trace}
          </button>
          {traceMode && traceRoot === null && <small className="knowledge-graph-hint">{strings.traceHint}</small>}
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
      {traceMode && traceResult && (
        <div className="knowledge-graph-trace">
          <header>
            <strong>
              {strings.traceFor} {traceResult.tree.title}
            </strong>
            <button type="button" onClick={() => setTraceRoot(null)}>
              {strings.traceClear}
            </button>
          </header>
          {traceResult.tree.children.length === 0 ? (
            <small>{strings.traceNone}</small>
          ) : (
            <TraceList nodes={traceResult.tree.children} onOpen={onOpen} />
          )}
          {traceResult.materials.length > 0 && (
            <div>
              <strong>{strings.traceMaterials}</strong>
              <ul>
                {traceResult.materials.map((path) => (
                  <li key={path}>
                    <button type="button" className="link" onClick={() => onOpen(path)}>
                      {String(graphRef.current?.getNodeAttribute(path, "fullTitle") ?? path)}
                    </button>
                  </li>
                ))}
              </ul>
            </div>
          )}
        </div>
      )}
      {loading && <p className="panel-note">{strings.loading}</p>}
      {error && <p className="panel-note warn">{error}</p>}
      {!loading && !error && snapshot?.nodes.length === 0 && <p className="panel-note">{strings.empty}</p>}
      <div className="knowledge-graph-stage">
        <div ref={containerRef} className={`knowledge-graph-canvas ${hasGraph ? "has-graph" : ""}`} />
        {edgeCard && !hover && <RelationCard card={edgeCard} />}
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

function TraceList({ nodes, onOpen }: { nodes: TraceNode[]; onOpen: (path: string) => void }) {
  return (
    <ul>
      {nodes.map((node) => {
        const colors = chipColors(node.grade);
        return (
          <li key={node.path}>
            <div className="trace-line">
              <span className="relation-chip" style={{ background: colors.background, color: colors.color }}>
                {node.grade ?? "—"}
              </span>
              <button type="button" className="link" onClick={() => onOpen(node.path)}>
                {node.title}
              </button>
              {node.kinds && <small>{node.kinds}</small>}
              {node.repeated && <small>↺</small>}
            </div>
            {node.evidence && <small>{node.evidence}</small>}
            {node.children.length > 0 && <TraceList nodes={node.children} onOpen={onOpen} />}
          </li>
        );
      })}
    </ul>
  );
}
