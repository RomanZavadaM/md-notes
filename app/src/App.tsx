import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { emit, listen } from "@tauri-apps/api/event";
import { getAllWindows, getCurrentWindow } from "@tauri-apps/api/window";
import { ask, message, open, save as saveDialog } from "@tauri-apps/plugin-dialog";
import {
  api,
  joinPath,
  parentPath,
  KEY_MISSING,
  OPENED_MARKDOWN_FILES,
  VAULT_CHANGED,
  type AttachmentInfo,
  type Note,
  type RuntimePlatform,
  type StandaloneMarkdown,
  type TemplateInfo,
  type TreeEntry,
  type VaultInfo,
} from "./api";
import { Editor } from "./components/Editor";
import { ImportKeyDialog } from "./components/EncryptionDialogs";
import { AppMenu } from "./components/AppMenu";
import { GitPanel } from "./components/GitPanel";
import { openEditorWindow } from "./editorWindowApi";
import { GRAPH_ACTIVE_NOTE, GRAPH_OPEN_NOTE, openGraphWindow, type GraphActiveNote } from "./graphWindowApi";
import { GRAPH_STRINGS } from "./i18n/graph";
import { IMPORTER_STRINGS } from "./i18n/importer";
import { MENU_STRINGS } from "./i18n/menu";
import { FileTree } from "./components/FileTree";
import { LinksPanel } from "./components/LinksPanel";
import { NameDialog } from "./components/NameDialog";
import { Preview } from "./components/Preview";
import { QuickSwitcher } from "./components/QuickSwitcher";
import { SearchPanel } from "./components/SearchPanel";
import { TagsPanel } from "./components/TagsPanel";
import { AttachmentsPanel } from "./components/AttachmentsPanel";
import { AboutDialog, COPYRIGHT } from "./components/AboutDialog";
import { LANGUAGES, useI18n, type LanguageCode } from "./i18n";
import { ENCRYPTION_STRINGS } from "./i18n/encryption";
import { BACKUP_STRINGS } from "./i18n/backup";
import { getRememberedVault, storage, useStoredState } from "./storage";

type ViewMode = "edit" | "split" | "preview";
type Theme = "system" | "light" | "dark";
type SidebarTab = "files" | "search" | "tags" | "attachments";
type DialogState =
  | { kind: "note"; dir: string }
  | { kind: "folder"; dir: string }
  | { kind: "rename"; entry: TreeEntry };

type OpenVaultDocument = {
  note: Note;
  draft: string;
  savedContent: string;
  editorVersion: number;
};

const AUTOSAVE_MS = 800;
const LAST_VAULT_KEY = "mdnotes.lastVault";
const IMAGE_ATTACHMENT = /\.(png|jpe?g|gif|webp|svg|bmp|avif)$/i;
const isNarrow = () => window.matchMedia("(max-width: 720px)").matches;

export default function App() {
  const { t, language, setLanguage } = useI18n();
  const [aboutOpen, setAboutOpen] = useState(false);
  const [encrypted, setEncrypted] = useState(false);
  const [gitOpen, setGitOpen] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const [keyPrompt, setKeyPrompt] = useState<string | null>(null);
  // Last vault that was open; offered as a way back from a standalone Markdown file.
  const lastVaultRef = useRef<string | null>(null);
  const [platform, setPlatform] = useState<RuntimePlatform>("unknown");
  const [vault, setVault] = useState<VaultInfo | null>(null);
  const [standalone, setStandalone] = useState<StandaloneMarkdown | null>(null);
  const [external, setExternal] = useState<StandaloneMarkdown | null>(null);
  const [externalDraft, setExternalDraft] = useState("");
  const [externalSaved, setExternalSaved] = useState("");
  const [activeDocument, setActiveDocument] = useState<"vault" | "external">("vault");
  const [tree, setTree] = useState<TreeEntry[]>([]);
  const [openDocuments, setOpenDocuments] = useState<OpenVaultDocument[]>([]);
  const openDocumentsRef = useRef<OpenVaultDocument[]>([]);
  const [note, setNote] = useState<Note | null>(null);
  const [draft, setDraft] = useState("");
  const [selected, setSelected] = useState<TreeEntry | null>(null);
  const [selectedEntries, setSelectedEntries] = useState<TreeEntry[]>([]);
  const [dialog, setDialog] = useState<DialogState | null>(null);
  const [switcherOpen, setSwitcherOpen] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [sidebarOpen, setSidebarOpen] = useState(() => !isNarrow());
  const [sidebarTab, setSidebarTab] = useState<SidebarTab>("files");
  const [mode, setMode] = useStoredState<ViewMode>("mdnotes.mode", isNarrow() ? "preview" : "split");
  const [theme, setTheme] = useStoredState<Theme>("mdnotes.theme", "system");
  const [linksPanel, setLinksPanel] = useStoredState<"on" | "off">("mdnotes.linksPanel", isNarrow() ? "off" : "on");
  const [refreshKey, setRefreshKey] = useState(0);
  const [editorVersion, setEditorVersion] = useState(0);

  const noteRef = useRef<Note | null>(null);
  const draftRef = useRef("");
  const savedRef = useRef("");
  const closingRef = useRef(false);
  const [savedContent, setSavedContent] = useState("");
  const dirty = note !== null && draft !== savedContent;

  const files = useMemo(() => {
    const out: string[] = [];
    const walk = (entries: TreeEntry[]) => {
      for (const entry of entries) {
        if (entry.kind === "dir") walk(entry.children ?? []);
        else out.push(entry.path);
      }
    };
    walk(tree);
    return out;
  }, [tree]);

  const report = useCallback((e: unknown) => setError(String(e)), []);
  const bump = useCallback(() => setRefreshKey((k) => k + 1), []);

  useEffect(() => {
    document.documentElement.dataset.theme = theme === "system" ? "" : theme;
  }, [theme]);

  const setDocuments = useCallback((documents: OpenVaultDocument[]) => {
    openDocumentsRef.current = documents;
    setOpenDocuments(documents);
  }, []);

  const updateDocument = useCallback(
    (path: string, updater: (document: OpenVaultDocument) => OpenVaultDocument) => {
      setDocuments(openDocumentsRef.current.map((document) => (document.note.path === path ? updater(document) : document)));
    },
    [setDocuments],
  );

  const activateDocument = useCallback((document: OpenVaultDocument | null) => {
    const n = document?.note ?? null;
    noteRef.current = n;
    draftRef.current = document?.draft ?? "";
    savedRef.current = document?.savedContent ?? "";
    setNote(n);
    setDraft(document?.draft ?? "");
    setSavedContent(document?.savedContent ?? "");
    if (document) setEditorVersion(document.editorVersion);
  }, []);

  const showNote = useCallback(
    (n: Note | null) => {
      if (!n) {
        activateDocument(null);
        return;
      }
      const document: OpenVaultDocument = {
        note: n,
        draft: n.content,
        savedContent: n.content,
        editorVersion: 0,
      };
      const existing = openDocumentsRef.current.findIndex((item) => item.note.path === n.path);
      const documents =
        existing >= 0
          ? openDocumentsRef.current.map((item, index) => (index === existing ? document : item))
          : [...openDocumentsRef.current, document];
      setDocuments(documents);
      activateDocument(document);
    },
    [activateDocument, setDocuments],
  );

  const refreshTree = useCallback(async () => {
    try {
      setTree(await api.getTree());
    } catch (e) {
      report(e);
    }
  }, [report]);

  const saveVaultDocument = useCallback(
    async (path: string) => {
      const document = openDocumentsRef.current.find((item) => item.note.path === path);
      if (!document || document.draft === document.savedContent) return true;
      const content = document.draft;
      try {
        const disk = await api.readNote(path);
        if (disk.content !== document.savedContent) {
          setError(t.externalChange);
          return false;
        }
        const saved = await api.saveNote(path, content);
        const current = openDocumentsRef.current.find((item) => item.note.path === path);
        if (!current) return true;
        const next: OpenVaultDocument = {
          ...current,
          note: saved,
          savedContent: content,
        };
        updateDocument(path, () => next);
        if (noteRef.current?.path === path) {
          noteRef.current = saved;
          savedRef.current = content;
          setNote(saved);
          setSavedContent(content);
        }
        bump();
        return true;
      } catch (e) {
        report(e);
        return false;
      }
    },
    [bump, report, t.externalChange, updateDocument],
  );

  const save = useCallback(async (): Promise<boolean> => {
    const current = noteRef.current;
    const content = draftRef.current;
    if (!current || content === savedRef.current) return true;
    if (!standalone) {
      return saveVaultDocument(current.path);
    }
    try {
      const saved = (await api.saveMarkdownFile(standalone.path, content)).note;
      savedRef.current = content;
      setSavedContent(content);
      noteRef.current = saved;
      setNote(saved);
      bump();
      return true;
    } catch (e) {
      report(e);
      return false;
    }
  }, [standalone, saveVaultDocument, report, bump]);

  const openVaultAt = useCallback(
    async (path: string) => {
      try {
        if (external && externalDraft !== externalSaved) {
          const discard = await ask(t.unsavedChanges, { kind: "warning" });
          if (!discard) return;
        }
        if (standalone) {
          if (!(await save())) return;
        } else {
          const paths = openDocumentsRef.current.map((document) => document.note.path);
          for (const openPath of paths) {
            if (!(await saveVaultDocument(openPath))) return;
          }
        }
        const info = await api.openVault(path);
        setKeyPrompt(null);
        setEncrypted(await api.encryptionStatus(info.root).then((status) => status.encrypted, () => false));
        setStandalone(null);
        setExternal(null);
        setActiveDocument("vault");
        setVault(info);
        setSelected(null);
        setSelectedEntries([]);
        setDocuments([]);
        showNote(null);
        storage.set(LAST_VAULT_KEY, path);
        lastVaultRef.current = path;
        setTree(await api.getTree());
        bump();
      } catch (e) {
        storage.remove(LAST_VAULT_KEY);
        // An encrypted vault without a key on this device asks for the key.
        if (String(e).includes(KEY_MISSING)) setKeyPrompt(path);
        else report(e);
      }
    },
    [
      save,
      saveVaultDocument,
      showNote,
      report,
      bump,
      external,
      externalDraft,
      externalSaved,
      standalone,
      t.unsavedChanges,
      setDocuments,
    ],
  );

  const pickVault = async () => {
    const dir = await open({ directory: true, multiple: false, title: t.pickFolderTitle });
    if (typeof dir === "string") await openVaultAt(dir);
  };

  const openStandaloneAt = useCallback(
    async (path: string) => {
      try {
        if (!(await save())) return;
        if (external && external.path !== path && externalDraft !== externalSaved) {
          const replace = await ask(t.unsavedChanges, { kind: "warning" });
          if (!replace) return;
        }
        const document = await api.openMarkdownFile(path);
        if (vault) {
          setExternal(document);
          setExternalDraft(document.note.content);
          setExternalSaved(document.note.content);
          setActiveDocument("external");
          return;
        }
        setVault(null);
        setTree([]);
        setSelected(null);
        setSelectedEntries([]);
        setStandalone(document);
        showNote(document.note);
        setEditorVersion((v) => v + 1);
      } catch (e) {
        report(e);
      }
    },
    [save, showNote, report, vault, external, externalDraft, externalSaved, t.unsavedChanges],
  );

  const saveExternal = useCallback(async (): Promise<boolean> => {
    if (!external || externalDraft === externalSaved) return true;
    try {
      const disk = await api.openMarkdownFile(external.path);
      if (disk.note.content !== externalSaved) {
        setError(t.externalChange);
        return false;
      }
      const updated = await api.saveMarkdownFile(external.path, externalDraft);
      setExternal(updated);
      setExternalSaved(externalDraft);
      return true;
    } catch (error) {
      report(error);
      return false;
    }
  }, [external, externalDraft, externalSaved, report, t.externalChange]);

  const openStandaloneAtRef = useRef(openStandaloneAt);
  const openVaultAtRef = useRef(openVaultAt);
  useEffect(() => {
    openStandaloneAtRef.current = openStandaloneAt;
  }, [openStandaloneAt]);
  useEffect(() => {
    openVaultAtRef.current = openVaultAt;
  }, [openVaultAt]);

  const isDesktop = platform === "windows" || platform === "linux" || platform === "macos";

  const saveAllOpenDocuments = useCallback(async (): Promise<boolean> => {
    if (standalone) return save();
    const paths = openDocumentsRef.current.map((document) => document.note.path);
    for (const path of paths) {
      if (!(await saveVaultDocument(path))) return false;
    }
    if (!(await saveExternal())) return false;
    return true;
  }, [save, saveExternal, saveVaultDocument, standalone]);

  const closeDetachedEditors = useCallback(async (): Promise<boolean> => {
    const detached = (await getAllWindows()).filter((window) => window.label.startsWith("editor-"));
    for (const window of detached) {
      await window.close();
    }
    if (detached.length === 0) return true;

    for (let attempt = 0; attempt < 50; attempt += 1) {
      await new Promise((resolve) => window.setTimeout(resolve, 100));
      const remaining = (await getAllWindows()).some((window) => window.label.startsWith("editor-"));
      if (!remaining) return true;
    }

    setError(t.unsavedChanges);
    return false;
  }, [t.unsavedChanges]);

  const createVaultBackup = useCallback(async () => {
    if (!vault || !isDesktop) return;
    if (!(await saveAllOpenDocuments())) return;
    const strings = BACKUP_STRINGS[language];
    const safeName = vault.name.replace(/[<>:"/\\|?*\u0000-\u001f]/g, "_").trim() || "MD-Notes";
    const date = new Date().toISOString().slice(0, 10);
    const picked = await saveDialog({
      title: strings.createTitle,
      defaultPath: `MD-Notes-${safeName}-${date}.zip`,
      filters: [{ name: strings.archiveFilter, extensions: ["zip"] }],
    });
    if (typeof picked !== "string") return;
    const destination = picked.toLowerCase().endsWith(".zip") ? picked : `${picked}.zip`;
    try {
      const summary = await api.createBackup(destination);
      const detail = summary.encrypted
        ? `${strings.created(summary.files, summary.archive)}\n\n${strings.createdEncrypted}`
        : strings.created(summary.files, summary.archive);
      await message(detail, { title: strings.createTitle, kind: "info" });
    } catch (error) {
      report(error);
    }
  }, [isDesktop, language, report, saveAllOpenDocuments, vault]);

  const restoreVaultBackup = useCallback(async () => {
    if (!isDesktop) return;
    const strings = BACKUP_STRINGS[language];
    const archive = await open({
      directory: false,
      multiple: false,
      title: strings.restoreArchiveTitle,
      filters: [{ name: strings.archiveFilter, extensions: ["zip"] }],
    });
    if (typeof archive !== "string") return;
    const destinationParent = await open({
      directory: true,
      multiple: false,
      title: strings.restoreFolderTitle,
    });
    if (typeof destinationParent !== "string") return;
    try {
      const summary = await api.restoreBackup(archive, destinationParent);
      const openNow = await ask(
        `${strings.restored(summary.files, summary.path)}\n\n${strings.openRestored}`,
        { title: strings.openRestoredTitle, kind: "info" },
      );
      if (openNow) await openVaultAt(summary.path);
    } catch (error) {
      report(error);
    }
  }, [isDesktop, language, openVaultAt, report]);

  const quitApp = useCallback(async () => {
    if (!(await saveAllOpenDocuments())) return;
    if (!(await closeDetachedEditors())) return;
    await api.quitApp();
  }, [closeDetachedEditors, saveAllOpenDocuments]);

  useEffect(() => {
    if (!isDesktop) return;
    const onKey = (event: KeyboardEvent) => {
      if ((event.ctrlKey || event.metaKey) && !event.shiftKey && !event.altKey && event.key.toLowerCase() === "q") {
        event.preventDefault();
        void quitApp();
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [isDesktop, quitApp]);

  useEffect(() => {
    if (!isDesktop) return;
    const appWindow = getCurrentWindow();
    let disposed = false;
    let unlisten: (() => void) | undefined;
    void appWindow.onCloseRequested(async (event) => {
      if (closingRef.current) return;
      const hasUnsaved =
        standalone
          ? draftRef.current !== savedRef.current
          : openDocumentsRef.current.some((document) => document.draft !== document.savedContent) ||
            Boolean(external && externalDraft !== externalSaved);
      if (!hasUnsaved) return;
      event.preventDefault();
      if (!(await saveAllOpenDocuments())) return;
      closingRef.current = true;
      await appWindow.destroy();
    }).then((fn) => {
      if (disposed) fn();
      else unlisten = fn;
    });
    return () => {
      disposed = true;
      unlisten?.();
    };
  }, [external, externalDraft, externalSaved, isDesktop, saveAllOpenDocuments, standalone]);


  // Toasts disappear on their own (a click still closes them at once).
  useEffect(() => {
    if (!notice) return;
    const timer = window.setTimeout(() => setNotice(null), 6000);
    return () => window.clearTimeout(timer);
  }, [notice]);
  useEffect(() => {
    if (!error) return;
    const timer = window.setTimeout(() => setError(null), 12000);
    return () => window.clearTimeout(timer);
  }, [error]);

  useEffect(() => {
    void api.runtimePlatform().then(setPlatform).catch(() => setPlatform("unknown"));
    let disposed = false;
    let unlisten: (() => void) | undefined;

    const openPendingMarkdown = async (fallbackToLastVault: boolean) => {
      try {
        const paths = await api.startupMarkdownFiles();
        if (disposed) return;
        const path = paths[0];
        if (path) {
          await openStandaloneAtRef.current(path);
          return;
        }
      } catch {
        // Fall back to the previous vault only during initial startup.
      }
      if (disposed || !fallbackToLastVault) return;
      const last = storage.get(LAST_VAULT_KEY);
      if (last) await openVaultAtRef.current(last);
    };

    // Attach the OS-open listener before draining startup files. On macOS this
    // closes the race between Finder's open event and frontend initialization.
    void listen<string[]>(OPENED_MARKDOWN_FILES, () => {
      void openPendingMarkdown(false);
    }).then((fn) => {
      if (disposed) {
        fn();
        return;
      }
      unlisten = fn;
      void openPendingMarkdown(true);
    });

    return () => {
      disposed = true;
      unlisten?.();
    };
  }, []);

  const configureMarkdownAssociation = async () => {
    try {
      const result = await api.configureMarkdownAssociation();
      if (result === "systemChoice") setError(t.markdownAssociationSystemChoice);
      else if (result === "defaultSet") setError(t.markdownAssociationDefaultSet);
      else setError(t.markdownAssociationUnsupported);
    } catch (e) {
      report(e);
    }
  };

  const importFolder = async () => {
    const strings = IMPORTER_STRINGS[language];
    const dir = await open({ directory: true, multiple: false, title: strings.pickTitle });
    if (typeof dir !== "string") return;
    try {
      let result = await api.importFolder(dir, false);
      // Files that are already there are left alone unless the user agrees to update them.
      if (result.skipped > 0 && (await ask(strings.overwritePrompt(result.skipped), { kind: "warning" }))) {
        const second = await api.importFolder(dir, true);
        result = { ...result, updated: second.updated, skipped: second.skipped };
      }
      const mergedNote = result.merged > 0 ? ` ${strings.merged(result.merged, result.conflicts)}` : "";
      setNotice(strings.result(result.created, result.updated, result.skipped, result.rejected) + mergedNote);
      await refreshTree();
      bump();
    } catch (e) {
      report(e);
    }
  };

  const showDuplicates = async () => {
    const strings = IMPORTER_STRINGS[language];
    try {
      const groups = await api.possibleDuplicates();
      if (groups.length === 0) {
        await message(strings.duplicatesNone, { title: strings.duplicatesTitle, kind: "info" });
        return;
      }
      const shown = groups.slice(0, 12).map((group) => {
        const names = group.notes.map((n) => `${n.title} (${n.path})`).join("  =  ");
        return group.sameCode ? `${names}  [${strings.duplicatesSameCode}]` : names;
      });
      const more = groups.length > shown.length ? `\n… ${groups.length - shown.length}` : "";
      const text = [strings.duplicatesFound(groups.length), "", ...shown].join("\n");
      await message(`${text}${more}\n\n${strings.duplicatesHint}`, {
        title: strings.duplicatesTitle,
        kind: "info",
      });
    } catch (e) {
      report(e);
    }
  };

  const pickMarkdownFile = async () => {
    const file = await open({
      directory: false,
      multiple: false,
      title: t.pickMarkdownFileTitle,
      filters: [{ name: "Markdown", extensions: ["md", "markdown"] }],
    });
    if (typeof file === "string") await openStandaloneAt(file);
  };

  const openNote = useCallback(
    async (path: string) => {
      try {
        const existing = openDocumentsRef.current.find((document) => document.note.path === path);
        if (existing) {
          activateDocument(existing);
        } else {
          const opened = await api.readNote(path);
          const document: OpenVaultDocument = {
            note: opened,
            draft: opened.content,
            savedContent: opened.content,
            editorVersion: 0,
          };
          setDocuments([...openDocumentsRef.current, document]);
          activateDocument(document);
        }
        setActiveDocument("vault");
        if (isNarrow()) setSidebarOpen(false);
      } catch (e) {
        report(e);
      }
    },
    [activateDocument, report, setDocuments],
  );

  const openGraphNote = useCallback(
    async (path: string) => {
      // Detached graph navigation must behave exactly like ordinary note
      // opening in main: reuse an existing tab or append a new one.
      await openNote(path);
    },
    [openNote],
  );

  const closeDocument = useCallback(
    async (path: string) => {
      const documents = openDocumentsRef.current;
      const index = documents.findIndex((document) => document.note.path === path);
      if (index < 0) return;
      if (!(await saveVaultDocument(path))) return;
      const next = openDocumentsRef.current.filter((document) => document.note.path !== path);
      setDocuments(next);
      if (noteRef.current?.path !== path) return;
      const fallback = next[Math.min(index - 1, next.length - 1)] ?? next[0] ?? null;
      activateDocument(fallback);
    },
    [activateDocument, saveVaultDocument, setDocuments],
  );

  const detachDocument = useCallback(async (path: string) => {
    if (!vault || !isDesktop) return;
    const documents = openDocumentsRef.current;
    const index = documents.findIndex((document) => document.note.path === path);
    if (index < 0) return;
    const document = documents[index];
    if (!(await saveVaultDocument(path))) return;
    try {
      await openEditorWindow(document.note.path, vault.root, document.note.title);
      const next = openDocumentsRef.current.filter((item) => item.note.path !== path);
      setDocuments(next);
      if (noteRef.current?.path === path) {
        const fallback = next[Math.min(index - 1, next.length - 1)] ?? next[0] ?? null;
        // Keep the currently visible document surface unchanged. If an external
        // Markdown file is active, only update the vault's background fallback.
        activateDocument(fallback);
      }
    } catch (e) {
      report(e);
    }
  }, [activateDocument, isDesktop, report, saveVaultDocument, setDocuments, vault]);

  const detachActiveDocument = useCallback(async () => {
    const current = noteRef.current;
    if (!current) return;
    await detachDocument(current.path);
  }, [detachDocument]);

  const createAndOpen = useCallback(
    async (dir: string, title: string, template?: string) => {
      try {
        const created = await api.createNote(dir, title, template);
        await refreshTree();
        bump();
        await openNote(created.path);
      } catch (e) {
        report(e);
      }
    },
    [openNote, refreshTree, bump, report],
  );

  const openToday = useCallback(async () => {
    try {
      const daily = await api.openDaily();
      await refreshTree();
      bump();
      await openNote(daily.path);
    } catch (e) {
      report(e);
    }
  }, [openNote, refreshTree, bump, report]);

  const [templates, setTemplates] = useState<TemplateInfo[]>([]);
  useEffect(() => {
    if (!vault) return;
    api
      .listTemplates()
      .then(setTemplates)
      .catch(() => setTemplates([]));
  }, [vault, refreshKey]);
  const templateChoices = [
    { value: "", label: t.emptyTemplate },
    ...templates.filter((tpl) => tpl.name !== "daily").map((tpl) => ({ value: tpl.name, label: tpl.label })),
  ];

  const openLink = useCallback(
    async (target: string) => {
      const name = target.split("#")[0].trim();
      if (!name) return;
      try {
        const path = await api.resolveLink(name);
        if (path) await openNote(path);
        else await createAndOpen("", name);
      } catch (e) {
        report(e);
      }
    },
    [openNote, createAndOpen, report],
  );

  const onExternalChange = useCallback(
    async (paths: string[]) => {
      await refreshTree();
      bump();
      const current = noteRef.current;
      if (!current || !paths.includes(current.path)) return;
      try {
        const fresh = await api.readNote(current.path);
        if (fresh.content === savedRef.current) return;
        if (draftRef.current !== savedRef.current) {
          setError(t.externalChange);
          return;
        }
        showNote(fresh);
        setEditorVersion((v) => v + 1);
      } catch {
        showNote(null);
      }
    },
    [refreshTree, bump, showNote, t],
  );

  // Detached graph window: it follows the active note and asks this window to open notes.
  const activePath = note?.path ?? null;
  useEffect(() => {
    if (!isDesktop || activePath === null) return;
    const payload: GraphActiveNote = { path: activePath };
    void emit(GRAPH_ACTIVE_NOTE, payload);
  }, [isDesktop, activePath]);

  useEffect(() => {
    if (!isDesktop) return;
    let cancelled = false;
    let unlisten: (() => void) | undefined;
    void listen<string>(GRAPH_OPEN_NOTE, (event) => {
      void openGraphNote(event.payload);
    }).then((fn) => {
      if (cancelled) fn();
      else unlisten = fn;
    });
    return () => {
      cancelled = true;
      unlisten?.();
    };
  }, [isDesktop, openGraphNote]);

  useEffect(() => {
    if (!vault) return;
    let disposed = false;
    let unlisten: (() => void) | undefined;
    void listen<string[]>(VAULT_CHANGED, (event) => void onExternalChange(event.payload)).then((fn) => {
      if (disposed) fn();
      else unlisten = fn;
    });
    return () => {
      disposed = true;
      unlisten?.();
    };
  }, [vault, onExternalChange]);

  useEffect(() => {
    if (!dirty) return;
    const timer = window.setTimeout(() => void save(), AUTOSAVE_MS);
    return () => window.clearTimeout(timer);
  }, [draft, dirty, save]);

  useEffect(() => {
    const flush = () => void save();
    window.addEventListener("beforeunload", flush);
    return () => window.removeEventListener("beforeunload", flush);
  }, [save]);

  useEffect(() => {
    if (!vault) return;
    const onKey = (e: KeyboardEvent) => {
      const mod = e.ctrlKey || e.metaKey;
      if (!mod) return;
      const key = e.key.toLowerCase();
      if (!e.shiftKey && (key === "o" || key === "p")) {
        e.preventDefault();
        setSwitcherOpen(true);
      } else if (e.shiftKey && key === "f") {
        e.preventDefault();
        setSidebarOpen(true);
        setSidebarTab("search");
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [vault]);

  const onChange = useCallback(
    (value: string) => {
      draftRef.current = value;
      setDraft(value);
      const path = noteRef.current?.path;
      if (path && !standalone) {
        updateDocument(path, (document) => ({ ...document, draft: value }));
      }
    },
    [standalone, updateDocument],
  );

  const insertAttachment = useCallback(
    (attachment: AttachmentInfo) => {
      if (!noteRef.current) return;
      const encodedPath = `/${attachment.path.split("/").map(encodeURIComponent).join("/")}`;
      const link = IMAGE_ATTACHMENT.test(attachment.name)
        ? `![${attachment.name}](<${encodedPath}>)`
        : `[${attachment.name}](<${encodedPath}>)`;
      const current = draftRef.current;
      const separator = current.length === 0 || current.endsWith("\n") ? "" : "\n";
      const next = `${current}${separator}${link}\n`;
      onChange(next);
      setError(t.attachmentInserted(attachment.name));
    },
    [onChange, t],
  );

  const targetDir = () => {
    if (!selected) return "";
    return selected.kind === "dir" ? selected.path : parentPath(selected.path);
  };

  const applyTreeSelection = useCallback((entries: TreeEntry[]) => {
    setSelectedEntries(entries);
    setSelected(entries.length > 0 ? entries[entries.length - 1] : null);
  }, []);

  const submitDialog = async (value: string, template: string) => {
    if (!dialog) return;
    setDialog(null);
    try {
      if (dialog.kind === "note") {
        await createAndOpen(dialog.dir, value, template || undefined);
      } else if (dialog.kind === "folder") {
        await api.createFolder(dialog.dir, value);
        await refreshTree();
      } else {
        const { entry } = dialog;
        let name = value;
        if (entry.kind === "note" && !/\.(md|markdown)$/i.test(name)) name += ".md";
        await save();
        const outcome = await api.renameEntry(entry.path, joinPath(parentPath(entry.path), name));
        const moved = outcome.path;
        await refreshTree();
        bump();
        setSelected(null);
        setSelectedEntries([]);
        const current = noteRef.current;
        if (current && current.path === entry.path) await openNote(moved);
        else if (current && current.path.startsWith(`${entry.path}/`)) {
          await openNote(moved + current.path.slice(entry.path.length));
        } else if (current && outcome.updated.includes(current.path)) {
          await openNote(current.path);
        }
        if (outcome.updated.length > 0) {
          setError(t.linksUpdated(outcome.updated.length));
        }
      }
    } catch (e) {
      report(e);
    }
  };

  const trashSelected = async () => {
    const chosen = selectedEntries.length > 0 ? selectedEntries : selected ? [selected] : [];
    if (chosen.length === 0) return;

    const roots = [...chosen]
      .sort((left, right) => left.path.length - right.path.length || left.path.localeCompare(right.path))
      .filter(
        (entry, index, items) =>
          !items
            .slice(0, index)
            .some((parent) => parent.kind === "dir" && entry.path.startsWith(`${parent.path}/`)),
      );
    const confirmed = await ask(t.trashConfirm(roots.map((entry) => entry.name).join(", ")), {
      title: t.trashTitle,
      kind: "warning",
    });
    if (!confirmed) return;
    if (!(await saveAllOpenDocuments())) return;

    try {
      for (const entry of roots) {
        await api.trashEntry(entry.path);
      }

      const isRemoved = (path: string) =>
        roots.some((entry) => path === entry.path || (entry.kind === "dir" && path.startsWith(`${entry.path}/`)));
      const remaining = openDocumentsRef.current.filter((document) => !isRemoved(document.note.path));
      const current = noteRef.current;
      setDocuments(remaining);
      if (current && isRemoved(current.path)) {
        activateDocument(remaining.length > 0 ? remaining[remaining.length - 1] : null);
      }

      setSelected(null);
      setSelectedEntries([]);
      await refreshTree();
      bump();
    } catch (e) {
      report(e);
    }
  };

  const fileMenu = (
    <AppMenu
      label={MENU_STRINGS[language].file}
      items={[
        { label: t.otherVault, onSelect: () => void pickVault() },
        { label: t.openMarkdownFile, onSelect: () => void pickMarkdownFile() },
        ...(vault ? [{ label: IMPORTER_STRINGS[language].button, onSelect: () => void importFolder() }] : []),
        ...(vault ? [{ label: IMPORTER_STRINGS[language].duplicatesButton, onSelect: () => void showDuplicates() }] : []),
        ...(vault && isDesktop
          ? [{ label: BACKUP_STRINGS[language].create, onSelect: () => void createVaultBackup(), separatorBefore: true }]
          : []),
        ...(isDesktop
          ? [{ label: BACKUP_STRINGS[language].restore, onSelect: () => void restoreVaultBackup() }]
          : []),
        ...(vault && isDesktop
          ? [
              {
                label: GRAPH_STRINGS[language].detach,
                onSelect: () => void openGraphWindow(note ? "local" : "global", note?.path ?? "", GRAPH_STRINGS[language].windowTitle),
              },
            ]
          : []),
        ...(vault && isDesktop ? [{ label: "Git…", onSelect: () => setGitOpen(true) }] : []),
        ...(isDesktop ? [{ label: t.makeMarkdownDefault, onSelect: () => void configureMarkdownAssociation() }] : []),
        { label: t.about, onSelect: () => setAboutOpen(true), separatorBefore: true },
        ...(isDesktop ? [{ label: `${MENU_STRINGS[language].quit} (Ctrl+Q)`, onSelect: () => void quitApp(), separatorBefore: true }] : []),
      ]}
    />
  );

  if (!vault && !standalone) {
    return (
      <div className="welcome">
        <h1>MD Notes</h1>
        <p>{t.appTagline}</p>
        <button type="button" className="primary" onClick={() => void pickVault()}>
          {t.openFolder}
        </button>
        <button type="button" onClick={() => void pickMarkdownFile()}>
          {t.openMarkdownFile}
        </button>
        {(platform === "windows" || platform === "linux" || platform === "macos") && (
          <button type="button" onClick={() => void configureMarkdownAssociation()}>
            {t.makeMarkdownDefault}
          </button>
        )}
        {error && <p className="welcome-error">{error}</p>}
        <select
          className="welcome-language"
          value={language}
          onChange={(e) => setLanguage(e.target.value as LanguageCode)}
          aria-label={t.language}
        >
          {LANGUAGES.map((lang) => (
            <option key={lang.code} value={lang.code} title={lang.ukrainianDescription}>
              {lang.flag} {lang.nativeName}
            </option>
          ))}
        </select>
        <button type="button" className="link-button" onClick={() => setAboutOpen(true)}>
          {t.about}
        </button>
        <p className="welcome-copyright">{COPYRIGHT}</p>
        {aboutOpen && <AboutDialog onClose={() => setAboutOpen(false)} />}
        {keyPrompt && (
          <ImportKeyDialog
            path={keyPrompt}
            onImported={() => void openVaultAt(keyPrompt)}
            onCancel={() => setKeyPrompt(null)}
          />
        )}
      </div>
    );
  }

  if (standalone && note) {
    return (
      <div className="app">
        <header className="toolbar">
          {fileMenu}
          {(lastVaultRef.current ?? getRememberedVault()) && (
            <button
              type="button"
              className="back-to-vault"
              onClick={() => {
                const target = lastVaultRef.current ?? getRememberedVault();
                if (target) void openVaultAt(target);
              }}
            >
              {t.backToVault}
            </button>
          )}
          <span className="vault-name" title={standalone.path}>
            {standalone.name}
          </span>
          <div className="segmented" role="group" aria-label={t.modeGroup}>
            {(
              [
                ["edit", t.modeEdit],
                ["split", t.modeSplit],
                ["preview", t.modePreview],
              ] as const
            ).map(([value, label]) => (
              <button
                key={value}
                type="button"
                className={`${mode === value ? "on" : ""} mode-${value}`}
                onClick={() => setMode(value)}
              >
                {label}
              </button>
            ))}
          </div>
          <button type="button" onClick={() => void pickMarkdownFile()}>
            {t.openMarkdownFile}
          </button>
          <button type="button" onClick={() => void pickVault()}>
            {t.openFolder}
          </button>
          {(platform === "windows" || platform === "linux" || platform === "macos") && (
            <button type="button" onClick={() => void configureMarkdownAssociation()}>
              {t.makeMarkdownDefault}
            </button>
          )}
        </header>

        <main className={`workspace mode-${mode}`}>
          {mode !== "preview" && (
            <section className="pane pane-editor">
              <Editor
                docKey={`${standalone.path}#${editorVersion}`}
                value={draft}
                onChange={onChange}
                onSave={() => void save()}
              />
            </section>
          )}
          {mode !== "edit" && (
            <section className="pane pane-preview">
              <Preview
                content={draft}
                notePath={standalone.name}
                vaultRoot={standalone.directory}
                files={[]}
                onOpenLink={() => {}}
                onOpenPath={() => {}}
              />
            </section>
          )}
        </main>

        <footer className="statusbar">
          <span>{standalone.path}</span>
          <span className="save-state">{dirty ? t.statusUnsaved : t.statusSaved}</span>
        </footer>

        {error && (
          <div className="toast" role="alert" onClick={() => setError(null)}>
            {error}
          </div>
        )}
      </div>
    );
  }

  if (!vault) return null;

  return (
    <div className="app">
      <header className="toolbar">
        {fileMenu}
        <button type="button" className="icon" title={t.sidebarToggle} onClick={() => setSidebarOpen((v) => !v)}>
          ☰
        </button>
        <span className="vault-name" title={vault.root}>
          {vault.name}
        </span>
        {encrypted && (
          <span className="encrypted-badge" title={ENCRYPTION_STRINGS[language].badgeTitle}>
            🔒 {ENCRYPTION_STRINGS[language].badge}
          </span>
        )}
        {(platform === "windows" || platform === "linux" || platform === "macos") && (
          <button type="button" className="git-button" title="Git" onClick={() => setGitOpen(true)}>
            Git
          </button>
        )}
        <button type="button" className="search-button" title={t.goToNote} onClick={() => setSwitcherOpen(true)}>
          <span className="note-title">
            {note ? note.title : t.goToNotePlaceholder}
            {dirty && <span className="dirty" title={t.unsavedChanges} />}
          </span>
        </button>
        {isDesktop && note && activeDocument === "vault" && (
          <button type="button" title={t.openSeparateWindow} onClick={() => void detachActiveDocument()}>
            ↗ {t.openSeparateWindow}
          </button>
        )}
        <div className="segmented" role="group" aria-label={t.modeGroup}>
          {(
            [
              ["edit", t.modeEdit],
              ["split", t.modeSplit],
              ["preview", t.modePreview],
            ] as const
          ).map(([value, label]) => (
            <button
              key={value}
              type="button"
              className={`${mode === value ? "on" : ""} mode-${value}`}
              onClick={() => setMode(value)}
            >
              {label}
            </button>
          ))}
        </div>
        <button
          type="button"
          className={`icon ${linksPanel === "on" ? "on" : ""}`}
          title={t.linksPanelToggle}
          onClick={() => setLinksPanel(linksPanel === "on" ? "off" : "on")}
        >
          ⇆
        </button>
        <select value={theme} onChange={(e) => setTheme(e.target.value as Theme)} aria-label={t.theme}>
          <option value="system">{t.themeSystem}</option>
          <option value="light">{t.themeLight}</option>
          <option value="dark">{t.themeDark}</option>
        </select>
        <select
          value={language}
          onChange={(e) => setLanguage(e.target.value as LanguageCode)}
          aria-label={t.language}
          title={t.language}
        >
          {LANGUAGES.map((lang) => (
            <option key={lang.code} value={lang.code} title={lang.ukrainianDescription}>
              {lang.flag} {lang.nativeName}
            </option>
          ))}
        </select>
        <button type="button" className="icon" title={t.about} onClick={() => setAboutOpen(true)}>
          ⓘ
        </button>
      </header>

      <div className="body">
        {sidebarOpen && (
          <aside className="sidebar">
            <div className="tabs" role="tablist">
              {(
                [
                  ["files", t.tabFiles],
                  ["search", t.tabSearch],
                  ["tags", t.tabTags],
                  ["attachments", t.tabAttachments],
                ] as const
              ).map(([value, label]) => (
                <button
                  key={value}
                  type="button"
                  role="tab"
                  aria-selected={sidebarTab === value}
                  className={sidebarTab === value ? "on" : ""}
                  onClick={() => setSidebarTab(value)}
                >
                  {label}
                </button>
              ))}
            </div>

            {sidebarTab === "files" && (
              <>
                <div className="sidebar-actions">
                  <button type="button" title={t.newNote} onClick={() => setDialog({ kind: "note", dir: targetDir() })}>
                    {t.newNoteButton}
                  </button>
                  <button type="button" title={t.todayTitle} onClick={() => void openToday()}>
                    {t.today}
                  </button>
                  <button type="button" title={t.newFolder} onClick={() => setDialog({ kind: "folder", dir: targetDir() })}>
                    {t.newFolderButton}
                  </button>
                  <button
                    type="button"
                    title={t.rename}
                    disabled={selectedEntries.length !== 1}
                    onClick={() => selectedEntries.length === 1 && setDialog({ kind: "rename", entry: selectedEntries[0] })}
                  >
                    ✎
                  </button>
                  <button
                    type="button"
                    title={t.moveToTrash}
                    disabled={selectedEntries.length === 0}
                    onClick={() => void trashSelected()}
                  >
                    🗑
                  </button>
                </div>
                <FileTree
                  entries={tree}
                  selectedPaths={selectedEntries.map((entry) => entry.path)}
                  activePath={note?.path ?? null}
                  onSelectionChange={applyTreeSelection}
                  onOpen={(entry) => {
                    if (entry.kind === "note") void openNote(entry.path);
                  }}
                />
              </>
            )}
            {sidebarTab === "search" && <SearchPanel refreshKey={refreshKey} onOpen={(p) => void openNote(p)} />}
            {sidebarTab === "tags" && <TagsPanel refreshKey={refreshKey} onOpen={(p) => void openNote(p)} />}
            {sidebarTab === "attachments" && (
              <AttachmentsPanel
                vaultRoot={vault.root}
                refreshKey={refreshKey}
                canInsert={note !== null}
                onInserted={insertAttachment}
                onChanged={() => {
                  void refreshTree();
                  bump();
                }}
                onError={report}
              />
            )}

            <div className="sidebar-footer">
              <button type="button" onClick={() => void pickMarkdownFile()}>
                {t.openMarkdownFile}
              </button>
              {(platform === "windows" || platform === "linux" || platform === "macos") && (
                <button type="button" onClick={() => void configureMarkdownAssociation()}>
                  {t.makeMarkdownDefault}
                </button>
              )}
              <button type="button" onClick={() => void importFolder()}>
                {IMPORTER_STRINGS[language].button}
              </button>
              <button type="button" onClick={() => void pickVault()}>
                {t.otherVault}
              </button>
            </div>
          </aside>
        )}

        <div className="document-area">
        {(openDocuments.length > 0 || external) && (
          <nav className="external-tabs" aria-label={t.goToNote}>
            {openDocuments.map((document) => (
              <span className="document-tab" key={document.note.path}>
                <button
                  type="button"
                  className={activeDocument === "vault" && note?.path === document.note.path ? "on" : ""}
                  onClick={() => {
                    activateDocument(document);
                    setActiveDocument("vault");
                  }}
                  title={document.note.path}
                >
                  {document.note.title}
                  {document.draft !== document.savedContent ? " •" : ""}
                </button>
                {isDesktop && (
                  <button
                    type="button"
                    className="document-tab-detach"
                    title={t.openSeparateWindow}
                    aria-label={t.openSeparateWindow}
                    onClick={() => void detachDocument(document.note.path)}
                  >
                    ↗
                  </button>
                )}
                <button type="button" className="document-tab-close" onClick={() => void closeDocument(document.note.path)}>
                  ×
                </button>
              </span>
            ))}
            {external && (
              <button
                type="button"
                className={activeDocument === "external" ? "on" : ""}
                onClick={() => setActiveDocument("external")}
                title={external.path}
              >
                {external.name}
                {externalDraft !== externalSaved ? " •" : ""}
              </button>
            )}
          </nav>
        )}
        <main className={`workspace mode-${mode}`}>
          {external && activeDocument === "external" ? (
            <>
              {mode !== "preview" && <section className="pane pane-editor"><Editor docKey={`${external.path}#external`} value={externalDraft} onChange={setExternalDraft} onSave={() => void saveExternal()} /></section>}
              {mode !== "edit" && <section className="pane pane-preview"><Preview content={externalDraft} notePath={external.name} vaultRoot={external.directory} files={[]} onOpenLink={() => {}} onOpenPath={() => {}} /></section>}
            </>
          ) : note ? (
            <>
              {mode !== "preview" && (
                <section className="pane pane-editor">
                  <Editor
                    docKey={`${note.path}#${editorVersion}`}
                    value={draft}
                    onChange={onChange}
                    onSave={() => void save()}
                  />
                </section>
              )}
              {mode !== "edit" && (
                <section className="pane pane-preview">
                  <Preview
                    content={draft}
                    notePath={note.path}
                    vaultRoot={vault.root}
                    files={files}
                    onOpenLink={(target) => void openLink(target)}
                    onOpenPath={(path) => void openNote(path)}
                  />
                </section>
              )}
            </>
          ) : (
            <div className="empty">{t.workspaceEmpty}</div>
          )}
        </main>

        </div>

        {linksPanel === "on" && note && (
          <LinksPanel
            note={note}
            refreshKey={refreshKey}
            onOpen={(p) => void openNote(p)}
            onOpenLink={(target) => void openLink(target)}
          />
        )}
      </div>

      {(note || external) && (
        <footer className="statusbar">
          {external && activeDocument === "external" ? (
            <>
              <span>{external.path}</span>
              <button type="button" onClick={() => void saveExternal()}>
                {externalDraft !== externalSaved ? t.statusUnsaved : t.statusSaved} · Ctrl+S
              </button>
            </>
          ) : note ? (
            <>
              <span>{note.path}</span>
              {note.tags.length > 0 && <span>{note.tags.map((tag) => `#${tag}`).join(" ")}</span>}
              <span>{t.statusLinks(note.links.length)}</span>
              {note.frontMatterError && <span className="warn">{t.statusPropertyError(note.frontMatterError)}</span>}
              <span className="save-state">{dirty ? t.statusUnsaved : t.statusSaved}</span>
            </>
          ) : null}
        </footer>
      )}

      {dialog && (
        <NameDialog
          title={dialog.kind === "note" ? t.newNote : dialog.kind === "folder" ? t.newFolder : t.rename}
          label={t.dialogName}
          initial={dialog.kind === "rename" ? dialog.entry.name.replace(/\.(md|markdown)$/i, "") : ""}
          submitText={dialog.kind === "rename" ? t.rename : t.create}
          choices={dialog.kind === "note" ? templateChoices : undefined}
          choiceLabel={t.dialogTemplate}
          onSubmit={(value, choice) => void submitDialog(value, choice)}
          onCancel={() => setDialog(null)}
        />
      )}

      {switcherOpen && (
        <QuickSwitcher
          onOpen={(path) => {
            setSwitcherOpen(false);
            void openNote(path);
          }}
          onCreate={(title) => {
            setSwitcherOpen(false);
            void createAndOpen("", title);
          }}
          onClose={() => setSwitcherOpen(false)}
        />
      )}

      {aboutOpen && <AboutDialog onClose={() => setAboutOpen(false)} />}

      {gitOpen && (
        <GitPanel
          encrypted={encrypted}
          onClose={() => setGitOpen(false)}
          onVaultChanged={() => {
            void api.getTree().then(setTree);
            bump();
          }}
        />
      )}

      {keyPrompt && (
        <ImportKeyDialog
          path={keyPrompt}
          onImported={() => void openVaultAt(keyPrompt)}
          onCancel={() => setKeyPrompt(null)}
        />
      )}

      {error && (
        <div className="toast" role="alert" onClick={() => setError(null)}>
          {error}
        </div>
      )}
      {notice && (
        <div className="toast" role="status" onClick={() => setNotice(null)}>
          {notice}
        </div>
      )}
    </div>
  );
}
