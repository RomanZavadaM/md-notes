import { emit } from "@tauri-apps/api/event";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

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

/** True when this webview is the detached graph window (opened with `?view=graph`). */
export function isGraphWindow(): boolean {
  return new URLSearchParams(window.location.search).get("view") === "graph";
}

/** Opens the graph in its own window, or brings the existing one to the front. */
export async function openGraphWindow(mode: GraphWindowMode, notePath: string, title: string): Promise<void> {
  const existing = await WebviewWindow.getByLabel(GRAPH_LABEL);
  if (existing) {
    await existing.setFocus();
    const payload: GraphActiveNote = { path: notePath, mode };
    await emit(GRAPH_ACTIVE_NOTE, payload);
    return;
  }
  const query = new URLSearchParams({ view: "graph", mode, note: notePath });
  new WebviewWindow(GRAPH_LABEL, {
    url: `index.html?${query.toString()}`,
    title,
    width: 1280,
    height: 860,
    minWidth: 640,
    minHeight: 480,
    center: true,
  });
}
