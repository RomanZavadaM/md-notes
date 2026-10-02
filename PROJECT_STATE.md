# PROJECT_STATE — MD Notes

Оновлено: **02.10.2026**

## Поточний стан

- Статус розвитку: **ACTIVE**.
- Останній опублікований checkpoint: **v0.2.1** prerelease, 01.10.2026.
- `main` уже містить сумісні зміни після v0.2.1 і тому випереджає опублікований release.
- Release commit v0.2.1: `26661d8c088df51ce162018d2a3b525502cf30e1`.
- Runtime startup hotfix: PR #21 / `de7b9d1b8fa7b9f5c7884b4082d977f4a1029b01`.
- Windows portable packaging: PR #20 / `715e29dfcf3bace2480e6b2bd824cfc80c91a668`.
- Schema/property form: PR #19 / `7441259a019eae1f7858898d8a29a9ac4d9ec7cd`.
- Vault presets/startup gate: PR #23 / `c078d075d089257c83b903eea44c317f391b765c`.
- Активний продукт: Tauri 2 + React (`app/`), ядро Rust (`crates/notes-core`).
- Платформи збірки: Windows / macOS / Linux. Android та iOS — етап v0.3.
- Наступні Windows release checkpoints: setup `.exe`, `.msi`, окремий `MD-Notes-<version>-Windows-x64-portable.zip`.
- UI-мови: UK / EN / FR / DE / ES / KO / JA.
- Ліцензійна модель: proprietary / All Rights Reserved (Roman Zavada). Публічний репозиторій не надає open-source ліцензії.

Детальна історія — у `CHANGELOG.md`, `docs/releases/`, merged PR і GitHub Issue #8.

## Інтегровано в лінію v0.2

- локальна папка як сховище, дерево файлів, створення, перейменування, кошик `.mdnotes/trash/`;
- редактор CodeMirror 6, перегляд, режим «поруч», автозбереження, атомарний запис;
- індекс SQLite (`.mdnotes/cache/index.db`), повнотекстовий пошук, теги, швидкий перехід;
- `[[вікі-посилання]]`, aliases, property links, backlinks з контекстом;
- оновлення посилань при перейменуванні та переміщенні;
- file watcher для зовнішніх змін на desktop;
- Mermaid 11, KaTeX, vault images, відносні `.md` links;
- шаблони та щоденні нотатки;
- 7 мов UI, About, light/dark/system themes, adaptive layout;
- restrictive Tauri CSP;
- вкладення: `attachments/YYYY/MM/`, preview, usage/orphans, Unicode paths;
- типи нотаток і відкритий `.mdnotes/schema.json` v1;
- schema-driven форма властивостей у редакторі;
- збереження невідомих YAML-полів, підтримка невідомих note types, safe refusal при invalid YAML;
- property form працює з поточним editor content, тому незбережене тіло нотатки не перезаписується окремим записом;
- safe vault creation presets: Empty, PARA, Zettelkasten;
- startup gate: «відкрити останнє / відкрити існуюче / створити нове», без неявного auto-reopen;
- preset creation дозволено лише в порожній папці; existing files не змінюються;
- startup/preset UI локалізовано сімома мовами і має scroll-friendly layout для малих екранів.

## Перевірка

- PR #21 runtime-startup hotfix: CI #92 — **PASS**;
- PR #20 portable packaging: CI #95 — **PASS**;
- PR #19 schema/property form: clean CI #97 — **PASS**;
- PR #23 vault presets/startup gate: clean CI #114 — **PASS**;
- notes-core перевірено на Windows/macOS/Linux; frontend build, Tauri clippy, cargo-deny та npm license gate — PASS;
- runtime GUI-перевірка schema/property form і startup/preset flow у поточному середовищі не виконувалась.

## Runtime blocker / межа доказу

- користувач підтвердив сильне зависання звичайної Windows-збірки v0.2.1 після запуску;
- startup тепер не читає `mdnotes.lastVault` без одноразового explicit-open marker, створеного лише після явної дії користувача;
- hotfix ще потребує повторної реальної Windows runtime-перевірки на новій збірці;
- якщо зависання повториться після ручного вибору vault, наступний технічний напрям — оптимізація `note_paths()` / `get_tree()` і винесення дорогого index sync із критичного open path;
- збірки не підписані;
- великі сховища все ще індексуються синхронно при ручному відкритті;
- Android/iOS ще не збираються.

## Наступний великий крок

Останній незавершений функціональний пункт v0.2: **граф знань — глобальний і локальний**. Після нього — завершення етапу v0.2 за рішенням власника і перехід до v0.3 (Android, iOS, Git/WebDAV sync). Поточна операційна дія — у `WORKLOG.md`.
