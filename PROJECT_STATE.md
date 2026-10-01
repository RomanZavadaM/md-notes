# PROJECT_STATE — MD Notes

Оновлено: **01.10.2026**

## Поточний стан

- Статус розвитку: **ACTIVE**.
- Версія в `main`: **v0.2.1**.
- Статус релізу: **test / prerelease checkpoint**.
- GitHub prerelease: **v0.2.1**, 01.10.2026. Попередній реліз: v0.2.0.
- Release commit: `26661d8c088df51ce162018d2a3b525502cf30e1`; attachment integration: PR #16 / `6b923369b8d5e3bb4e031f201dd9de7cd8c2c594`; CSP integration: PR #15 / `a8e1106b40b5552f453c410d96fbf5f1da9a243a`.
- Активний продукт: Tauri 2 + React (`app/`), ядро Rust (`crates/notes-core`).
- Платформи збірки: Windows / macOS / Linux. Android та iOS — етап v0.3.
- UI-мови: UK / EN / FR / DE / ES / KO / JA.
- Ліцензійна модель: proprietary / All Rights Reserved (Roman Zavada), єдина ліцензія — `LICENSE.md`. Публічний репозиторій не надає open-source ліцензії.

Детальна історія — у `CHANGELOG.md`, `docs/releases/`, merged PR і GitHub Issue #8.

## Що працює у v0.2.1

- локальна папка як сховище, дерево файлів, створення, перейменування, кошик `.mdnotes/trash/`;
- редактор CodeMirror 6, перегляд, режим «поруч», автозбереження, атомарний запис;
- індекс SQLite (`.mdnotes/cache/index.db`), повнотекстовий пошук, теги, швидкий перехід;
- `[[вікі-посилання]]`, `aliases`, посилання у властивостях, зворотні посилання з контекстом;
- оновлення посилань при перейменуванні та переміщенні нотаток і папок;
- стеження за змінами файлів ззовні (настільні ОС);
- Mermaid 11, KaTeX, зображення зі сховища, відносні посилання на `.md`;
- шаблони нотаток, щоденні нотатки;
- інтерфейс сімома мовами, вікно «Про програму»;
- світла, темна і системна теми, адаптивне компонування;
- restrictive Tauri CSP: зовнішні scripts/frames/network sources не дозволені, локальний Tauri asset protocol дозволений для vault assets та PDF preview;
- вкладення: імпорт у `attachments/YYYY/MM/`, унікальні імена без мовчазного перезапису, вставлення Markdown-посилання, image/PDF preview, відкриття файлу, «де використовується», пошук файлів без посилань, підтримка URL-encoded Unicode-шляхів.

## Verification v0.2.1

- PR #16 CI #80: `cargo fmt`, `cargo clippy -D warnings`, тести `notes-core` на Windows/macOS/Linux, frontend build, Tauri clippy, cargo-deny + npm license gate — **PASS**;
- release workflow #19 (`36924302514`) — **SUCCESS**;
- Windows: `.exe` + `.msi` — зібрано й опубліковано;
- macOS: universal `.dmg` + `.app.tar.gz` — зібрано й опубліковано;
- Linux: `.AppImage` + `.deb` + `.rpm` — зібрано й опубліковано;
- `MD-Notes-0.2.1-START.zip`, `LICENSE.md`, `COPYRIGHT.md`, `THIRD_PARTY_NOTICES.md`, `SHA256SUMS.txt` — опубліковано;
- runtime-перевірки у вікні застосунку **не виконувалися** (CI/compile evidence не прирівнюється до runtime validation).

## Межа доказу

- збірки не підписані: Windows SmartScreen, macOS Gatekeeper можуть попереджати;
- мобільні платформи ще не збираються;
- відкриття великого сховища індексує його синхронно під час відкриття.

## Наступний великий крок

Залишок v0.2: типи нотаток і `schema.json` з формою властивостей, пресети сховища (PARA / Zettelkasten / порожнє), граф знань. Після завершення v0.2 — етап v0.3 (Android, iOS, Git/WebDAV sync). Поточна робота — `WORKLOG.md`.
