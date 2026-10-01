# MD Notes v0.2.1 — release notes

Дата checkpoint: **01.10.2026**  
Статус: **test / prerelease**  
Tag: `v0.2.1`  
Release commit: `26661d8c088df51ce162018d2a3b525502cf30e1`

## Основне

v0.2.1 завершує два сумісні slice етапу v0.2: **restrictive CSP** і **керування вкладеннями**. Формат сховища лишається відкритим: нотатки — Markdown, вкладення — звичайні файли у папці сховища.

### Вкладення

- імпорт файлів у `attachments/YYYY/MM/`;
- без мовчазного перезапису: повторний імпорт отримує унікальне ім'я;
- окрема вкладка «Вкладення» у sidebar;
- системний file picker;
- автоматичне або ручне вставлення Markdown-посилання в поточну нотатку;
- thumbnails для зображень;
- локальний PDF preview;
- відкриття вкладення системною програмою;
- «де використовується» та кількість використань;
- перелік вкладень без посилань;
- підтримка URL-encoded Unicode-шляхів;
- новий UI локалізовано UK / EN / FR / DE / ES / KO / JA.

### Безпека

- Tauri webview більше не працює з `csp: null`;
- scripts обмежені `self`;
- network connect обмежений Tauri IPC;
- зовнішні `object`, `frame`, `form` targets заборонені;
- для vault assets та PDF frame дозволено лише локальний Tauri asset protocol;
- `style-src 'unsafe-inline'` лишається контрольованим винятком для runtime styles/CodeMirror до окремої runtime-перевірки.

### CI

- `notes-core`: format/clippy/tests — PASS на Windows, macOS, Linux;
- frontend build — PASS;
- Tauri clippy — PASS;
- cargo-deny + npm license gate — PASS;
- Tauri CI app-job вирівняно з Linux release environment `ubuntu-22.04`.

## Опубліковані пакети

### Windows

- `MD.Notes_0.2.1_x64-setup.exe`
- `MD.Notes_0.2.1_x64_en-US.msi`

### macOS universal

- `MD.Notes_0.2.1_universal.dmg`
- `MD.Notes_universal.app.tar.gz`

### Linux x86_64

- `MD.Notes_0.2.1_amd64.AppImage`
- `MD.Notes_0.2.1_amd64.deb`
- `MD.Notes-0.2.1-1.x86_64.rpm`

### Source / legal / verification

- `MD-Notes-0.2.1-START.zip`
- `LICENSE.md`
- `COPYRIGHT.md`
- `THIRD_PARTY_NOTICES.md`
- `SHA256SUMS.txt`

Release workflow #19 (`36924302514`) завершився **SUCCESS**.

## Межа перевірки

- ручний runtime GUI test v0.2.1 не виконувався; зелені CI та platform builds не подаються як заміна runtime validation;
- збірки не підписані, тому Windows SmartScreen і macOS Gatekeeper можуть показувати попередження;
- Android/iOS ще не входять у цей checkpoint.

## Наступний пункт roadmap

Наступний незавершений slice v0.2: **типи нотаток + `schema.json` + форма властивостей**. До v0.3 (mobile + Git/WebDAV sync) не переходити без рішення власника.
