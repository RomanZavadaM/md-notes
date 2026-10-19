import type Graph from "graphology";
import type Sigma from "sigma";
import { edgeStyle, evidenceStyle } from "./palette";

/** Parts of the panel state the overlay needs to draw. */
export interface OverlayView {
  hovered: string | null;
  neighbours: Set<string>;
  search: string;
  hidden: Set<string>;
}

function fitCanvas(canvas: HTMLCanvasElement, width: number, height: number): CanvasRenderingContext2D | null {
  const ratio = window.devicePixelRatio || 1;
  const w = Math.max(1, Math.round(width * ratio));
  const h = Math.max(1, Math.round(height * ratio));
  if (canvas.width !== w || canvas.height !== h) {
    canvas.width = w;
    canvas.height = h;
  }
  const context = canvas.getContext("2d");
  if (!context) return null;
  context.setTransform(ratio, 0, 0, ratio, 0, 0);
  context.clearRect(0, 0, width, height);
  return context;
}

function layer(): HTMLCanvasElement {
  const canvas = document.createElement("canvas");
  canvas.style.position = "absolute";
  canvas.style.inset = "0";
  canvas.style.width = "100%";
  canvas.style.height = "100%";
  canvas.style.pointerEvents = "none";
  return canvas;
}

/**
 * Draws what WebGL lines and fills cannot: dashed and dotted lines by grade
 * under the nodes, and outlines by evidence (plus a rumour badge) above them.
 * Sigma's own edges are hidden by the panel's edge reducer.
 */
export function attachOverlay(
  container: HTMLElement,
  renderer: Sigma,
  graph: Graph,
  view: OverlayView,
  dark: boolean,
): () => void {
  const below = layer();
  const above = layer();
  container.insertBefore(below, container.firstChild);
  container.appendChild(above);
  const outline = dark ? "#E5E7EB" : "#1F2937";

  const draw = () => {
    const width = container.clientWidth;
    const height = container.clientHeight;
    const lines = fitCanvas(below, width, height);
    const rings = fitCanvas(above, width, height);
    if (!lines || !rings) return;
    const searching = view.search.length > 0;
    const focus = view.hovered;

    graph.forEachEdge((_edge, attributes, source, target) => {
      const sourceData = graph.getNodeAttributes(source);
      const targetData = graph.getNodeAttributes(target);
      if (view.hidden.has(String(sourceData.typeKey)) || view.hidden.has(String(targetData.typeKey))) return;
      const style = edgeStyle(attributes.grade as string | undefined, dark);
      const from = renderer.graphToViewport({ x: sourceData.x, y: sourceData.y });
      const to = renderer.graphToViewport({ x: targetData.x, y: targetData.y });
      const incident = focus !== null && (source === focus || target === focus);
      lines.save();
      lines.globalAlpha = focus === null ? (searching ? 0.4 : 0.9) : incident ? 1 : 0.07;
      lines.strokeStyle = style.color;
      lines.lineWidth = style.width + (incident ? 1 : 0);
      lines.setLineDash(style.dash);
      lines.beginPath();
      lines.moveTo(from.x, from.y);
      lines.lineTo(to.x, to.y);
      lines.stroke();
      lines.restore();
    });

    graph.forEachNode((node, attributes) => {
      if (view.hidden.has(String(attributes.typeKey))) return;
      const point = renderer.graphToViewport({ x: attributes.x, y: attributes.y });
      const radius = renderer.scaleSize(Number(attributes.size)) + 1;
      const matches = searching && String(attributes.fullTitle).toLowerCase().includes(view.search);
      const related = focus !== null && (node === focus || view.neighbours.has(node));
      const dimmed = (focus !== null && !related) || (searching && !matches);
      const evidence = evidenceStyle(attributes.evidence as number | undefined);
      rings.save();
      rings.globalAlpha = dimmed ? 0.25 : 1;
      rings.strokeStyle = outline;
      rings.lineWidth = 1.5;
      rings.setLineDash(evidence.dash);
      rings.beginPath();
      rings.arc(point.x, point.y, radius, 0, Math.PI * 2);
      rings.stroke();
      if (evidence.rumour) {
        // Badge «Ч»: rumour, never shown by colour alone.
        const badge = Math.max(6, Math.min(9, radius * 0.6));
        const bx = point.x + radius * 0.8;
        const by = point.y - radius * 0.8;
        rings.setLineDash([]);
        rings.fillStyle = dark ? "#FF7A70" : "#B3261E";
        rings.beginPath();
        rings.arc(bx, by, badge, 0, Math.PI * 2);
        rings.fill();
        rings.fillStyle = "#FFFFFF";
        rings.font = `bold ${Math.round(badge * 1.4)}px sans-serif`;
        rings.textAlign = "center";
        rings.textBaseline = "middle";
        rings.fillText("Ч", bx, by + 0.5);
      }
      rings.restore();
    });
  };

  renderer.on("afterRender", draw);
  draw();
  return () => {
    renderer.off("afterRender", draw);
    below.remove();
    above.remove();
  };
}
