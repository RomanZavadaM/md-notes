# Roadmap

> Рішення власника від 30.10.2026: **перша повноцінна desktop-версія MD Notes функціонально завершена**. До stable 1.0.0 входить стабілізація, безпека, recovery, пакування та документація — без розширення продуктного scope.

## Desktop v1.0.0 — stable baseline

Платформи релізу: **Windows, macOS, Linux**.

Готово:

- [x] local-first Markdown vault без прихованого формату даних;
- [x] дерево файлів, tabs, standalone Markdown, multi-window/detached views;
- [x] CodeMirror editor, preview/split, autosave, Mermaid, KaTeX, attachments;
- [x] SQLite/FTS5 index, search, tags, backlinks, aliases, quick open;
- [x] schema-driven properties, templates, daily notes, PARA/Zettelkasten/Empty presets;
- [x] global/local knowledge graph, relation types A1–F6, reliability grades, relation tree, weakest-link path;
- [x] Git foundation: clone/open, status, commit, fetch/pull safety, push guard for encrypted vaults;
- [x] folder import / identity merge by exact code and duplicate candidates;
- [x] encrypted vault (age/X25519), random physical names, OS secret store, recovery recipient management;
- [x] encrypted image/PDF preview with plaintext only in memory; no plaintext temp-file external open;
- [x] portable ZIP backup + manifest/SHA-256 + restore only into a new vault;
- [x] automated encrypted recovery drill: backup → restore → recovery key → note + attachment read;
- [x] 7 interface/documentation languages: UK / EN / FR / DE / ES / KO / JA;
- [x] CI on Windows/macOS/Linux, Android/iOS build smoke, i18n parity gate, production npm security gate;
- [x] Windows NSIS/MSI offline-capable for WebView2 and bundled VC runtime;
- [x] Windows portable ZIP retained as lightweight package using the system WebView2 Runtime.

Known 1.0 boundaries:

- builds are unsigned / not notarized until real signing credentials exist;
- encrypted vault cache index `.mdnotes/cache` is outside the encrypted trust boundary;
- Android/iOS physical-device runtime is not claimed as validated;
- WebDAV and full SyncManifest conflict policy are not part of desktop 1.0.

## Post-1.0 — stabilization driven by real use

Near-term candidates, each as a separate slice:

- batch move, then batch rename;
- Git state ↔ `SyncManifest` / ADR-0005 conflict policy;
- WebDAV;
- stronger encrypted-cache mode if justified by the threat model;
- signing/notarization and auto-update when credentials/release channel are available;
- targeted `App.tsx` decomposition when future work touches the relevant domains.

## Mobile — post desktop 1.0

Current status: Android/iOS continue to pass build smoke in CI.

Before claiming mobile support:

- physical Android/iOS runtime validation;
- optional Android SAF / iOS security-scoped external folders;
- mobile-specific table/card interaction decisions;
- store packaging/publication.

## Longer-term ideas

Not committed to a version until owner approval:

- tasks/kanban/calendar/timeline;
- SFTP/S3-compatible storage;
- Google Drive / OneDrive / Dropbox;
- JSON Canvas / mind maps / attachment gallery;
- Git-backed note history / diff / restore;
- SMB.

Historical design rationale remains in the ADRs and Git history.
