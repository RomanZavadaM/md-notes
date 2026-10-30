# MD Notes v1.0.0 — stable desktop release

Дата підготовки: 30.10.2026

## Статус

**MD Notes 1.0.0 — перша повноцінна stable desktop-версія.**

Рішення власника: функціональну розробку першої desktop-версії завершено; після 1.0 продукт переходить у режим реального використання, стабілізації та окремих post-1.0 slices.

Цільові платформи 1.0:

- Windows x64;
- macOS Universal (Apple Silicon + Intel);
- Linux x86_64.

Android/iOS не є частиною обіцянки desktop 1.0: CI підтверджує buildability, але physical-device runtime лишається post-1.0.

## Що входить у 1.0

### Markdown і база знань

- звичайні `.md/.markdown` файли без proprietary content format;
- редактор CodeMirror, preview/split, autosave й атомарні записи;
- tabs, standalone Markdown і multi-window/detached views;
- Mermaid, KaTeX, images, attachments, templates, daily notes;
- schema-driven properties та Empty / PARA / Zettelkasten presets;
- SQLite/FTS5 search index, tags, aliases, backlinks і quick open.

### Зв'язки та граф

- wiki-links і typed `relations`;
- класи A1–F6 та оцінка надійності;
- global/local graph;
- relation tree;
- weakest-link path semantics;
- окреме native graph window;
- детальні людські процедури 7 мовами та окремий канонічний AI contract.

### Зашифроване сховище

- age/X25519;
- random physical names для нових encrypted vaults;
- working private key у системному secret store;
- recovery recipient можна додати до вже створеного vault;
- images і PDF читаються через active storage provider, розшифровуються лише в пам'яті та відображаються через Blob URL;
- external “Open file” для encrypted attachments навмисно вимкнене, щоб не створювати plaintext temp file на диску;
- Git push guard не дозволяє відправляти plaintext із encrypted vault.

Boundary 1.0: `.mdnotes/cache` не входить у encrypted trust boundary.

### Backup / restore / recovery

- portable ZIP backup;
- manifest + SHA-256;
- включаються notes, attachments, config, templates і trash;
- виключаються `.git`, `.mdnotes/cache` та тимчасовий системний шум;
- encrypted vault копіюється як ciphertext, без recovery/private keys;
- restore повністю валідовує archive до запису;
- restore дозволений лише в новий vault, без silent overwrite.

Перед 1.0 додано автоматизований encrypted recovery drill:
**створити encrypted vault → note + attachment → backup → restore → відкрити recovery private key → прочитати note + attachment**.

## Stabilization gates перед 1.0

- PR #192 — encrypted-vault recovery UX + safe in-memory attachment preview.
  - CI run `41079247425` = SUCCESS.
  - Mobile smoke `41079247290` = SUCCESS.
- PR #194 — 7-language parity + production npm audit gate.
  - CI run `41081985372` = SUCCESS.
  - Mobile smoke `41081985348` = SUCCESS.
  - i18n parity перевіряє UK / EN / FR / DE / ES / KO / JA.
  - production audit блокує high/critical advisories; на checkpoint лишались тільки low KaTeX-chain advisories.
- PR #196 — Windows offline installer packaging.
  - CI run `41083660181` = SUCCESS.
  - Mobile smoke `41083660161` = SUCCESS.
  - NSIS/MSI реально зібрані з offline WebView2; smoke artifact id `12991027276`, digest `sha256:1b328fab1502a03f2471cb12c8b9e884cdfebd6f2438db6ecdab82300a0659a7`.
- PR #197 — encrypted backup/recovery drill.
  - CI run `41085366766` = SUCCESS.
  - Mobile smoke `41085366949` = SUCCESS.

## Windows packaging

Для 1.0:

- NSIS EXE і MSI містять **offline WebView2 installer**;
- installer bundle також включає VC runtime;
- Windows portable ZIP лишається легким і використовує WebView2 Runtime, уже встановлений у Windows.

## Мови

Інтерфейс і основна user-facing документація: **UK / EN / FR / DE / ES / KO / JA**.

CI має автоматичний parity gate, щоб спеціалізований language map не міг тихо втратити одну з семи мов.

## Що свідомо не блокує desktop 1.0

- Android/iOS physical-device acceptance;
- WebDAV;
- Git ↔ SyncManifest conflict policy;
- batch move / batch rename;
- encrypted cache;
- store publication;
- signing/notarization без реальних credentials;
- нові task/calendar/cloud/canvas features.

## Відомі release boundaries

- збірки unsigned / not notarized;
- Windows portable залежить від системного WebView2 Runtime;
- automated recovery drill пройдено, але окремий ручний owner backup/restore drill на реальних даних не заявляється як виконаний;
- mobile build smoke ≠ physical-device runtime acceptance.

## Дані та сумісність

MD Notes не вимагає міграції нотаток у власний формат. Звичайні vault залишаються звичайними папками Markdown. Existing vault можна відкрити без перетворення; encrypted vault є окремим свідомим режимом.

Copyright © 2026 Roman Zavada / Роман Завада. All Rights Reserved.
