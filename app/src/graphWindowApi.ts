import { LogicalPosition, LogicalSize } from "@tauri-apps/api/dpi";
import { emit, emitTo } from "@tauri-apps/api/event";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { currentMonitor, Window } from "@tauri-apps/api/window";

/** Label of the main editor window from tauri.conf.json. */
export const MAIN_LABEL = "main";
/** Label of the detached graph window (see capabilities/default.json). */
export const GRAPH_LABEL = "graph";
/** Main window -> graph window: the note the user is working with changed. */
export const GRAPH_ACTIVE_NOTE = "graph-active-note";
/** Graph window -> main window: open this note. */
export const GRAPH_OPEN_NOTE = "graph-open-note";

export type GraphWindowMode = "local" | "global" | "tree";

export interface GraphActiveNote {
  path: string;
  mode?: GraphWindowMode;
}

type SideBySideLayout = {
  main: { x: number; y: number; width: number; height: number };
  graph: { x: number; y: number; width: number; height: number };
};

const TILE_GAP = 10;
const TILE_MARGIN = 8;
const WINDOW_CHROME_RESERVE = 40;
const MIN_MAIN_WIDTH = 680;
const MIN_GRAPH_WIDTH = 520;
const MIN_TILE_HEIGHT = 560;
const MAIN_SHARE = 0.58;

/** True when this webview is the detached graph window (opened with `?view=graph`). */
export function isGraphWindow(): boolean {
  return new URLSearchParams(window.location.search).get("view") === "graph";
}

async function sideBySideLayout(): Promise<SideBySideLayout | null> {
  const monitor = await currentMonitor();
  if (!monitor) return null;

  const position = monitor.workArea.position.toLogical(monitor.scaleFactor);
  const size = monitor.workArea.size.toLogical(monitor.scaleFactor);
  const availableWidth = Math.floor(size.width - TILE_MARGIN * 2 - TILE_GAP);
  const height = Math.floor(size.height - TILE_MARGIN * 2 - WINDOW_CHROME_RESERVE);

  if (availableWidth < MIN_MAIN_WIDTH + MIN_GRAPH_WIDTH || height < MIN_TILE_HEIGHT) return null;

  const mainWidth = Math.max(MIN_MAIN_WIDTH, Math.floor(availableWidth * MAIN_SHARE));
  const graphWidth = availableWidth - mainWidth;
  if (graphWidth < MIN_GRAPH_WIDTH) return null;

  const x = Math.floor(position.x + TILE_MARGIN);
  const y = Math.floor(position.y + TILE_MARGIN);
  return {
    main: { x, y, width: mainWidth, height },
    graph: { x: x + mainWidth + TILE_GAP, y, width: graphWidth, height },
  };
}

async function placeWindow(window: Window, frame: SideBySideLayout["main"]): Promise<void> {
  if (await window.isMinimized()) await window.unminimize();
  if (await window.isMaximized()) await window.unmaximize();
  await window.setPosition(new LogicalPosition(frame.x, frame.y));
  await window.setSize(new LogicalSize(frame.width, frame.height));
  await window.show();
}

async function arrangeMainAndExistingGraph(layout: SideBySideLayout): Promise<void> {
  const [main, graph] = await Promise.all([
    Window.getByLabel(MAIN_LABEL),
    Window.getByLabel(GRAPH_LABEL),
  ]);
  if (!main || !graph) return;
  await Promise.all([
    placeWindow(main, layout.main),
    placeWindow(graph, layout.graph),
  ]);
}

/**
 * Opens the graph in its own native window.
 *
 * Detaching the graph is a workspace action, not a full-screen action: when
 * there is enough room, main and graph are tiled side-by-side in the current
 * monitor work area so graph navigation remains visible together with the
 * destination note. The user can freely move/resize/maximize either window
 * after that initial layout.
 */
export async function openGraphWindow(mode: GraphWindowMode, notePath: string, title: string): Promise<void> {
  const layout = await sideBySideLayout();
  const existing = await WebviewWindow.getByLabel(GRAPH_LABEL);
  if (existing) {
    if (layout) await arrangeMainAndExistingGraph(layout);
    await existing.setFocus();
    const payload: GraphActiveNote = { path: notePath, mode };
    await emit(GRAPH_ACTIVE_NOTE, payload);
    return;
  }

  const main = await Window.getByLabel(MAIN_LABEL);
  if (layout && main) await placeWindow(main, layout.main);

  const query = new URLSearchParams({ view: "graph", mode, note: notePath });
  const options = layout
    ? {
        url: `index.html?${query.toString()}`,
        title,
        x: layout.graph.x,
        y: layout.graph.y,
        width: layout.graph.width,
        height: layout.graph.height,
        minWidth: MIN_GRAPH_WIDTH,
        minHeight: 480,
      }
    : {
        url: `index.html?${query.toString()}`,
        title,
        width: 1000,
        height: 760,
        minWidth: MIN_GRAPH_WIDTH,
        minHeight: 480,
        center: true,
      };

  new WebviewWindow(GRAPH_LABEL, options);
}

/**
 * Route graph navigation to the main editor only.
 *
 * Detached editor windows intentionally keep their own note identity. With the
 * side-by-side layout main is already visible, so navigation no longer raises
 * main above graph or bounces focus between windows.
 */
export async function openGraphNoteInMain(path: string): Promise<void> {
  await emitTo(MAIN_LABEL, GRAPH_OPEN_NOTE, path);

  const main = await Window.getByLabel(MAIN_LABEL);
  if (!main) return;
  if (await main.isMinimized()) await main.unminimize();
  await main.show();
}
