# WORKLOG — MD Notes

Оновлено: **02.10.2026**

## STATUS

**ACTIVE. Checkpoint v0.2.1 — DONE.** Runtime-startup hotfix уже інтегровано в `main`; поточний функціональний candidate: **типи нотаток + `schema.json` + форма властивостей**.

## Runtime blocker 02.10.2026

Користувач підтвердив на реальній Windows-збірці: застосунок встановлюється і запускається, але після запуску комп'ютер може сильно зависати; симптом видно навіть при спробі відкрити «Про програму».

Аудит показав небезпечний startup path: frontend автоматично відкривав `mdnotes.lastVault`, backend синхронно індексував vault, а дерево файлів після цього будувалося ще раз. Hotfix PR #21 прибрав автоматичне відкриття останнього vault під час startup.

- PR #21 — `fix: prevent unsafe vault auto-reopen on startup`;
- CI #92 — **PASS**;
- merge у `main`: `de7b9d1b8fa7b9f5c7884b4082d977f4a1029b01`.

Hotfix ще потребує реального Windows runtime-підтвердження. Якщо зависання лишиться після ручного вибору vault, наступний крок — оптимізація `note_paths()` / `get_tree()` та винесення дорогого index sync із критичного startup/open path.

## Поточний slice

Branch: `feature/note-types-schema-v0.2`.
PR #19: `feat: add schema-driven note properties`.

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

Важливе уточнення: `sample-vault/.mdnotes/schema.json` уже мав визначений формат. Candidate приведено до цього існуючого формату; міграція сховищ не потрібна.

## Перевірка PR #19

- перший прогін виявив `cargo fmt` і відсутній direct dependency `serde_json` у Tauri crate;
- обидві проблеми виправлено на цій самій гілці;
- CI #91 — **PASS**: notes-core Windows/macOS/Linux, frontend build + Tauri clippy, dependency licenses, Conventional PR title;
- runtime GUI-перевірка schema/property form у поточному середовищі не виконувалась і не вважається виконаною.

## Паралельний packaging slice

- PR #20 — `build: add Windows portable release package`;
- portable додається як стандартний release artifact поряд із `.exe` та `.msi`;
- після аудиту виправлено пошук binary у Cargo workspace `target/release` із fallback;
- portable не вважається виправленням runtime-зависання.

## Наступна дія

1. дочекатися clean CI оновленого PR #20;
2. інтегрувати PR #20 у `main` після green checks;
3. переконатися, що PR #19 не конфліктує з runtime-hotfix і не повертає auto-reopen;
4. перевести PR #19 з draft у ready та інтегрувати після green/mergeability checks;
5. після merge оновити roadmap/state і Issue #8;
6. **не створювати release checkpoint без окремої команди власника «зливай у main»**;
7. наступний незавершений пункт v0.2 після schema slice — пресети сховища: PARA / Zettelkasten / порожнє.

## Останній checkpoint

- GitHub prerelease [`v0.2.1`](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.2.1), 01.10.2026;
- attachment PR #16 → `main` squash commit `6b923369b8d5e3bb4e031f201dd9de7cd8c2c594`;
- release PR #14 → release commit `26661d8c088df51ce162018d2a3b525502cf30e1`;
- tag `v0.2.1`, release workflow #19 / run `36924302514` — **SUCCESS**;
- Windows/macOS/Linux packages + `MD-Notes-0.2.1-START.zip` + legal notices + `SHA256SUMS.txt` published.

## Відомі обмеження

- runtime GUI-перевірка v0.2.1 виявила blocker із зависанням; hotfix інтегровано, але ще не підтверджено реальною Windows-перевіркою;
- великі сховища все ще індексуються синхронно під час ручного відкриття;
- форма властивостей може нормалізувати форматування YAML front matter; YAML-коментарі всередині front matter не гарантуються після застосування форми;
- окремого GUI-редактора самого `schema.json` поки немає;
- збірки не підписані;
- Android/iOS ще не збираються.
