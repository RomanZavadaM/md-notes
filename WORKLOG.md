# WORKLOG — MD Notes

Оновлено: **01.10.2026**

## STATUS

**ACTIVE.** Checkpoint **v0.2.0** опубліковано — DONE. CSP slice інтегровано в `main` через PR #15. Активний slice залишку v0.2: **вкладення**.

## Поточний slice

`feature/attachments-v0.2`, PR #16 (ready for review): керування вкладеннями у відкритому файловому форматі.

- база: `main` після merge PR #15 (`a8e1106b40b5552f453c410d96fbf5f1da9a243a`);
- ядро: імпорт файлу в `attachments/YYYY/MM/` без мовчазного перезапису;
- ядро: перелік вкладень;
- ядро: «де використовується» через посилання у Markdown;
- ядро: перелік вкладень без посилань;
- ядро: розпізнавання URL-encoded Unicode-шляхів у Markdown;
- unit-тести: імпорт, унікальні імена, usage/orphan detection, Unicode URL path;
- Tauri bridge: `import_attachment`, `list_attachments`, `attachment_used_by`, `orphan_attachments`;
- frontend API: типізовані wrappers для attachment commands;
- UI: окрема вкладка sidebar «Вкладення»;
- UI: системний file picker, список, image thumbnails, orphan marker, usage details і відкриття файлу;
- UI: вставлення Markdown-посилання в поточну нотатку після імпорту або вручну для вибраного вкладення;
- UI: inline preview зображень і локальний PDF preview;
- CSP: `frame-src` дозволяє лише `self` та локальний Tauri asset protocol; зовнішні frame лишаються забороненими;
- локалізація: attachment UI синхронізовано для UK / EN / FR / DE / ES / KO / JA;
- стилі: окремий `AttachmentsPanel.css`, включно з вузьким layout.

## Поточна перевірка

PR #16 переведено з draft у ready for review. GitHub Actions CI run #36922336498 / run number 76 створено для head `3c776e5928d1178a7c54559676f3df2e330fc9d1`; на момент цього запису він очікує runner (`pending`). Runtime GUI-перевірка в поточному середовищі не виконувалась і не вважається виконаною.

## Наступна дія

Дочекатися результату CI #76. Якщо є помилки — виправити на цій самій гілці й повторити checks. Якщо всі стандартні checks Windows/macOS/Linux, frontend/Tauri та license gate зелені — інтегрувати PR #16 у `main`, синхронізувати `docs/roadmap.md`, `WORKLOG.md` та Issue #8 і перейти до наступного незавершеного slice v0.2.

## Останній checkpoint

- GitHub prerelease [`v0.2.0`](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.2.0), 01.10.2026;
- release PR #11, release commit `d3302b0527839ac2b5cbf0f09d14e6ed58500f51`;
- release run #36913195476 — SUCCESS (Windows, macOS universal, Linux, assets);
- assets: `.exe`, `.msi`, `.dmg`, `.app.tar.gz`, `.AppImage`, `.deb`, `.rpm`, `MD-Notes-0.2.0-START.zip`, `LICENSE.md`, `COPYRIGHT.md`, `THIRD_PARTY_NOTICES.md`, `SHA256SUMS.txt` (контрольні суми перевірено).

## Нещодавно завершено

- CSP: PR #15 — restrictive Tauri CSP, CI PASS, інтегровано в `main` (`a8e1106`).
- v0.2.0 scope: PR #2 (індекс), #3 (пошук, теги, панель зв'язків, стеження), #4 (оновлення посилань), #5 (Mermaid/KaTeX/зображення), #6 (перейменування в застосунку), #7 (шаблони, щоденні нотатки), #9 (правила, ліцензія, документація 7 мовами, license gate), #10 (інтерфейс 7 мовами, «Про програму») — DONE.
- v0.1.0: каркас, CI, release-please, реліз (PR #1, `447e8bf`).
- Аудит ліцензій: 501 Rust-крейт і 400 npm-пакетів; Mermaid 12 → 11.17 через EPL-2.0 `elkjs`.

## Відомі обмеження

- runtime-перевірки у вікні застосунку не виконувалися (немає локального Rust/Node на робочій машині розробки);
- збірки не підписані.
