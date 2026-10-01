# WORKLOG — MD Notes

Оновлено: **01.10.2026**

## STATUS

**ACTIVE.** Checkpoint **v0.2.0** опубліковано — DONE. CSP slice інтегровано в `main` через PR #15. Активний slice залишку v0.2: **вкладення**.

## Поточний slice

`feature/attachments-v0.2`, draft PR #16: керування вкладеннями у відкритому файловому форматі.

- база: `main` після merge PR #15 (`a8e1106b40b5552f453c410d96fbf5f1da9a243a`);
- ядро: імпорт файлу в `attachments/YYYY/MM/` без мовчазного перезапису;
- ядро: перелік вкладень;
- ядро: «де використовується» через посилання у Markdown;
- ядро: перелік вкладень без посилань;
- unit-тести: імпорт, унікальні імена, usage/orphan detection;
- Tauri bridge: `import_attachment`, `list_attachments`, `attachment_used_by`, `orphan_attachments`;
- frontend API: типізовані wrappers для attachment commands;
- UI: додано `AttachmentsPanel` з системним picker, списком, image thumbnails, orphan marker, usage details і відкриттям файлу;
- локалізація: нові attachment-рядки додані у UK / EN / FR / DE / ES / KO / JA.

## Наступна дія

Підключити `AttachmentsPanel` у `App.tsx`: додати вкладку sidebar «Вкладення», вставлення Markdown-посилання в поточну нотатку після імпорту, refresh tree/index. Далі додати стилі панелі, перевірити поведінку PDF (відкриття/перегляд), пройти стандартний CI + license gate і runtime-перевірку, якщо доступна. PR #16 залишати draft до завершення UI.

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
