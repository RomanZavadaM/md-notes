# PROJECT_STATE — MD Notes

Оновлено: **02.10.2026**

## Поточний стан

- Статус розвитку: **ACTIVE**.
- Останній опублікований checkpoint: **v0.2.2** prerelease, 02.10.2026.
- Release commit: `b422b497b4e3980c665f7ae7017f34e73788218d`.
- Release workflow #30 / run `36990461869` — **SUCCESS**.
- Main CI run `36990461817` — **PASS**.
- Функціональний roadmap v0.2 завершений.
- Runtime startup hotfix: PR #21 / `de7b9d1b8fa7b9f5c7884b4082d977f4a1029b01`.
- Windows portable packaging: PR #20 / `715e29dfcf3bace2480e6b2bd824cfc80c91a668`.
- Schema/property form: PR #19 / `7441259a019eae1f7858898d8a29a9ac4d9ec7cd`.
- Vault presets/startup gate: PR #23 / `c078d075d089257c83b903eea44c317f391b765c`.
- Knowledge graph: PR #25 / `39040a461d0b29f09db382f7bca32483350c19ba`.
- Version policy: PR #27 / `8ca052c6aa69c31f5af8afce186d606b97ffd22a`; feature-checkpoint-и pre-1.0 лишаються на patch line всередині поточного roadmap stage.
- Активний продукт: Tauri 2 + React (`app/`), ядро Rust (`crates/notes-core`).
- Платформи збірки: Windows / macOS / Linux. Android та iOS — roadmap v0.3.
- Windows release artifacts: setup `.exe`, `.msi`, `MD-Notes-<version>-Windows-x64-portable.zip`.
- UI-мови: UK / EN / FR / DE / ES / KO / JA.
- Ліцензійна модель: proprietary / All Rights Reserved (Roman Zavada). Публічний репозиторій не надає open-source ліцензії.

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
- global/local knowledge graph із Sigma.js 3.0.3 + Graphology 0.26.0;
- graph source — rebuildable SQLite index; Markdown лишається джерелом істини;
- global graph включає всі indexed notes, local graph — active note + one-hop incoming/outgoing neighbors;
- click graph node відкриває нотатку; graph UI локалізовано 7 мовами.

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

## Перевірка

- PR #21 runtime-startup hotfix: CI #92 — **PASS**;
- PR #20 portable packaging: CI #95 — **PASS**;
- PR #19 schema/property form: CI #97 — **PASS**;
- PR #23 vault presets/startup gate: CI #114 — **PASS**;
- PR #25 knowledge graph: clean CI #132 — **PASS**;
- PR #27 version policy: CI #138 — **PASS**;
- release commit main CI `36990461817` — **PASS**;
- release workflow #30 / `36990461869` — **SUCCESS**;
- Windows/macOS/Linux build jobs, dependency licenses, START/legal/checksums — PASS/SUCCESS.

## Межа доказу / runtime blocker

- користувач на реальній Windows v0.2.1 підтвердив сильне зависання після запуску;
- v0.2.2 містить startup hotfix: silent auto-reopen останнього vault заблоковано, startup flow вимагає явної дії;
- **ручний runtime test v0.2.2 ще не підтверджено**; green CI/build не вважається runtime validation;
- startup/preset UI, schema/property form і graph UI також не проходили manual runtime validation у поточному середовищі;
- якщо зависання повториться після ручного відкриття vault, наступний технічний напрям — оптимізація `note_paths()` / `get_tree()` і перенесення дорогого index sync з критичного open path;
- великі vault-и все ще індексуються синхронно при ручному відкритті;
- збірки не підписані.

## Наступна дія

**Перший операційний пріоритет — реальна Windows runtime-перевірка v0.2.2**: startup, «Про програму», ручне відкриття vault, presets, schema/property form, local/global graph. Якщо freeze відтворюється — спочатку закрити performance blocker. Якщо базовий runtime стабільний — переходити до roadmap v0.3 (Android/iOS + Git/WebDAV sync) за рішенням власника.
