// Typed wrappers around the Tauri commands defined in src-tauri/src/lib.rs.
import { invoke } from "@tauri-apps/api/core";

export type EntryKind = "dir" | "note" | "file";

export interface TreeEntry {
  name: string;
  path: string;
  kind: EntryKind;
  children?: TreeEntry[];
}

export interface WikiLink {
  target: string;
  heading: string | null;
  alias: string | null;
  embed: boolean;
}

export interface Note {
  path: string;
  title: string;
  content: string;
  frontMatter: unknown;
  frontMatterError: string | null;
  links: WikiLink[];
  tags: string[];
}

export interface VaultInfo {
  root: string;
  name: string;
}

export const api = {
  openVault: (path: string) => invoke<VaultInfo>("open_vault", { path }),
  getTree: () => invoke<TreeEntry[]>("get_tree"),
  readNote: (path: string) => invoke<Note>("read_note", { path }),
  saveNote: (path: string, content: string) => invoke<Note>("save_note", { path, content }),
  createNote: (dir: string, title: string) => invoke<Note>("create_note", { dir, title }),
  createFolder: (parent: string, name: string) => invoke<string>("create_folder", { parent, name }),
  renameEntry: (from: string, to: string) => invoke<string>("rename_entry", { from, to }),
  trashEntry: (path: string) => invoke<string>("trash_entry", { path }),
  resolveLink: (target: string) => invoke<string | null>("resolve_link", { target }),
};

/** `a/b/c.md` -> `a/b`, `c.md` -> `` */
export function parentPath(path: string): string {
  const i = path.lastIndexOf("/");
  return i < 0 ? "" : path.slice(0, i);
}

export function joinPath(dir: string, name: string): string {
  return dir ? `${dir}/${name}` : name;
}
