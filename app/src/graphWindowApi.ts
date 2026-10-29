import { invoke } from "@tauri-apps/api/core";
import { emit, emitTo } from "@tauri-apps/api/event";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { Window } from "@tauri-apps/api/window";

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

export interface GraphWorkspaceLayout {
  monitorWidth: number;
  monitorHeight: number;
  mainWidth: number;
  graphWidth: number;
}

/** True when this webview is the detached graph window (opened with `?view=graph`). */
export function isGraphWindow(): boolean {
  return new URLSearchParams(window.location.search).get("view") === "graph";
}

/**
 * Ask the Rust/Tauri shell to tile main + graph using the monitor's physical
 * work area. Keeping this in native code avoids Windows DPI/logical-pixel
 * ambiguity that made the earlier JS-only r3/r4 layout unreliable.
 */
export async function tileGraphWorkspace(): Promise<GraphWorkspaceLayout> {
  return invoke<GraphWorkspaceLayout>("tile_graph_workspace");
}

async function tileGraphWorkspaceWithRetry(): Promise<void> {
  const delays = [0, 80, 240, 500];
  let lastError: unknown = null;
  for (const delay of delays) {
    if (delay > 0) await new Promise((resolve) => window.setTimeout(resolve, delay));
    try {
      await tileGraphWorkspace();
      return;
    } catch (error) {
      lastError = error;
    }
  }
  console.error("Failed to tile graph workspace after native window creation", lastError);
}

/**
 * Opens the graph in its own native window.
 *
 * r5 keeps the initial graph window compact even before native tiling finishes.
 * Once the native window exists, Rust explicitly places main on the left and
 * graph on the right in physical monitor coordinates.
 */
export async function openGraphWindow(mode: GraphWindowMode, notePath: string, title: string): Promise<void> {
  const existing = await WebviewWindow.getByLabel(GRAPH_LABEL);
  if (existing) {
    await tileGraphWorkspaceWithRetry();
    await existing.setFocus();
    const payload: GraphActiveNote = { path: notePath, mode };
    await emit(GRAPH_ACTIVE_NOTE, payload);
    return;
  }

  const query = new URLSearchParams({ view: "graph", mode, note: notePath });
  const graph = new WebviewWindow(GRAPH_LABEL, {
    url: `index.html?${query.toString()}`,
    title,
    width: 900,
    height: 760,
    minWidth: 420,
    minHeight: 420,
    center: true,
  });

  await graph.once("tauri://created", () => {
    void tileGraphWorkspaceWithRetry();
  });

  await graph.once("tauri://error", (event) => {
    console.error("Failed to create detached graph window", event.payload);
  });
}

/**
 * Route graph navigation to the main editor only.
 *
 * Owner runtime confirmed this routing already changes the active note in main;
 * r5 deliberately does not alter tab/document semantics. Its scope is window
 * visibility/layout only.
 */
export async function openGraphNoteInMain(path: string): Promise<void> {
  await emitTo(MAIN_LABEL, GRAPH_OPEN_NOTE, path);

  const main = await Window.getByLabel(MAIN_LABEL);
  if (!main) return;
  if (await main.isMinimized()) await main.unminimize();
  await main.show();
}
