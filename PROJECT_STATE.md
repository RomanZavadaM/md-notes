# PROJECT_STATE — MD Notes

Оновлено: **01.10.2026**

## Поточний стан

- Статус розвитку: **ACTIVE**, етап v0.2.
- Версія в `main`: **v0.1.0**.
- Статус релізу: **test / prerelease checkpoint**.
- GitHub Release: **v0.1.0**, опубліковано 01.10.2026 (release PR #1).
- Активний продукт: Tauri 2 + React (`app/`), ядро Rust (`crates/notes-core`).
- Платформи збірки: Windows / macOS / Linux. Android та iOS — етап v0.3.
- Ліцензійна модель: proprietary / All Rights Reserved (Roman Zavada), `LICENSE.md`. Реліз v0.1.0 вийшов із файлом MIT `LICENSE`; цей файл видалено за вказівкою власника — див. `docs/LEGAL_AND_COPYRIGHT.md`, розділ «Історія ліцензії».

Детальна історія — у `CHANGELOG.md`, `docs/releases/`, merged PR і GitHub Issue #8.

## Що працює в v0.1.0

- відкриття локальної папки як сховища, дерево файлів;
- створення, перейменування, переміщення в кошик `.mdnotes/trash/`;
- редактор CodeMirror 6, перегляд, режим «поруч», автозбереження, атомарний запис;
- `[[вікі-посилання]]` з переходом і створенням відсутньої нотатки;
- `#теги` і властивості YAML front matter;
- світла, темна і системна теми, адаптивне компонування;
- приклад сховища `sample-vault/`.

## Verification v0.1.0

- CI: `cargo fmt`, `cargo clippy -D warnings`, тести `notes-core` на Windows/macOS/Linux, перевірка типів і збірка інтерфейсу — PASS;
- release run: Windows (MSI, NSIS), Linux (AppImage, deb, rpm), macOS (universal);
- runtime-перевірки у вікні застосунку в межах цього checkpoint **не виконувалися** (CI/compile evidence не прирівнюється до runtime validation).

## Межа доказу

- збірки не підписані: Windows SmartScreen, macOS Gatekeeper можуть попереджати;
- мобільні платформи ще не збираються;
- START/source пакет і `SHA256SUMS.txt` додаються до релізів, починаючи з наступного checkpoint.

## Наступний великий крок

v0.2 — індекс, пошук, зворотні посилання, оновлення посилань при перейменуванні, Mermaid/KaTeX, шаблони, локалізація інтерфейсу. Поточний стан робіт — `WORKLOG.md`.
