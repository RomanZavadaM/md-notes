// Typed wrappers around the Tauri commands defined in src-tauri/src/lib.rs.
import { invoke } from "@tauri-apps/api/core";

export type EntryKind = "dir" | "note" | "file";
export type VaultPreset = "empty" | "para" | "zettelkasten";
export type RuntimePlatform = "android" | "ios" | "windows" | "macos" | "linux" | "unknown";

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

export interface EncryptionStatus {
  encrypted: boolean;
  hasKey: boolean;
  vaultId: string | null;
  recipients: number;
}

export interface CreatedEncryptedVault {
  info: VaultInfo;
  /** Private key; shown once so the user can back it up. */
  secretKey: string;
  publicKey: string;
}

/** Error text of `openVault` when the vault is encrypted and this device has no key. */
export const KEY_MISSING = "KEY_MISSING";

export interface DuplicateNote {
  path: string;
  title: string;
  identity: string | null;
}

export interface DuplicateGroup {
  notes: DuplicateNote[];
  /** Every note carries the same identity code: they should have been merged. */
  sameCode: boolean;
}

export interface RegistryFacts {
  code: string;
  name: string | null;
  edr: { registered: string; state: string; form: string; founderCountries: string[]; beneficiaryCountries: string[] } | null;
  amcu: { name: string; appealed: boolean; decisions: Array<{ date: string; body: string; no: string }> } | null;
  tenders: {
    asBuyer: number;
    asWinner: number;
    buyerAmount: number;
    winnerAmount: number;
    latest: Array<{ id: string; title: string; url: string; date: string; role: string; amount: number }>;
  };
}

export interface LookupHit {
  path: string;
  title: string;
  noteType: string | null;
  /** `identifier` (exact code) or `name` (title or alias). */
  kind: "identifier" | "name";
  key: string | null;
  /** Exact: same code or same normalised name. Otherwise only a candidate. */
  exact: boolean;
}

export interface RelationTreeNode {
  path: string;
  title: string;
  noteType: string | null;
  kinds: string[];
  /** Best grade among the lines between this note and its parent. */
  grade: string | null;
  /** Weakest grade along the chain from the root to this note. */
  pathGrade: string | null;
  origin: string | null;
  status: string | null;
  evidence: string | null;
  repeated: boolean;
  hasMore: boolean;
  children: RelationTreeNode[];
}

export interface RelationTreeResult {
  root: RelationTreeNode;
  nodes: number;
  truncated: boolean;
}

export interface RelationPathStep {
  path: string;
  title: string;
  noteType: string | null;
  kind: string | null;
  grade: string | null;
  forward: boolean;
  origin: string | null;
  status: string | null;
}

export interface RelationPathResult {
  steps: RelationPathStep[];
  weakest: string | null;
}

export interface ImportReport {
  created: number;
  updated: number;
  skipped: number;
  rejected: number;
  /** Notes folded into an existing note with the same identity code (ADR-0008). */
  merged: number;
  /** Differing values kept from existing notes during merges, to be reviewed. */
  conflicts: number;
}

export type GitRelation = "upToDate" | "fastForward" | "localAhead" | "diverged" | "unrelated";

export interface GitStatus {
  isRepo: boolean;
  branch: string | null;
  remoteUrl: string | null;
  hasChanges: boolean;
  relation: GitRelation | null;
}

export interface GitPullResult {
  relation: GitRelation;
  worktreeClean: boolean;
  applied: boolean;
}

export interface StandaloneMarkdown {
  path: string;
  directory: string;
  name: string;
  note: Note;
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

export interface GraphNode {
  path: string;
  title: string;
  noteType: string | null;
}

export interface GraphEdge {
  source: string;
  target: string;
}

export interface GraphRelation {
  source: string;
  target: string;
  kind: string;
  grade: string | null;
  origin: string | null;
  status: string | null;
  evidence: string | null;
}

export interface KnowledgeGraph {
  nodes: GraphNode[];
  edges: GraphEdge[];
  relations: GraphRelation[];
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

export interface BackupSummary {
  archive: string;
  files: number;
  bytes: number;
  encrypted: boolean;
}

export interface RestoreSummary {
  path: string;
  files: number;
  bytes: number;
  encrypted: boolean;
}

export type FieldKind =
  | "text"
  | "string"
  | "number"
  | "boolean"
  | "date"
  | "enum"
  | "list"
  | "link"
  | "links"
  | "url"
  | "file";

export interface FieldSpec {
  label: string | null;
  type: FieldKind;
  readonly: boolean;
  values: string[];
  noteType: string | null;
}

export interface NoteTypeSpec {
  label: string | null;
  template: string | null;
  fields: string[];
  required: string[];
}

export interface SchemaDocument {
  version: number;
  fields: Record<string, FieldSpec>;
  types: Record<string, NoteTypeSpec>;
}

export const MATCH_START = "\u0002";
export const MATCH_END = "\u0003";

/** Event emitted by the backend with vault-relative paths changed on disk. */
export const VAULT_CHANGED = "vault-changed";
export const OPENED_MARKDOWN_FILES = "opened-markdown-files";

export const api = {
  runtimePlatform: () => invoke<RuntimePlatform>("runtime_platform"),
  openMobileSandboxVault: () => invoke<VaultInfo>("open_mobile_sandbox_vault"),
  openMarkdownFile: (path: string) => invoke<StandaloneMarkdown>("open_markdown_file", { path }),
  saveMarkdownFile: (path: string, content: string) =>
    invoke<StandaloneMarkdown>("save_markdown_file", { path, content }),
  startupMarkdownFiles: () => invoke<string[]>("startup_markdown_files"),
  configureMarkdownAssociation: () => invoke<"systemChoice" | "defaultSet" | "unsupported">("configure_markdown_association"),
  openVault: (path: string) => invoke<VaultInfo>("open_vault", { path }),
  createVault: (path: string, name: string | null, preset: VaultPreset) =>
    invoke<VaultInfo>("create_vault", { path, name, preset }),
  encryptionStatus: (path: string) => invoke<EncryptionStatus>("encryption_status", { path }),
  createEncryptedVault: (path: string, name: string | null, recoveryPublicKey: string | null) =>
    invoke<CreatedEncryptedVault>("create_encrypted_vault_cmd", { path, name, recoveryPublicKey }),
  importEncryptionKey: (path: string, secretKey: string) =>
    invoke<VaultInfo>("import_encryption_key", { path, secretKey }),
  addEncryptionRecipient: (recipient: string) => invoke<number>("add_encryption_recipient", { recipient }),
  /** Pushes the current branch (system Git, desktop only); resolves to `"pushed"` or `"upToDate"`. */
  gitPush: (remoteName: string | null = null) =>
    invoke<"pushed" | "upToDate">("git_push_with_stored_credentials", { remoteName }),
  /** Closes the application (desktop). */
  quitApp: () => invoke<void>("quit_app"),
  importFolder: (source: string, overwrite = false) => invoke<ImportReport>("import_folder", { source, overwrite }),
  possibleDuplicates: () => invoke<DuplicateGroup[]>("possible_duplicates"),
  registryFacts: (code: string, dataDir: string) => invoke<RegistryFacts>("registry_facts", { code, dataDir }),
  lookupSubject: (query: string, limit = 20) => invoke<LookupHit[]>("lookup_subject", { query, limit }),
  relationTree: (root: string, direction: "up" | "down", maxDepth: number) =>
    invoke<RelationTreeResult | null>("relation_tree_for", { root, direction, maxDepth }),
  relationPath: (from: string, to: string, mostReliable = false) =>
    invoke<RelationPathResult | null>("relation_path_between", { from, to, mostReliable }),
  gitStatus: (remoteName: string | null = null) => invoke<GitStatus>("git_status_cmd", { remoteName }),
  gitInit: () => invoke<void>("git_init_repo"),
  gitSetRemote: (url: string, remoteName: string | null = null) =>
    invoke<void>("git_set_remote_url", { remoteName, url }),
  gitCommit: (message: string) => invoke<string | null>("git_commit_all", { message }),
  gitFetch: (remoteName: string | null = null) => invoke<void>("git_fetch_with_stored_credentials", { remoteName }),
  gitPull: (remoteName: string | null = null) =>
    invoke<GitPullResult>("git_pull_with_stored_credentials", { remoteName }),
  saveGitCredentials: (username: string, token: string) =>
    invoke<void>("save_git_credentials", { username, token }),
  hasGitCredentials: () => invoke<boolean>("has_git_credentials"),
  clearGitCredentials: () => invoke<void>("clear_git_credentials"),
  pushSafety: () => invoke<{ path: string; reason: string }[]>("push_safety"),
  getTree: () => invoke<TreeEntry[]>("get_tree"),
  readNote: (path: string) => invoke<Note>("read_note", { path }),
  parseNoteContent: (path: string, content: string) => invoke<Note>("parse_note_content", { path, content }),
  saveNote: (path: string, content: string) => invoke<Note>("save_note", { path, content }),
  getSchema: () => invoke<SchemaDocument>("get_schema"),
  formatNoteProperties: (content: string, patch: Record<string, unknown>) =>
    invoke<string>("format_note_properties", { content, patch }),
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
  knowledgeGraph: (focus?: string) =>
    invoke<KnowledgeGraph>("knowledge_graph_snapshot", { focus: focus ?? null }),
  unresolvedLinks: () => invoke<UnresolvedLink[]>("unresolved_links"),
  listTags: () => invoke<TagCount[]>("list_tags"),
  search: (query: string, limit = 50) => invoke<SearchHit[]>("search", { query, limit }),
  createBackup: (destination: string) => invoke<BackupSummary>("create_backup", { destination }),
  restoreBackup: (archive: string, destinationParent: string) =>
    invoke<RestoreSummary>("restore_backup", { archive, destinationParent }),
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
