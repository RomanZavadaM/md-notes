# Roadmap

Порядок етапів змінено порівняно з початковим планом. Мобільна версія без
синхронізації мало корисна, тому базова синхронізація (Git і WebDAV)
переїхала у v0.3, разом з Android та iOS. Причини описані в
[ADR-0003](adr/0003-storage-providers.md) і
[ADR-0006](adr/0006-mobile-file-access.md).

## v0.1 — ядро і настільна версія ✅

- [x] Монорепо: Cargo workspace, Tauri 2, React + Vite, CI на трьох ОС
- [x] `notes-core`: відкриття сховища, дерево файлів, читання й атомарний запис
- [x] Розбір front matter, `[[посилань]]`, `#тегів`, заголовків
- [x] Створення, перейменування, переміщення в кошик
- [x] Редактор CodeMirror 6, перегляд, режим «поруч», автозбереження
- [x] Перехід за `[[посиланням]]` і створення відсутньої нотатки
- [x] Світла, темна і системна теми, адаптивне компонування
- [x] Приклад сховища, документація, ADR
- [x] Строга CSP для вебвікна — завершено у v0.2.1

## v0.2 — структура і зв'язки ✅ checkpoint v0.2.2 опублікований

- [x] Індекс SQLite у `.mdnotes/cache/index.db`, інкрементальне оновлення
- [x] Повнотекстовий пошук (FTS5), швидкий перехід `Ctrl+O`
- [x] Зворотні посилання, пошук за aliases, теги
- [x] Оновлення посилань при перейменуванні та переміщенні
- [x] Відстеження зовнішніх змін файлів
- [x] Mermaid, KaTeX, вкладення, шаблони та daily notes
- [x] Інтерфейс сімома мовами (UK, EN, FR, DE, ES, KO, JA)
- [x] Schema-driven properties
- [x] PARA / Zettelkasten / Empty presets
- [x] Global/local knowledge graph
- [x] Windows portable
- [x] Повний test-release checkpoint v0.2.2
- [x] Windows runtime-test v0.2.2 — PASS 03.10.2026

## v0.3 — мобільні платформи і синхронізація

### Завершено до checkpoint v0.3.1

- [x] Provider-neutral storage foundation
- [x] Mobile sandbox vault
- [x] Android/iOS CI build smoke
- [x] Local-first sync planning foundation
- [x] Persistent sync state
- [x] Git clone/open foundation
- [x] Dirty worktree detection
- [x] Local Git commit
- [x] Safe HTTPS fetch
- [x] In-memory Git authentication
- [x] System credential storage
- [x] Standalone Markdown open/edit/save
- [x] Windows/Linux/macOS Markdown desktop integration
- [x] Git pull relation planner
- [x] Git pull safety preflight
- [x] Safe clean fast-forward apply
- [x] Повний test-release checkpoint **v0.3.1**

### Зроблено після v0.3.1 (релізи 0.3.2-0.3.18)

- [x] Імпорт папки у сховище, об’єднання за кодом ЄДРПОУ/ІПН ([ADR-0008](adr/0008-identity-merge.md)), список можливих дублікатів
- [x] Граф: жива фізика, окреме вікно, форми й легенда, оцінка надійності, «звідки це прийшло», дерево зв’язків, найнадійніший шлях
- [x] Панель Git у застосунку
- [x] Факти з локальної копії даних «З-під Ляди» за точним кодом у пошуку

### Наступне

- [x] **Зашифроване сховище (age), локальні комп'ютери** (ADR-0007; рішення власника 19.10.2026): зріз 0 - дослідження, де ядро читає файли поза `StorageProvider`; зріз 1 - `EncryptedProvider` за feature `encrypted-vault`; зріз 2 - індексатор і вкладення через провайдера; зріз 3 - інтерфейс і ключі в системному сховищі секретів; зрізи 4-5 (стиль вузлів і зв'язків графа за властивостями, режим трасування, ієрархічна розкладка) - лише за окремою вказівкою власника; зрізи 1-3 готові (0.3.2+), імена файлів випадкові, імпорт папки шифрується дорогою; граф: форми, легенда, дерево зв'язків, найнадійніший шлях, фільтр за оцінкою, окреме вікно
- [x] **Git push** через системний `git` із запобіжником для зашифрованих сховищ (відкритий текст не відправляється); у живому застосунку не перевірено
- [ ] Зашифроване сховище, прогалини: зображення й вкладення, ключ відновлення для готового сховища, шифрування кеш-індексу
- [ ] Git state ↔ `SyncManifest` / ADR-0005 conflict policy
- [ ] WebDAV
- [ ] Physical Android/iOS runtime validation
- [ ] Optional Android SAF / iOS security-scoped external folders
- [ ] Tables / query language follow-up
- [ ] Mobile table-as-cards modes

## v0.4 — час і задачі

- Канбан (dnd-kit), на мобільних по одній колонці
- Календар, часова шкала
- Щоденні нотатки
- SFTP і S3-сумісні сховища

## v0.5 — хмари і полотно

- Google Drive, OneDrive, Dropbox
- Полотно у форматі JSON Canvas (React Flow)
- Інтелект-карти (markmap), галерея вкладень

## v0.6 — історія версій

- Історія нотатки з Git, порівняння версій, відновлення
- SMB

## v1.0 — публікація

- Microsoft Store, Mac App Store, Google Play, App Store
- Підписування коду, автооновлення
- Стабілізація форматів `config.json`, `schema.json`, `*.view.json`
