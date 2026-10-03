# PROJECT_STATE — MD Notes

Оновлено: **03.10.2026**

## Поточний стан

- Статус розвитку: **ACTIVE**.
- Останній опублікований checkpoint: **v0.2.2** prerelease, 02.10.2026.
- Release commit: `b422b497b4e3980c665f7ae7017f34e73788218d`.
- Release workflow `36990461869` — **SUCCESS**.
- 03.10.2026 власник підтвердив v0.2.2 реальним Windows runtime-тестом; startup-freeze blocker закритий.
- Roadmap v0.2 завершений; активний roadmap stage — **v0.3**.
- Активний продукт: Tauri 2 + React (`app/`) + Rust core (`crates/notes-core`).
- Desktop: Windows / macOS / Linux. Android/iOS — active development target.
- UI/README languages: UK / EN / FR / DE / ES / KO / JA.
- Ліцензійна модель: proprietary / All Rights Reserved, Roman Zavada / Роман Завада.

## Інтегрований v0.3 baseline

### Storage / mobile

- PR #30 — provider-neutral `StorageProvider` + `LocalFsProvider`;
- PR #31 — provider-backed `VaultStorage`;
- PR #32 — mobile sandbox vault + mobile startup flow;
- PR #35 — Android aarch64 + iOS simulator build smoke у CI.

### Sync foundation

- PR #33 — `SyncManifest`, snapshots, upload/download/delete planning, visible conflict-copy policy;
- PR #37 — persistent atomic `.mdnotes/cache/sync-state.json`.

### Git foundation

- PR #39 — `gix` 0.88 HTTPS clone/open foundation з isolated config і safe URL validation;
- PR #41 — dirty worktree detection;
- PR #42 — local Git commit pipeline з explicit application identity і clean-index rebuild;
- PR #43 — safe public HTTPS fetch without credential-helper fallback;
- PR #44 — in-memory HTTPS authentication; credentials існують лише в callback під час network call.

PR #44 інтегрований у `main` merge commit `b142973ea4315987b28aa3a4e6563506c219c255`; PR head `fe5cdd66d416bc80e7e5dcc25e6d969ae917492f` пройшов CI #211 та Mobile smoke #29.

## Активний checkpoint — PR #45

PR #45 додає system credential storage у Tauri layer:

- Windows Credential Manager;
- macOS Keychain;
- iOS Protected Data;
- Android Keystore-backed storage;
- Linux Secret Service;
- target-specific native backend dependencies замість broad CLI wrapper;
- `save_git_credentials`, `has_git_credentials`, `clear_git_credentials`;
- token не експонується командою читання у frontend;
- `git_fetch_with_stored_credentials` завантажує token лише всередині Rust;
- vault mutex не утримується під час credential/network роботи;
- `Cargo.lock` оновлений Cargo-generated dependency graph;
- `THIRD_PARTY_NOTICES.md` оновлений;
- README UK/EN/FR/DE/ES/KO/JA синхронізовані з active v0.3 state.

Не вважати PR #45 інтегрованим до green final CI + Mobile smoke + merge.

## Функціонально завершено у v0.2

- local-folder vault, tree, create/rename/trash;
- CodeMirror 6, preview/split, autosave, atomic writes;
- SQLite/FTS5 search/tags/index;
- wiki-links, aliases, backlinks, link rewrite;
- file watcher на desktop;
- Mermaid 11, KaTeX, images, attachments;
- templates + daily notes;
- 7 мов UI, About, light/dark/system themes;
- restrictive Tauri CSP;
- open schema v1 + schema-driven properties;
- Empty / PARA / Zettelkasten presets;
- startup gate без silent auto-reopen;
- global/local knowledge graph.

## Межа доказу / відомі обмеження

- Windows v0.2.2 runtime — підтверджено власником;
- Android/iOS buildability — підтверджується CI; physical-device runtime ще не підтверджений;
- Git pull/merge policy ще не завершена;
- Git push ще не реалізований;
- Git sync ще не зв’язаний повністю з `SyncManifest` / ADR-0005 conflict resolution;
- WebDAV ще не реалізований;
- Android SAF / iOS security-scoped bookmarks ще не реалізовані;
- large vault open/indexing може лишатися синхронним;
- релізні builds не підписані.

## Наступний напрям після PR #45

1. Git pull/merge policy без silent overwrite.
2. Git push.
3. Integration Git state ↔ `SyncManifest` / ADR-0005.
4. WebDAV.
5. Physical-device Android/iOS runtime validation.
6. Optional external-folder adapters: Android SAF / iOS security-scoped bookmarks.

Перед новими dependency обов’язково проходити license gates та оновлювати `THIRD_PARTY_NOTICES.md`.
