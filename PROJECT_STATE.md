# PROJECT_STATE — MD Notes

Оновлено: **17.10.2026**

## Поточний стан

- Статус розвитку: **ACTIVE**.
- Останній опублікований checkpoint: **v0.3.0** prerelease, 03.10.2026.
- Release commit: `42656f0075d0072e295bee33c74f059c3ae6e656`.
- v0.3.0 release published with Windows/macOS/Linux packages, START/source, legal files and checksums.
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

### Standalone Markdown / desktop integration

- PR #54 — відкриття одиночних `.md/.markdown` файлів без створення vault;
- читання, редагування і збереження назад у той самий файл;
- кнопка «Відкрити Markdown-файл…» доступна незалежно від installer/portable;
- Windows/Linux можуть добровільно зареєструвати саме запущену MD Notes як програму для Markdown;
- Windows після реєстрації відкриває штатне системне вікно вибору типової програми;
- Linux після явної команди користувача встановлює MD Notes типовим handler-ом через XDG;
- portable нічого не змінює в системі без натискання користувача;
- PR #54 merge: `c940dcc7aa2f8a4028b7ba4a8a15862070b7907b`; CI #270 SUCCESS + Mobile smoke #78 SUCCESS.

### Git foundation

- PR #39 — `gix` 0.88 HTTPS clone/open foundation з isolated config і safe URL validation;
- PR #41 — dirty worktree detection;
- PR #42 — local Git commit pipeline з explicit application identity і clean-index rebuild;
- PR #43 — safe public HTTPS fetch without credential-helper fallback;
- PR #44 — in-memory HTTPS authentication; credentials існують лише в callback під час network call.

PR #44 інтегрований у `main` merge commit `b142973ea4315987b28aa3a4e6563506c219c255`; PR head `fe5cdd66d416bc80e7e5dcc25e6d969ae917492f` пройшов CI #211 та Mobile smoke #29.

## Завершений v0.3 Git/security checkpoint

- PR #45 — native system credential storage у Tauri layer;
- Windows Credential Manager;
- macOS Keychain;
- iOS Protected Data;
- Android Keystore-backed storage;
- Linux Secret Service;
- save / has / clear API без IPC read-back секрету;
- authenticated fetch читає secret тільки всередині Rust;
- target-specific native dependency backends;
- Cargo-generated lockfile + legal notices;
- README UK/EN/FR/DE/ES/KO/JA синхронізовані.

PR #45 merge commit: `70d81622574b365334046520f534a9c3be8784af`.
Фінальний PR head `78e159c02691973cb9caa7468e40d3d1ea982cc5`: CI #235 SUCCESS + Android/iOS Mobile smoke SUCCESS.

Release PR #40 інтегрований у `main` commit `42656f0075d0072e295bee33c74f059c3ae6e656`; prerelease `v0.3.0` опублікований.

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
- macOS Finder/open-event + explicit handler action інтегровані у PR #56, merge `a6429a40f87827e710dbd2e6ebb7b9e6df194375`; desktop CI та Android green, final iOS rerun pending as evidence;
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
