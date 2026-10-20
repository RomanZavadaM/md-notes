# MD Notes

**🇺🇦 Українська** · [🇬🇧 English](docs/readme/README.en.md) · [🇫🇷 Français](docs/readme/README.fr.md) · [🇩🇪 Deutsch](docs/readme/README.de.md) · [🇪🇸 Español](docs/readme/README.es.md) · [🇰🇷 한국어](docs/readme/README.ko.md) · [🇯🇵 日本語](docs/readme/README.ja.md)

[![CI](https://github.com/RomanZavadaM/md-notes/actions/workflows/ci.yml/badge.svg)](https://github.com/RomanZavadaM/md-notes/actions/workflows/ci.yml)

<!-- x-release-please-start-version -->
> **Release checkpoint: MD Notes v0.3.12**
>
> **Статус розвитку: ACTIVE — roadmap stage v0.3 «мобільні платформи і синхронізація».**

## Завантажити MD Notes v0.3.12

**Швидке встановлення:**

- **Windows 64-bit:** [завантажити інсталятор .exe](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.12/MD.Notes_0.3.12_x64-setup.exe) · [MSI](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.12/MD.Notes_0.3.12_x64_en-US.msi) · [portable ZIP](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.12/MD-Notes-0.3.12-Windows-x64-portable.zip)
- **macOS Universal:** [завантажити DMG](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.12/MD.Notes_0.3.12_universal.dmg)
- **Linux x86_64:** [AppImage](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.12/MD.Notes_0.3.12_amd64.AppImage) · [DEB](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.12/MD.Notes_0.3.12_amd64.deb) · [RPM](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.12/MD.Notes-0.3.12-1.x86_64.rpm)

[**Усі файли релізу v0.3.12 →**](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.3.12)
<!-- x-release-please-end -->

> Збірки поки не підписані/не нотаризовані, тому Windows, macOS або Linux можуть показувати стандартне попередження безпеки.

## Про продукт

**MD Notes** — кросплатформний local-first застосунок для особистої бази знань у Markdown. Він працює зі звичайними `.md/.markdown` файлами, вкладеннями та відкритими службовими JSON-файлами без прихованого формату даних.

**Дані належать користувачу.** Нотатки можна відкрити будь-яким редактором, зберігати у власній файловій системі та версіонувати через Git. MD Notes не має власного сервера, аналітики чи телеметрії.

Платформи: **Windows, macOS, Linux**. Android та iOS активно розробляються у v0.3; CI підтверджує їх buildability, але runtime на фізичних пристроях ще не заявляється як підтверджений.

## Що вже є

### v0.2 — завершений функціональний baseline

- локальна папка як vault, дерево файлів, створення, перейменування та `.mdnotes/trash/`;
- CodeMirror 6, preview/split, autosave й атомарні записи;
- SQLite/FTS5 індекс, пошук, теги, quick open;
- `[[вікі-посилання]]`, aliases, backlinks і автоматичне оновлення посилань при rename/move;
- Mermaid 11, KaTeX, зображення, вкладення, шаблони та daily notes;
- schema-driven properties через відкритий `.mdnotes/schema.json`;
- Empty / PARA / Zettelkasten presets;
- global/local knowledge graph;
- UI сімома мовами, світла/темна/системна теми.

### v0.3 — інтегрований стан до checkpoint v0.3.4

- provider-neutral `StorageProvider` / `VaultStorage`;
- mobile sandbox vault для Android/iOS;
- Android + iOS build smoke у CI;
- local-first sync decision foundation і persistent sync state;
- `gix`/gitoxide HTTPS Git foundation;
- dirty-worktree detection і локальний Git commit pipeline;
- safe public HTTPS fetch;
- in-memory HTTPS authentication без запису token у Git config, vault або URL;
- системне сховище Git credentials: Windows Credential Manager, macOS Keychain, iOS Protected Data, Android Keystore-backed storage і Linux Secret Service;
- standalone `.md/.markdown`: відкрити без vault, редагувати й зберегти назад;
- Windows/Linux/macOS desktop integration для Markdown, але системна асоціація виконується лише після явної команди користувача;
- Git pull planner: `UpToDate / FastForward / LocalAhead / Diverged / Unrelated`;
- pull safety preflight: dirty worktree блокує apply;
- safe fast-forward apply лише для clean `FastForward`, без blind `hard reset`, merge commit чи silent overwrite;
- опціональне зашифроване сховище (age): нотатки лежать на диску як `.age`, приватний ключ зберігається лише в системному сховищі секретів пристрою (Windows Credential Manager, macOS Keychain, Linux Secret Service), у діалогах — створення з показом ключа один раз, імпорт ключа на іншому пристрої, необов’язковий ключ відновлення.

Ще **не завершено**: Git push, повна інтеграція Git state з `SyncManifest` / ADR-0005 conflict policy, WebDAV, runtime-перевірка на фізичних Android/iOS пристроях та optional Android SAF / iOS security-scoped external folders.

**Зашифроване сховище** — опція, а не вимога: звичайні vault працюють без змін, а збірка без feature `encrypted-vault` не містить коду шифрування. Поки що в зашифрованому vault не відображаються зображення й вкладення, немає інтерфейсу додавання ключа відновлення до вже створеного vault, а кеш-індекс `.mdnotes/cache` лишається незашифрованим; Git push із такого vault буде додано із запобіжником, що не відправляє відкритий текст.

Повний план — [docs/roadmap.md](docs/roadmap.md).

## Встановлення

Завантажте пакет зі сторінки [релізів](https://github.com/RomanZavadaM/md-notes/releases):

- **Windows** — `.exe`, `.msi` або portable ZIP;
- **macOS** — `.dmg` / `.app.tar.gz`;
- **Linux** — `.AppImage`, `.deb` або `.rpm`.

Checkpoint **v0.3.4** включає standalone Markdown і безпечний Git fast-forward pipeline. Release builds поки не підписані/не нотаризовані, якщо сторінка конкретного релізу прямо не вказує інше.

## Для розробника

Потрібні Rust stable, Node.js 20+ і системні залежності Tauri.

```bash
cd app
npm ci
npm run tauri dev
```

Обов’язкові gates: Rust fmt/clippy/tests, frontend build, dependency-license checks, desktop CI та Android/iOS mobile smoke для релевантних змін.

Джерела істини: [START_HERE.md](START_HERE.md), [PROJECT_RULES.md](PROJECT_RULES.md), [PROJECT_STATE.md](PROJECT_STATE.md), [WORKLOG.md](WORKLOG.md) і GitHub Issue #8.

## Керівництво і документація

- [Керівництво користувача](docs/user-guide/USER_GUIDE.uk.md) · [усі мови](docs/user-guide/README.md)
- [Архітектура](docs/architecture.md) · [Модель даних](docs/data-model.md) · [Мова запитів](docs/query-language.md) · [ADR](docs/adr/)
- [Roadmap](docs/roadmap.md) · [Changelog](CHANGELOG.md) · [Release notes](docs/releases/)

## Дані, приватність і Git credentials

MD Notes працює local-first. Нотатки, вкладення та rebuildable index залишаються на пристрої або у вибраному користувачем сховищі. Секрети синхронізації не зберігаються у vault, Markdown-файлах, remote URL, Git config, логах чи `localStorage`; для них використовується системне credential storage.

Конфлікти синхронізації не повинні перезаписуватися мовчки. Автоматичний Git apply дозволений тільки для чистого fast-forward стану.

## Авторські права

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.**

MD Notes — **proprietary software**. Публічний репозиторій не надає open-source ліцензії. Ваші нотатки належать вам.

Див. [LICENSE.md](LICENSE.md), [COPYRIGHT.md](COPYRIGHT.md), [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md), [LEGAL_AND_COPYRIGHT.md](docs/LEGAL_AND_COPYRIGHT.md).
