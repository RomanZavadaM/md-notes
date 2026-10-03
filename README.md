# MD Notes

**🇺🇦 Українська** · [🇬🇧 English](docs/readme/README.en.md) · [🇫🇷 Français](docs/readme/README.fr.md) · [🇩🇪 Deutsch](docs/readme/README.de.md) · [🇪🇸 Español](docs/readme/README.es.md) · [🇰🇷 한국어](docs/readme/README.ko.md) · [🇯🇵 日本語](docs/readme/README.ja.md)

[![CI](https://github.com/RomanZavadaM/md-notes/actions/workflows/ci.yml/badge.svg)](https://github.com/RomanZavadaM/md-notes/actions/workflows/ci.yml)

> **Останній опублікований checkpoint: [MD Notes v0.2.2](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.2.2)**
>
> **Статус розвитку: ACTIVE — roadmap stage v0.3 «мобільні платформи і синхронізація».**

## Про продукт

**MD Notes** — кросплатформний local-first застосунок для особистої бази знань у Markdown. Він працює зі звичайними `.md` файлами та вкладеннями без прихованого формату даних.

**Дані належать користувачу.** Нотатки можна відкрити будь-яким редактором, зберігати у власній файловій системі та версіонувати через Git. MD Notes не має власного сервера, аналітики чи телеметрії.

Платформи: **Windows, macOS, Linux**. Android та iOS активно розробляються у v0.3; CI вже підтверджує їх buildability, але runtime на фізичних пристроях ще не заявляється як підтверджений.

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

### v0.3 — активна розробка

У `main` уже інтегровано:

- provider-neutral `StorageProvider` / `VaultStorage`;
- mobile sandbox vault для Android/iOS;
- Android + iOS build smoke у CI;
- local-first sync decision foundation і persistent sync state;
- `gix`/gitoxide HTTPS Git foundation;
- визначення dirty worktree;
- локальний Git commit pipeline без залежності від user/system Git config;
- safe public HTTPS fetch;
- in-memory HTTPS authentication без запису token у Git config, vault або URL.

Поточний security checkpoint додає **системне сховище Git credentials** на рівні Tauri: Windows Credential Manager, macOS Keychain, iOS Protected Data, Android Keystore-backed storage і Linux Secret Service. Frontend може зберегти/перевірити/очистити credentials, але не отримує token назад; authenticated fetch читає секрет тільки всередині Rust перед network call.

Ще **не завершено**: Git pull/merge policy, push, повна інтеграція Git sync з conflict policy, WebDAV, runtime-перевірка на фізичних Android/iOS пристроях та optional Android SAF / iOS security-scoped external folders.

Повний план — [docs/roadmap.md](docs/roadmap.md).

## Встановлення

Завантажте пакет зі сторінки [релізів](https://github.com/RomanZavadaM/md-notes/releases):

- **Windows** — `.exe`, `.msi` або portable ZIP;
- **macOS** — `.dmg` / `.app.tar.gz`;
- **Linux** — `.AppImage`, `.deb` або `.rpm`.

Останній опублікований prerelease — **v0.2.2**. Збірки поки не підписані/не нотаризовані, тому ОС може показувати стандартні попередження безпеки.

## Для розробника

Потрібні Rust stable, Node.js 20+ і системні залежності Tauri.

```bash
cd app
npm ci
npm run tauri dev
```

Обов’язкові gates перед інтеграцією: Rust format/clippy/tests, frontend build, dependency-license checks, desktop CI та Android/iOS mobile smoke для змін, що зачіпають mobile/Tauri boundary.

Джерела істини: [START_HERE.md](START_HERE.md), [PROJECT_RULES.md](PROJECT_RULES.md), [PROJECT_STATE.md](PROJECT_STATE.md), [WORKLOG.md](WORKLOG.md) і GitHub Issue #8.

## Керівництво і документація

- [Керівництво користувача](docs/user-guide/USER_GUIDE.uk.md) · [усі мови](docs/user-guide/README.md)
- [Архітектура](docs/architecture.md) · [Модель даних](docs/data-model.md) · [Мова запитів](docs/query-language.md) · [ADR](docs/adr/)
- [Roadmap](docs/roadmap.md) · [Changelog](CHANGELOG.md) · [Release notes](docs/releases/)

## Дані, приватність і Git credentials

MD Notes працює local-first. Нотатки, вкладення та rebuildable index залишаються на пристрої або у вибраному користувачем сховищі. Секрети синхронізації не повинні зберігатися у vault, `.md` файлах, remote URL, Git config, логах чи `localStorage`; для них використовується системне credential storage.

Конфлікти синхронізації не повинні перезаписуватися мовчки. Робіть резервні копії важливих vault-ів.

## Авторські права

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.**

MD Notes — **proprietary software**. Публічний репозиторій не надає open-source ліцензії чи дозволу на копіювання, модифікацію, перепублікацію, продаж або створення похідних продуктів без письмового дозволу правовласника. Ваші нотатки належать вам.

Див. [LICENSE.md](LICENSE.md), [COPYRIGHT.md](COPYRIGHT.md), [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md), [LEGAL_AND_COPYRIGHT.md](docs/LEGAL_AND_COPYRIGHT.md).
