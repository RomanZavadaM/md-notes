import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

const EDITOR_PREFIX = "editor-";

function pathHash(path: string): string {
  let hash = 2166136261;
  for (let index = 0; index < path.length; index += 1) {
    hash ^= path.charCodeAt(index);
    hash = Math.imul(hash, 16777619);
  }
  return (hash >>> 0).toString(16).padStart(8, "0");
}

export function editorWindowLabel(path: string): string {
  return `${EDITOR_PREFIX}${pathHash(path)}`;
}

export function isEditorWindow(): boolean {
  return new URLSearchParams(window.location.search).get("view") === "editor";
}

export async function openEditorWindow(path: string, vaultRoot: string, title: string): Promise<void> {
  const label = editorWindowLabel(path);
  const existing = await WebviewWindow.getByLabel(label);
  if (existing) {
    await existing.unminimize();
    await existing.show();
    await existing.setFocus();
    return;
  }

  const query = new URLSearchParams({ view: "editor", note: path, root: vaultRoot });
  const editor = new WebviewWindow(label, {
    url: `index.html?${query.toString()}`,
    title,
    width: 980,
    height: 760,
    minWidth: 520,
    minHeight: 420,
    center: true,
  });

  await new Promise<void>((resolve, reject) => {
    void editor.once("tauri://created", () => resolve());
    void editor.once("tauri://error", (event) => reject(new Error(String(event.payload))));
  });
}
