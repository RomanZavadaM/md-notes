# MD Notes

**🇺🇦 Українська** · [🇬🇧 English](docs/readme/README.en.md) · [🇫🇷 Français](docs/readme/README.fr.md) · [🇩🇪 Deutsch](docs/readme/README.de.md) · [🇪🇸 Español](docs/readme/README.es.md) · [🇰🇷 한국어](docs/readme/README.ko.md) · [🇯🇵 日本語](docs/readme/README.ja.md)

[![CI](https://github.com/RomanZavadaM/md-notes/actions/workflows/ci.yml/badge.svg)](https://github.com/RomanZavadaM/md-notes/actions/workflows/ci.yml)

<!-- x-release-please-start-version -->
> **Release checkpoint: MD Notes v1.0.0**
>
> **Статус: desktop feature-complete; stable v1.0.0 опубліковано. Проєкт перейшов у режим використання й стабілізації.**

## Завантажити MD Notes v1.0.0

**Швидке встановлення:**

- **Windows 64-bit:** [завантажити інсталятор .exe](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD.Notes_1.0.0_x64-setup.exe) · [MSI](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD.Notes_1.0.0_x64_en-US.msi) · [portable ZIP](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD-Notes-1.0.0-Windows-x64-portable.zip)
- **macOS Universal:** [завантажити DMG](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD.Notes_1.0.0_universal.dmg)
- **Linux x86_64:** [AppImage](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD.Notes_1.0.0_amd64.AppImage) · [DEB](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD.Notes_1.0.0_amd64.deb) · [RPM](https://github.com/RomanZavadaM/md-notes/releases/download/v1.0.0/MD.Notes-1.0.0-1.x86_64.rpm)

[**Усі файли релізу v1.0.0 →**](https://github.com/RomanZavadaM/md-notes/releases/tag/v1.0.0)
<!-- x-release-please-end -->

> Збірки поки не підписані/не нотаризовані, тому Windows, macOS або Linux можуть показувати стандартне попередження безпеки.

> **Windows 1.0 packaging:** NSIS/MSI містять offline WebView2 installer і VC runtime; portable ZIP лишається легким та використовує WebView2 Runtime, уже встановлений у Windows.

## Про продукт

**MD Notes** — кросплатформний local-first застосунок для особистої бази знань у Markdown. Він працює зі звичайними `.md/.markdown` файлами, вкладеннями та відкритими службовими JSON-файлами без прихованого формату даних.

**Дані належать користувачу.** Нотатки можна відкрити будь-яким редактором, зберігати у власній файловій системі та версіонувати через Git. MD Notes не має власного сервера, аналітики чи телеметрії.

Платформи desktop 1.0: **Windows, macOS, Linux**. Android/iOS збираються в CI, але фізичний runtime і публікація в mobile stores перенесені після desktop 1.0.

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
- жива фізика графа («плаваючий» граф), граф в окремому вікні, дерево зв’язків із оцінкою за найслабшою ланкою, шлях між двома нотатками;
- пошук суб’єкта за точним кодом (ЄДРПОУ/ІПН) чи назвою, імпорт папки з об’єднанням за кодом, можливі дублікати;
- UI сімома мовами, світла/темна/системна теми.

### v0.3 — інтегрований стан до checkpoint v0.3.18

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
- опціональне зашифроване сховище (age): нотатки лежать на диску як `.age`, приватний ключ зберігається лише в системному сховищі секретів пристрою (Windows Credential Manager, macOS Keychain, Linux Secret Service), у діалогах — створення з показом ключа один раз, імпорт ключа на іншому пристрої, необов’язковий ключ відновлення;
- Git push через системний `git` (на комп’ютері має бути встановлений Git) із запобіжником для зашифрованих сховищ: відкритий текст не відправляється; панель Git у застосунку (стан, fetch, pull, commit, push).

Ще **не завершено**: повна інтеграція Git state з `SyncManifest` / ADR-0005 conflict policy, WebDAV, runtime-перевірка на фізичних Android/iOS пристроях та optional Android SAF / iOS security-scoped external folders.

**Зашифроване сховище** — опція, а не вимога: звичайні vault працюють без змін, а збірка без feature `encrypted-vault` не містить коду шифрування. У стабілізації 1.0 зображення й PDF у encrypted vault відображаються через розшифрування лише в пам’яті, а до вже створеного vault можна додати публічний recovery recipient через меню «Файл». Зовнішнє відкриття encrypted attachment вимкнене, щоб не створювати plaintext-файл на диску; кеш-індекс `.mdnotes/cache` лишається незашифрованим, а Git push проходить через запобіжник, що не відправляє відкритий текст.

Повний план — [docs/roadmap.md](docs/roadmap.md).

> Переклади README, керівництва користувача та рядків інтерфейсу для FR / DE / ES / KO / JA створені автоматично й не вичитані носіями мов; еталон змісту — українська. Усе в цьому розділі перевірено автоматичними тестами та збіркою на GitHub; ручну перевірку в живому застосунку та на фізичних Android/iOS пристроях власник ще не підтвердив.

## Встановлення

Завантажте пакет зі сторінки [релізів](https://github.com/RomanZavadaM/md-notes/releases):

- **Windows** — `.exe`, `.msi` або portable ZIP;
- **macOS** — `.dmg` / `.app.tar.gz`;
- **Linux** — `.AppImage`, `.deb` або `.rpm`.

Checkpoint **v0.3.18** включає standalone Markdown і безпечний Git fast-forward pipeline. Release builds поки не підписані/не нотаризовані, якщо сторінка конкретного релізу прямо не вказує інше.

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
- **Детальні принципи:** [зв’язки і граф](docs/concepts/KNOWLEDGE_GRAPH.uk.md) · [зашифроване сховище](docs/security/ENCRYPTED_VAULT.uk.md) · [backup і відновлення](docs/security/BACKUP_AND_RECOVERY.uk.md)
- **Для ШІ/автоматизації:** [канонічні контракти](docs/ai/README.md)

## Дані, приватність і Git credentials

MD Notes працює local-first. Нотатки, вкладення та rebuildable index залишаються на пристрої або у вибраному користувачем сховищі. Секрети синхронізації не зберігаються у vault, Markdown-файлах, remote URL, Git config, логах чи `localStorage`; для них використовується системне credential storage.

Конфлікти синхронізації не повинні перезаписуватися мовчки. Автоматичний Git apply дозволений тільки для чистого fast-forward стану.

## Авторські права

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.**

MD Notes — **proprietary software**. Публічний репозиторій не надає open-source ліцензії. Ваші нотатки належать вам.

Див. [LICENSE.md](LICENSE.md), [COPYRIGHT.md](COPYRIGHT.md), [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md), [LEGAL_AND_COPYRIGHT.md](docs/LEGAL_AND_COPYRIGHT.md).


## Вкладки та багатовіконний редактор (stable v1.0.0)

У stable **v1.0.0** багатовіконний сценарій ADR-0010 прийнятий для desktop: вкладки, окремі editor-вікна та незалежне вікно графа входять до стабільної бази. Кілька нотаток сховища можна тримати відкритими у вкладках; кожна вкладка має власну чернетку. На desktop активну нотатку можна перенести кнопкою **«Відкрити в окремому вікні»** у незалежне вікно редактора. Перед перенесенням зміни зберігаються, а головне вікно повертається до попередньої вкладки або порожнього стану.

Conflict guard, safe-close та save-all/safe Quit інтегровані. Історичний Windows-тест v0.3.21 від 28.10.2026 ще виявляв неправильну маршрутизацію кліків графа; подальші r4–r8 це виправили, owner runtime-приймання завершено 29.10.2026, і багатовіконний сценарій входить до stable v1.0.0.


## Резервне копіювання і відновлення

У меню **Файл** доступні ручні команди **«Створити резервну копію…»** та **«Відновити резервну копію…»**.

- backup — переносимий ZIP із `manifest.json` і SHA-256 для кожного файла;
- перед backup MD Notes зберігає відкриті нотатки;
- до копії входять нотатки, вкладення, конфігурація, шаблони та кошик; не входять `.git`, `.mdnotes/cache` і тимчасовий системний шум;
- зашифроване сховище копіюється **без розшифрування**; приватний/recovery key ніколи не додається до архіву;
- restore спочатку перевіряє структуру, розміри й контрольні суми, а потім створює **нову** папку сховища; чинне сховище не перезаписується.
