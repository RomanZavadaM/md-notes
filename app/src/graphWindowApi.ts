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

/** True when this webview is the detached graph window. */
export function isGraphWindow(): boolean {
  return new URLSearchParams(window.location.search).get("view") === "graph";
}

/**
 * Opens the graph as an independent native window, like a detached note tab.
 * It must not resize, reposition, hide or otherwise mutate the main window.
 */
export async function openGraphWindow(mode: GraphWindowMode, notePath: string, title: string): Promise<void> {
  const existing = await WebviewWindow.getByLabel(GRAPH_LABEL);
  if (existing) {
    await existing.setFocus();
    const payload: GraphActiveNote = { path: notePath, mode };
    await emit(GRAPH_ACTIVE_NOTE, payload);
    return;
  }

  const query = new URLSearchParams({ view: "graph", mode, note: notePath });
  const graph = new WebviewWindow(GRAPH_LABEL, {
    url: `index.html?${query.toString()}`,
    title,
    width: 1100,
    height: 800,
    minWidth: 520,
    minHeight: 480,
    center: true,
  });

  await graph.once("tauri://error", (event) => {
    console.error("Failed to create detached graph window", event.payload);
  });
}

/** Route graph navigation back to the central main window. */
export async function openGraphNoteInMain(path: string): Promise<void> {
  await emitTo(MAIN_LABEL, GRAPH_OPEN_NOTE, path);

  const main = await Window.getByLabel(MAIN_LABEL);
  if (!main) return;
  if (await main.isMinimized()) await main.unminimize();
  await main.show();
}
