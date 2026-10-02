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

export interface NoteSummary {
  path: string;
  title: string;
  noteType: string | null;
  tags: string[];
  aliases: string[];
}

export interface Backlink {
  path: string;
  title: string;
  context: string;
}

export interface UnresolvedLink {
  source: string;
  target: string;
}

export interface SearchHit {
  path: string;
  title: string;
  /** Matches are wrapped in MATCH_START / MATCH_END. */
  snippet: string;
}

export interface RenameOutcome {
  /** New path of the renamed entry. */
  path: string;
  /** Notes whose links were updated. */
  updated: string[];
}

export interface TemplateInfo {
  name: string;
  label: string;
}

export interface TagCount {
  tag: string;
  count: number;
}

export interface AttachmentInfo {
  name: string;
  path: string;
  size: number;
}

export type PropertyKind = "string" | "text" | "number" | "boolean" | "date" | "select" | "tags";

export interface PropertySpec {
  label: string | null;
  type: PropertyKind;
  required: boolean;
  options: string[];
}

export interface NoteTypeSpec {
  label: string | null;
  template: string | null;
  properties: Record<string, PropertySpec>;
}

export interface SchemaDocument {
  version: number;
  types: Record<string, NoteTypeSpec>;
}

export const MATCH_START = "\u0002";
export const MATCH_END = "\u0003";

/** Event emitted by the backend with vault-relative paths changed on disk. */
export const VAULT_CHANGED = "vault-changed";

export const api = {
  openVault: (path: string) => invoke<VaultInfo>("open_vault", { path }),
  getTree: () => invoke<TreeEntry[]>("get_tree"),
  readNote: (path: string) => invoke<Note>("read_note", { path }),
  saveNote: (path: string, content: string) => invoke<Note>("save_note", { path, content }),
  getSchema: () => invoke<SchemaDocument>("get_schema"),
  updateNoteProperties: (path: string, patch: Record<string, unknown>) =>
    invoke<Note>("update_note_properties", { path, patch }),
  createNote: (dir: string, title: string, template?: string) =>
    invoke<Note>("create_note", { dir, title, template: template ?? null }),
  listTemplates: () => invoke<TemplateInfo[]>("list_templates"),
  openDaily: () => invoke<Note>("open_daily"),
  createFolder: (parent: string, name: string) => invoke<string>("create_folder", { parent, name }),
  renameEntry: (from: string, to: string) => invoke<RenameOutcome>("rename_entry", { from, to }),
  trashEntry: (path: string) => invoke<string>("trash_entry", { path }),
  resolveLink: (target: string) => invoke<string | null>("resolve_link", { target }),
  listNotes: () => invoke<NoteSummary[]>("list_notes"),
  backlinks: (path: string) => invoke<Backlink[]>("backlinks", { path }),
  unresolvedLinks: () => invoke<UnresolvedLink[]>("unresolved_links"),
  listTags: () => invoke<TagCount[]>("list_tags"),
  search: (query: string, limit = 50) => invoke<SearchHit[]>("search", { query, limit }),
  importAttachment: (source: string) => invoke<AttachmentInfo>("import_attachment", { source }),
  listAttachments: () => invoke<AttachmentInfo[]>("list_attachments"),
  attachmentUsedBy: (path: string) => invoke<string[]>("attachment_used_by", { path }),
  orphanAttachments: () => invoke<AttachmentInfo[]>("orphan_attachments"),
};

/** `a/b/c.md` -> `a/b`, `c.md` -> `` */
export function parentPath(path: string): string {
  const i = path.lastIndexOf("/");
  return i < 0 ? "" : path.slice(0, i);
}

export function joinPath(dir: string, name: string): string {
  return dir ? `${dir}/${name}` : name;
}
