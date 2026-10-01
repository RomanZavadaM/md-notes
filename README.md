# MD Notes

[![CI](https://github.com/RomanZavadaM/md-notes/actions/workflows/ci.yml/badge.svg)](https://github.com/RomanZavadaM/md-notes/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Кросплатформний застосунок для особистої бази знань у форматі Markdown:
перегляд, редагування, структурування й візуалізація звичайних `.md` файлів
разом із вкладеннями.

**Дані належать користувачу.** Нотатки — це відкриті текстові файли. Їх можна
відкрити будь-яким редактором, переглянути на GitHub і версіонувати через Git.
Застосунок не створює прихованих форматів.

> Статус: рання розробка (v0.1 → v0.2). Працюють локальні сховища, редактор і
> перегляд, індекс із пошуком, зворотні посилання, теги, оновлення посилань при
> перейменуванні, Mermaid і KaTeX.
> Повний план — у [docs/roadmap.md](docs/roadmap.md).

## Можливості

| Є зараз | Заплановано |
|---|---|
| Локальна папка як сховище, стеження за змінами | Мережеві й хмарні сховища, Git |
| Дерево файлів: створення, перейменування, кошик | Вкладення, «де використовується» |
| Редактор CodeMirror 6, перегляд, режим «поруч» | Граф знань |
| `[[Вікі-посилання]]`, зворотні посилання, `aliases` | Типи нотаток, шаблони, форми |
| Оновлення посилань при перейменуванні | Таблиці й мова запитів |
| Повнотекстовий пошук, теги, швидкий перехід `Ctrl+O` | Канбан, календар, полотно |
| Mermaid, KaTeX, зображення зі сховища | Android та iOS |
| Світла, темна і системна теми | |

## Платформи

Windows, macOS, Linux, а пізніше Android та iOS. Усе з однієї кодової бази
(Tauri 2).

## Швидкий старт для розробника

Потрібні:

- [Rust](https://rustup.rs/) (stable);
- [Node.js](https://nodejs.org/) 20 або новіший;
- системні залежності Tauri для вашої ОС:
  [tauri.app/start/prerequisites](https://v2.tauri.app/start/prerequisites/).

```bash
git clone https://github.com/RomanZavadaM/md-notes.git
cd md-notes/app
npm install
npm run tauri dev
```

Після запуску натисніть **«Відкрити папку»** і виберіть `sample-vault/`, щоб
побачити приклад бази знань.

Корисні команди:

```bash
cargo test -p notes-core          # тести ядра
cargo clippy --workspace          # лінтер Rust
cd app && npm run build           # перевірка типів і збірка інтерфейсу
cd app && npm run tauri build     # інсталятор для поточної ОС
```

## Структура репозиторію

```
md-notes/
├── .github/workflows/    CI і релізи
├── crates/notes-core/    ядро на Rust: сховища, нотатки, розбір Markdown
├── app/                  інтерфейс (React + TypeScript) і Tauri-застосунок
│   └── src-tauri/        команди Tauri, конфігурація, іконки
├── docs/                 архітектура, модель даних, roadmap, ADR
├── sample-vault/         приклад бази знань
├── CHANGELOG.md
└── README.md
```

## Документація

- [Архітектура](docs/architecture.md)
- [Модель даних](docs/data-model.md)
- [Мова запитів](docs/query-language.md)
- [Roadmap](docs/roadmap.md)
- [Архітектурні рішення (ADR)](docs/adr/)
- [Як долучитися](CONTRIBUTING.md)

## Ліцензія

[MIT](LICENSE)
