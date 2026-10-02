# WORKLOG — MD Notes

Оновлено: **02.10.2026**

## STATUS

**ACTIVE. Checkpoint v0.2.1 — DONE.** Поточний slice залишку v0.2: **типи нотаток + `schema.json` + форма властивостей**.

## Поточний slice

Branch: `feature/note-types-schema-v0.2`.

Реалізовано у candidate:

- `notes-core`: типізована модель наявного `.mdnotes/schema.json` v1 без міграції формату;
- сумісність із фактичним `sample-vault`: глобальні `fields`, `types.<name>.fields`, `required`, `template`;
- типи полів: `text`, `string`, `number`, `boolean`, `date`, `enum`, `list`, `link`, `links`, `url`, `file`;
- підтримка `readonly`, `values`, `noteType`;
- невідомі YAML-поля і невідомі note types не блокують відкриті дані;
- при помилковому YAML форма не переписує front matter;
- formatter працює з поточним текстом редактора у пам'яті, тому незбережене тіло нотатки не перезаписується окремим записом у файл;
- Tauri bridge: читання схеми, parse поточного editor content, safe formatting властивостей;
- типізований frontend API;
- schema-driven форма «Властивості» інтегрована в CodeMirror editor;
- форма підтримує required/readonly/enum/list/links/date/url/file/number/boolean/text;
- після застосування зміни повертаються у CodeMirror і проходять через звичайний autosave;
- UI локалізовано UK / EN / FR / DE / ES / KO / JA;
- додано `docs/SCHEMA.md` з форматом, прикладом і межами сумісності;
- unit-тести ядра: existing schema shape, missing schema, required/enum validation, invalid YAML, збереження невідомих полів, тіла і незбереженого тексту.

Важливе уточнення, знайдене під час реалізації: `sample-vault/.mdnotes/schema.json` уже мав визначений формат. Початковий мінімальний candidate було відразу перероблено під цей існуючий формат; міграція сховищ не потрібна.

## Поточна перевірка

CI ще не запускався для завершеного candidate. Runtime GUI-перевірка у поточному середовищі не виконувалась і не вважається виконаною.

## Наступна дія

1. створити PR для `feature/note-types-schema-v0.2`;
2. прогнати стандартний CI: fmt, clippy/tests Windows/macOS/Linux, frontend/Tauri, license gates;
3. виправити всі знайдені Rust/TypeScript/API нестикування на цій самій гілці;
4. після зеленого CI синхронізувати roadmap/state і інтегрувати PR у `main` як звичайний технічний merge;
5. **не створювати release checkpoint без окремої команди власника «зливай у main»**;
6. після цього наступний незавершений пункт v0.2 — пресети сховища: PARA / Zettelkasten / порожнє.

## Останній checkpoint

- GitHub prerelease [`v0.2.1`](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.2.1), 01.10.2026;
- attachment PR #16 → `main` squash commit `6b923369b8d5e3bb4e031f201dd9de7cd8c2c594`;
- release PR #14 → release commit `26661d8c088df51ce162018d2a3b525502cf30e1`;
- tag `v0.2.1`, release workflow #19 / run `36924302514` — **SUCCESS**;
- Windows/macOS/Linux packages + `MD-Notes-0.2.1-START.zip` + legal notices + `SHA256SUMS.txt` published.

## Відомі обмеження

- runtime GUI-перевірка v0.2.1 і поточного candidate вручну не виконувалась; CI/build evidence не є runtime test;
- форма властивостей може нормалізувати форматування YAML front matter; YAML-коментарі всередині front matter не гарантуються після застосування форми;
- окремого GUI-редактора самого `schema.json` поки немає;
- збірки не підписані;
- Android/iOS ще не збираються;
- великі сховища індексуються синхронно під час відкриття.
