# MD Notes

**🇺🇦 Українська** · [🇬🇧 English](docs/readme/README.en.md) · [🇫🇷 Français](docs/readme/README.fr.md) · [🇩🇪 Deutsch](docs/readme/README.de.md) · [🇪🇸 Español](docs/readme/README.es.md) · [🇰🇷 한국어](docs/readme/README.ko.md) · [🇯🇵 日本語](docs/readme/README.ja.md)

[![CI](https://github.com/RomanZavadaM/md-notes/actions/workflows/ci.yml/badge.svg)](https://github.com/RomanZavadaM/md-notes/actions/workflows/ci.yml)

> **Поточний checkpoint: [MD Notes v0.2.0](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.2.0)**
>
> **Статус розвитку: ACTIVE — наступний етап v0.3 «мобільні платформи і синхронізація».**

## Про продукт

**MD Notes** — кросплатформний local-first застосунок для особистої бази знань у форматі Markdown: перегляд, редагування, структурування й візуалізація звичайних `.md` файлів разом із вкладеннями.

**Дані належать користувачу.** Нотатки — це відкриті текстові файли. Їх можна відкрити будь-яким редактором, переглянути на GitHub і версіонувати через Git. Застосунок не створює прихованих форматів.

Платформи: **Windows, macOS, Linux**; Android та iOS — етап v0.3.

## Що входить у v0.2.0

- локальна папка як сховище; дерево файлів, створення, перейменування, кошик;
- редактор CodeMirror 6, перегляд, режим «поруч», автозбереження, атомарний запис;
- `[[вікі-посилання]]`, `aliases`, панель зворотних посилань із контекстом;
- **оновлення посилань при перейменуванні й переміщенні** нотаток і папок;
- повнотекстовий пошук, теги з кількістю нотаток, швидкий перехід `Ctrl+O`;
- Mermaid, KaTeX, зображення зі сховища;
- шаблони нотаток і щоденні нотатки;
- стеження за змінами файлів поза застосунком (настільні ОС);
- інтерфейс сімома мовами, вікно «Про програму»;
- світла, темна і системна теми; приклад бази знань `sample-vault/`.

Наступний етап — v0.3: Android та iOS, синхронізація через Git і WebDAV, таблиці й мова запитів. Повний план — [docs/roadmap.md](docs/roadmap.md).

## Встановлення

Завантажте пакет для своєї ОС зі сторінки [релізу](https://github.com/RomanZavadaM/md-notes/releases):

- **Windows** — `MD.Notes_<версія>_x64-setup.exe` або `.msi`. Збірка не підписана, тому Windows може показати SmartScreen.
- **macOS** — `.dmg` / `.app.tar.gz` (universal). Збірка не нотаризована; можливо, знадобиться **System Settings → Privacy & Security → Open Anyway**.
- **Linux** — `.AppImage`, `.deb` або `.rpm`.

Після запуску натисніть **«Відкрити папку»** і виберіть папку з нотатками або `sample-vault/` з цього репозиторію.

## Для розробника

Потрібні [Rust](https://rustup.rs/) (stable), [Node.js](https://nodejs.org/) 20+ і [системні залежності Tauri](https://v2.tauri.app/start/prerequisites/).

```bash
cd app
npm ci
npm run tauri dev
```

Перевірки перед PR: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p notes-core`, `npm run build` у `app/`. Правила розробки — [PROJECT_RULES.md](PROJECT_RULES.md), вхід для нової сесії — [START_HERE.md](START_HERE.md).

## Керівництво і документація

- [Керівництво користувача](docs/user-guide/USER_GUIDE.uk.md) · [усі мови](docs/user-guide/README.md)
- [Архітектура](docs/architecture.md) · [Модель даних](docs/data-model.md) · [Мова запитів](docs/query-language.md) · [ADR](docs/adr/)
- [Roadmap](docs/roadmap.md) · [Changelog](CHANGELOG.md) · [Release notes](docs/releases/)
- [START_HERE](START_HERE.md) · [PROJECT_RULES](PROJECT_RULES.md) · [PROJECT_STATE](PROJECT_STATE.md) · [WORKLOG](WORKLOG.md)

## Дані та приватність

MD Notes працює local-first: нотатки, вкладення та індекс залишаються на вашому пристрої або у вибраному вами сховищі. Застосунок не має серверів, аналітики чи телеметрії. Конфлікти синхронізації ніколи не перезаписуються мовчки. Робіть резервні копії своїх сховищ.

## Авторські права

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.**

MD Notes — **proprietary software**. Публічний репозиторій не надає open-source ліцензії чи дозволу на копіювання, модифікацію, перепублікацію, продаж або створення похідних продуктів без письмового дозволу правовласника. Ваші нотатки належать вам.

Див. [LICENSE.md](LICENSE.md), [COPYRIGHT.md](COPYRIGHT.md), [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md), [LEGAL_AND_COPYRIGHT.md](docs/LEGAL_AND_COPYRIGHT.md).
