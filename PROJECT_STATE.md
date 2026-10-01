# PROJECT_STATE — MD Notes

Оновлено: **01.10.2026**

## Поточний стан

- Статус розвитку: **ACTIVE**.
- Версія в `main`: **v0.2.0** (release PR від release-please).
- Статус релізу: **test / prerelease checkpoint**.
- GitHub prerelease: **v0.2.0**, 01.10.2026. Попередній реліз: v0.1.0.
- Інтегровано в цьому checkpoint: PR #2, #3, #4, #5, #6, #7, #9, #10.
- Активний продукт: Tauri 2 + React (`app/`), ядро Rust (`crates/notes-core`).
- Платформи збірки: Windows / macOS / Linux. Android та iOS — етап v0.3.
- UI-мови: UK / EN / FR / DE / ES / KO / JA.
- Ліцензійна модель: proprietary / All Rights Reserved (Roman Zavada), єдина ліцензія — `LICENSE.md`. Реліз v0.1.0 вийшов із файлом MIT `LICENSE`, який згодом видалено за вказівкою власника (див. `docs/LEGAL_AND_COPYRIGHT.md`).

Детальна історія — у `CHANGELOG.md`, `docs/releases/`, merged PR і GitHub Issue #8.

## Що працює в v0.2.0

- локальна папка як сховище, дерево файлів, створення, перейменування, кошик `.mdnotes/trash/`;
- редактор CodeMirror 6, перегляд, режим «поруч», автозбереження, атомарний запис;
- індекс SQLite (`.mdnotes/cache/index.db`), повнотекстовий пошук, теги, швидкий перехід;
- `[[вікі-посилання]]`, `aliases`, посилання у властивостях, зворотні посилання з контекстом;
- оновлення посилань при перейменуванні та переміщенні нотаток і папок;
- стеження за змінами файлів ззовні (настільні ОС);
- Mermaid 11, KaTeX, зображення зі сховища, відносні посилання на `.md`;
- шаблони нотаток, щоденні нотатки;
- інтерфейс сімома мовами, вікно «Про програму»;
- світла, темна і системна теми, адаптивне компонування.

## Verification v0.2.0

- CI: `cargo fmt`, `cargo clippy -D warnings`, тести `notes-core` (зокрема на `sample-vault`) на Windows/macOS/Linux, перевірка типів і збірка інтерфейсу, license gate (cargo-deny + npm) — PASS;
- release run: пакети Windows, macOS, Linux; START/source, legal notices, `SHA256SUMS.txt`;
- runtime-перевірки у вікні застосунку **не виконувалися** (CI/compile evidence не прирівнюється до runtime validation).

## Межа доказу

- збірки не підписані: Windows SmartScreen, macOS Gatekeeper можуть попереджати;
- мобільні платформи ще не збираються;
- відкриття великого сховища індексує його синхронно під час відкриття.

## Наступний великий крок

Залишок v0.2 (вкладення, форма властивостей за `schema.json`, пресети сховища, граф знань, строга CSP) і v0.3 (Android, iOS, синхронізація Git/WebDAV). Поточна робота — `WORKLOG.md`.
