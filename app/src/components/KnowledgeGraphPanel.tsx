import { useEffect, useRef, useState } from "react";
import Graph from "graphology";
import Sigma from "sigma";
import { api, type KnowledgeGraph } from "../api";
import { useI18n } from "../i18n";
import { GRAPH_STRINGS } from "../i18n/graph";
import "./KnowledgeGraphPanel.css";

interface Props {
  mode: "local" | "global";
  notePath: string;
  refreshKey: number;
  onOpen: (path: string) => void;
}

export function KnowledgeGraphPanel({ mode, notePath, refreshKey, onOpen }: Props) {
  const { language } = useI18n();
  const strings = GRAPH_STRINGS[language];
  const containerRef = useRef<HTMLDivElement | null>(null);
  const [snapshot, setSnapshot] = useState<KnowledgeGraph | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

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

  useEffect(() => {
    const container = containerRef.current;
    if (!container || !snapshot || snapshot.nodes.length === 0) return;

    const graph = new Graph({ type: "directed", multi: false, allowSelfLoops: false });
    const count = snapshot.nodes.length;
    const others = snapshot.nodes.filter((node) => !(mode === "local" && node.path === notePath));

    snapshot.nodes.forEach((node, index) => {
      const isFocus = mode === "local" && node.path === notePath;
      let x = 0;
      let y = 0;
      if (!isFocus) {
        const position = mode === "local" ? others.findIndex((item) => item.path === node.path) : index;
        const denominator = Math.max(mode === "local" ? others.length : count, 1);
        const angle = (Math.PI * 2 * position) / denominator - Math.PI / 2;
        const radius = mode === "local" ? 1 : Math.max(1, Math.sqrt(count) / 2);
        x = Math.cos(angle) * radius;
        y = Math.sin(angle) * radius;
      }
      graph.addNode(node.path, {
        label: node.title,
        x,
        y,
        size: isFocus ? 13 : 8,
      });
    });

    snapshot.edges.forEach((edge, index) => {
      if (graph.hasNode(edge.source) && graph.hasNode(edge.target)) {
        graph.addDirectedEdgeWithKey(`${edge.source}->${edge.target}#${index}`, edge.source, edge.target, { size: 1.5 });
      }
    });

    const renderer = new Sigma(graph, container, {
      renderEdgeLabels: false,
      labelDensity: 1,
      labelGridCellSize: 90,
      labelRenderedSizeThreshold: 6,
    });
    renderer.on("clickNode", ({ node }) => onOpen(node));

    return () => renderer.kill();
  }, [snapshot, mode, notePath, onOpen]);

  return (
    <section className="knowledge-graph" aria-label={mode === "local" ? strings.localTitle : strings.globalTitle}>
      <div className="knowledge-graph-meta">
        <strong>{mode === "local" ? strings.localTitle : strings.globalTitle}</strong>
        {snapshot && (
          <span>
            {strings.nodes(snapshot.nodes.length)} · {strings.edges(snapshot.edges.length)}
          </span>
        )}
      </div>
      {loading && <p className="panel-note">{strings.loading}</p>}
      {error && <p className="panel-note warn">{error}</p>}
      {!loading && !error && snapshot?.nodes.length === 0 && <p className="panel-note">{strings.empty}</p>}
      <div
        ref={containerRef}
        className={`knowledge-graph-canvas ${snapshot?.nodes.length ? "has-graph" : ""}`}
      />
      {snapshot && snapshot.nodes.length > 0 && <p className="knowledge-graph-hint">{strings.openHint}</p>}
    </section>
  );
}
