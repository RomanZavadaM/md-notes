# WORKLOG — MD Notes

Оновлено: **02.10.2026**

## STATUS

**ACTIVE. Checkpoint v0.2.1 — DONE; `main` випереджає опублікований checkpoint.** Runtime hotfix, Windows portable packaging і schema/property form уже інтегровані. Наступний функціональний slice v0.2: **створення vault із пресетом PARA / Zettelkasten / порожнє**.

## Нещодавно завершено

### Runtime startup hotfix
- користувач на реальній Windows-збірці v0.2.1 підтвердив сильне зависання після запуску;
- аудит показав небезпечне автоматичне відкриття `mdnotes.lastVault` перед поверненням контролю UI;
- PR #21 блокує читання цього ключа через `storage.get`, залишаючи шлях write-only для майбутньої явної дії «відкрити недавнє»;
- CI #92 — **PASS**;
- merge: `de7b9d1b8fa7b9f5c7884b4082d977f4a1029b01`.

**Не закрито доказом:** hotfix ще треба перевірити реальною новою Windows-збіркою. Якщо після ручного вибору vault зависання повториться — оптимізувати `note_paths()` / `get_tree()` і винести дорогий index sync із критичного open path.

### Windows portable
- PR #20 додає стандартний release artifact `MD-Notes-<version>-Windows-x64-portable.zip` поряд із `.exe` та `.msi`;
- portable містить `MD Notes.exe`, README та legal notices;
- виправлено пошук executable у workspace `target/release` із fallback;
- CI #95 — **PASS**;
- merge: `715e29dfcf3bace2480e6b2bd824cfc80c91a668`.

### Note types / schema / property form
- PR #19: `feat: add schema-driven note properties`;
- використано наявний відкритий `.mdnotes/schema.json` v1 без міграції формату;
- глобальні `fields`, `types.<name>.fields`, `required`, `template`;
- field types: text/string/number/boolean/date/enum/list/link/links/url/file;
- підтримка readonly/values/noteType;
- невідомі YAML-поля і невідомі note types зберігаються;
- invalid YAML не переписується формою;
- formatter працює з поточним editor content, тому незбережене тіло нотатки не губиться;
- thin Tauri bridge + typed frontend API;
- schema-driven «Властивості» в CodeMirror;
- UI UK / EN / FR / DE / ES / KO / JA;
- `docs/SCHEMA.md` + unit-тести;
- попередній CI #91 — PASS; після синхронізації з hotfix + portable clean CI #97 — **PASS**;
- merge: `7441259a019eae1f7858898d8a29a9ac4d9ec7cd`.

Runtime GUI-перевірка schema/property form у поточному середовищі не виконувалась.

## Поточний slice

**Ще не розпочато в коді.** Наступний незавершений пункт roadmap v0.2: **створення сховища з пресетом**.

Очікуваний scope:
- `empty` — мінімальне відкрите сховище без нав'язаної структури;
- `PARA` — Projects / Areas / Resources / Archives;
- `Zettelkasten` — мінімальна структура і стартові шаблони без vendor lock-in;
- усі пресети створюють тільки звичайні папки, Markdown, JSON та `.mdnotes` service metadata;
- не змінювати існуючий vault без явної дії користувача;
- UI та документація — 7 мов;
- логіка створення структури в `notes-core`, Tauri thin wrapper, tests.

## Наступна дія

1. перевірити існуючий flow відкриття/ініціалізації vault і `Vault::init`;
2. описати точний склад трьох preset-ів і безпечну поведінку при непорожній папці;
3. реалізувати через нову branch + PR;
4. пройти стандартний CI і license gates;
5. після зеленого CI інтегрувати slice технічно в `main`;
6. після preset slice наступний незавершений пункт v0.2 — **граф знань**;
7. **не створювати release checkpoint без окремої команди власника «зливай у main»**.

## Останній опублікований checkpoint

- GitHub prerelease [`v0.2.1`](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.2.1), 01.10.2026;
- release commit `26661d8c088df51ce162018d2a3b525502cf30e1`;
- release workflow #19 / run `36924302514` — **SUCCESS**;
- Windows/macOS/Linux packages + `MD-Notes-0.2.1-START.zip` + legal notices + `SHA256SUMS.txt` published.

## Відомі обмеження

- runtime freeze hotfix інтегровано, але ще не підтверджено реальною новою Windows-збіркою;
- великі сховища все ще індексуються синхронно при ручному відкритті;
- форма властивостей може нормалізувати YAML formatting; YAML-коментарі всередині front matter не гарантуються після застосування форми;
- окремого GUI-редактора `schema.json` поки немає;
- збірки не підписані;
- Android/iOS ще не збираються.
