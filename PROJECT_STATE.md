# PROJECT_STATE — MD Notes

Оновлено: **02.10.2026**

## Поточний стан

- Статус розвитку: **ACTIVE**.
- Останній опублікований checkpoint: **v0.2.1** prerelease, 01.10.2026.
- `main` випереджає опублікований release: функціональний scope v0.2 вже завершений у коді.
- Release commit v0.2.1: `26661d8c088df51ce162018d2a3b525502cf30e1`.
- Runtime startup hotfix: PR #21 / `de7b9d1b8fa7b9f5c7884b4082d977f4a1029b01`.
- Windows portable packaging: PR #20 / `715e29dfcf3bace2480e6b2bd824cfc80c91a668`.
- Schema/property form: PR #19 / `7441259a019eae1f7858898d8a29a9ac4d9ec7cd`.
- Vault presets/startup gate: PR #23 / `c078d075d089257c83b903eea44c317f391b765c`.
- Knowledge graph: PR #25 / `39040a461d0b29f09db382f7bca32483350c19ba`.
- Активний продукт: Tauri 2 + React (`app/`), ядро Rust (`crates/notes-core`).
- Платформи збірки: Windows / macOS / Linux. Android та iOS — етап v0.3.
- Наступні Windows release checkpoints: setup `.exe`, `.msi`, `MD-Notes-<version>-Windows-x64-portable.zip`.
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
- graph source — existing rebuildable SQLite index; Markdown лишається джерелом істини;
- global graph включає всі indexed notes, local graph — active note + one-hop incoming/outgoing neighbors;
- клік graph node відкриває нотатку; graph UI локалізовано 7 мовами.

## Перевірка

- PR #21 runtime-startup hotfix: CI #92 — **PASS**;
- PR #20 portable packaging: CI #95 — **PASS**;
- PR #19 schema/property form: CI #97 — **PASS**;
- PR #23 vault presets/startup gate: CI #114 — **PASS**;
- PR #25 knowledge graph: clean CI #132 / run `36981607612` — **PASS**;
- graph CI підтвердив notes-core Windows/macOS/Linux, frontend TypeScript/Vite build, Tauri clippy, cargo/npm license gates і PR title;
- Sigma.js 3.0.3 + Graphology 0.26.0 пройшли npm license gate; обидві MIT і внесені в `THIRD_PARTY_NOTICES.md`.

## Межа доказу / runtime blocker

- користувач на реальній Windows v0.2.1 підтвердив сильне зависання після запуску;
- startup тепер не читає `mdnotes.lastVault` без одноразового explicit-open marker після явної дії користувача;
- цей hotfix **ще не підтверджено повторним runtime-тестом на новій Windows-збірці**;
- startup/preset UI, schema/property form і graph UI також не проходили manual runtime validation у поточному середовищі;
- якщо зависання повториться після ручного вибору vault, наступний технічний напрям — оптимізація `note_paths()` / `get_tree()` і перенесення дорогого index sync з критичного open path;
- великі vault-и все ще індексуються синхронно при ручному відкритті;
- збірки не підписані;
- Android/iOS ще не збираються.

## Наступна дія

Функціональний roadmap v0.2 завершений. **Перший операційний пріоритет — реальна Windows runtime-перевірка актуального `main`**, особливо startup hotfix, створення/open vault і базова працездатність нових v0.2 UI slices. Повний release checkpoint створюється лише за командою власника **«зливай у main»**. Після runtime evidence — рішення про release checkpoint або початок roadmap v0.3.
