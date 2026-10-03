# PROJECT_STATE — MD Notes

Оновлено: **03.10.2026**

## Поточний стан

- Статус розвитку: **ACTIVE**.
- Останній опублікований checkpoint: **v0.2.2** prerelease, 02.10.2026.
- Release commit: `b422b497b4e3980c665f7ae7017f34e73788218d`.
- Release workflow #30 / run `36990461869` — **SUCCESS**.
- 03.10.2026 власник повторно протестував v0.2.2 на реальній Windows і підтвердив: **працює нормально**.
- Startup-freeze blocker v0.2.1 для v0.2.2 закритий ручним runtime evidence.
- Функціональний roadmap v0.2 завершений; власник погодив перехід до roadmap stage **v0.3**.
- Активний продукт: Tauri 2 + React (`app/`), ядро Rust (`crates/notes-core`).
- Desktop платформи: Windows / macOS / Linux. Android та iOS — активний v0.3 development target.
- UI-мови: UK / EN / FR / DE / ES / KO / JA.
- Ліцензійна модель: proprietary / All Rights Reserved (Roman Zavada). Публічний репозиторій не надає open-source ліцензії.

## Інтегровано у v0.3

### PR #30 — StorageProvider foundation

Merge: `4545c938fcb70c80caa9e7595b4d89e8eead27aa`.
CI #147 — **PASS**.

- provider-neutral `StorageProvider`;
- `LocalFsProvider`;
- `list`, `read`, `write`, `remove`, `metadata`, `changes_since`;
- atomic writes;
- path escape rejection;
- unit-тести.

### PR #31 — provider-backed Vault I/O

Merge: `230ee8e4373fe909c178c44865c5b5cd9d7bcd3e`.
CI #153 — **PASS**.

- `VaultStorage` поверх `StorageProvider`;
- provider-backed tree traversal, note paths, config, read/write notes, unique paths;
- існуючий `Vault` делегує provider-neutral I/O через `VaultStorage`;
- local cache/rename/trash поки лишаються локальними операціями.

### PR #32 — mobile sandbox bootstrap

Merge: `588219e2310e94f2fbd04a3ba97288b8ba94813f`.
CI #161 — **PASS**.

- runtime platform detection;
- app-data `vault` як default mobile sandbox за ADR-0006;
- safe open-or-create local mobile vault;
- Tauri command `open_mobile_sandbox_vault`;
- typed frontend API;
- mobile startup gate без desktop folder picker;
- startup UI локалізовано UK/EN/FR/DE/ES/KO/JA.

### PR #33 — local-first sync decision foundation

Merge: `e91734df8f37d2be1a776fcad7883bbd30160b01`.
CI #162 — **PASS**.

- `SyncManifest`, `SyncSnapshot`, `SyncDecision`;
- planning upload/download/delete/no-change;
- concurrent deletion vs modification: modification wins;
- divergent edits => merge-or-conflict decision;
- visible conflict-copy filename;
- unit-тести за політикою ADR-0005;
- без нових sync dependencies на цьому foundation slice.

## Поточна перевірка mobile

Draft PR #35 — `ci: validate Android and iOS mobile builds`.

Мета:
- Android: `tauri android init --ci` + aarch64 debug build;
- iOS: `tauri ios init --ci` + arm64 simulator debug build;
- не видавати build evidence за runtime validation на реальному пристрої.

## Функціонально завершено у v0.2

- локальна папка як vault, дерево файлів, створення, перейменування, `.mdnotes/trash/`;
- CodeMirror 6, preview/split, autosave, atomic writes;
- SQLite index, FTS5, теги, quick open;
- wiki-links, aliases, property links, backlinks, link rewrite при rename/move;
- desktop file watcher;
- Mermaid 11, KaTeX, vault images, relative `.md` links;
- templates і daily notes;
- 7 мов UI, About, light/dark/system themes, adaptive layout;
- restrictive Tauri CSP;
- attachments: `attachments/YYYY/MM/`, image/PDF preview, usage/orphans, Unicode paths;
- відкритий `.mdnotes/schema.json` v1 і schema-driven property form;
- safe Empty / PARA / Zettelkasten vault presets;
- startup gate без silent auto-reopen останнього vault;
- global/local knowledge graph із Sigma.js 3.0.3 + Graphology 0.26.0.

## Опубліковано у v0.2.2

### Windows x64
- `MD.Notes_0.2.2_x64-setup.exe`
- `MD.Notes_0.2.2_x64_en-US.msi`
- `MD-Notes-0.2.2-Windows-x64-portable.zip`

### macOS universal
- `MD.Notes_0.2.2_universal.dmg`
- `MD.Notes_universal.app.tar.gz`

### Linux x86_64
- `MD.Notes_0.2.2_amd64.AppImage`
- `MD.Notes_0.2.2_amd64.deb`
- `MD.Notes-0.2.2-1.x86_64.rpm`

### Source / legal / verification
- `MD-Notes-0.2.2-START.zip`
- `LICENSE.md`
- `COPYRIGHT.md`
- `THIRD_PARTY_NOTICES.md`
- `SHA256SUMS.txt`

## Межа доказу / відомі обмеження

- v0.2.2 Windows runtime вручну підтверджений власником;
- Android/iOS runtime на реальних пристроях ще не підтверджений;
- mobile build smoke PR #35 є лише build evidence;
- Git/WebDAV sync ще не реалізований, лише provider-neutral sync decision foundation;
- optional Android SAF / iOS security-scoped bookmark adapters ще не реалізовані;
- local cache/rename/trash ще не узагальнені під external mobile providers;
- великі vault-и все ще можуть індексуватися синхронно при відкритті;
- збірки не підписані.

## Наступна дія

1. Завершити mobile build smoke PR #35 і зафіксувати фактичний результат Android/iOS build.
2. Далі реалізувати Git sync через `gix`/gitoxide поверх `SyncManifest` / `plan_sync`.
3. Після Git — WebDAV через OpenDAL.
4. Після стабільного sandbox + sync — optional external-folder adapters: Android SAF і iOS security-scoped bookmarks.
5. Перед додаванням нових dependency перевіряти licenses і оновлювати `THIRD_PARTY_NOTICES.md`.
