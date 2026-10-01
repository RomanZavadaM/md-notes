# WORKLOG — MD Notes

Оновлено: **01.10.2026**

## STATUS

**ACTIVE. Checkpoint v0.2.1 — DONE.** CSP та вкладення інтегровані в `main`, prerelease `v0.2.1` опубліковано з пакетами Windows/macOS/Linux, START/source, legal notices і SHA256SUMS.

## Останній checkpoint

- GitHub prerelease [`v0.2.1`](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.2.1), 01.10.2026;
- attachment PR #16 → `main` squash commit `6b923369b8d5e3bb4e031f201dd9de7cd8c2c594`;
- release PR #14 → release commit `26661d8c088df51ce162018d2a3b525502cf30e1`;
- tag: `v0.2.1`, target `26661d8c088df51ce162018d2a3b525502cf30e1`;
- release workflow #19 / run `36924302514` — **SUCCESS**;
- Windows: `MD.Notes_0.2.1_x64-setup.exe`, `MD.Notes_0.2.1_x64_en-US.msi`;
- macOS universal: `MD.Notes_0.2.1_universal.dmg`, `MD.Notes_universal.app.tar.gz`;
- Linux: `MD.Notes_0.2.1_amd64.AppImage`, `MD.Notes_0.2.1_amd64.deb`, `MD.Notes-0.2.1-1.x86_64.rpm`;
- source/test package: `MD-Notes-0.2.1-START.zip`;
- legal/checksums: `LICENSE.md`, `COPYRIGHT.md`, `THIRD_PARTY_NOTICES.md`, `SHA256SUMS.txt`.

## Завершено у v0.2.1

### CSP
- restrictive Tauri CSP замість `csp: null`;
- scripts лише `self`, network connect лише Tauri IPC;
- локальні vault assets через Tauri asset protocol;
- local-only frame policy для PDF preview;
- зовнішні object/frame/form targets заблоковані.

### Вкладення
- імпорт у `attachments/YYYY/MM/`;
- повторний імпорт не перезаписує файл, створюється унікальне ім'я;
- Tauri bridge + типізований frontend API;
- окрема вкладка «Вкладення» у sidebar;
- системний file picker;
- автоматичне та ручне вставлення Markdown-посилання в нотатку;
- image thumbnails та локальний PDF preview;
- відкриття вкладення системною програмою;
- «де використовується», кількість використань, вкладення без посилань;
- URL-encoded Unicode paths;
- attachment UI усіма 7 мовами;
- unit-тести ядра.

### CI / packaging
- виправлено Clippy `unnecessary_sort_by`;
- Tauri CI app-job вирівняно з release Linux environment (`ubuntu-22.04`);
- PR #16 clean CI #80: Windows/macOS/Linux core, frontend/Tauri, license gates — PASS;
- release builds Windows/macOS/Linux та assets job — PASS.

## Поточний slice

**Не розпочато.** Наступний незавершений пункт roadmap v0.2: **типи нотаток + `schema.json` + форма властивостей**.

## Наступна дія

Перед кодом нового slice:
1. перечитати `docs/roadmap.md` і поточну модель front matter;
2. спроєктувати мінімальний відкритий `schema.json` без vendor lock-in;
3. визначити сумісність із наявними шаблонами та довільним YAML front matter;
4. реалізувати через branch + PR з tests і локалізацією 7 мовами;
5. не переходити до v0.3 без рішення власника.

## Відомі обмеження

- runtime GUI-перевірка v0.2.1 вручну не виконувалась; CI/build evidence не є runtime test;
- збірки не підписані;
- Android/iOS ще не збираються;
- великі сховища індексуються синхронно під час відкриття.
