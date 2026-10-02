# WORKLOG — MD Notes

Оновлено: **02.10.2026**

## STATUS

**ACTIVE. Checkpoint v0.2.1 — DONE; `main` випереджає опублікований checkpoint.** Runtime hotfix, Windows portable packaging і schema/property form уже інтегровані. Поточний slice v0.2: **створення vault із пресетом PARA / Zettelkasten / порожнє**.

## Нещодавно завершено

### Runtime startup hotfix
- користувач на реальній Windows-збірці v0.2.1 підтвердив сильне зависання після запуску;
- аудит показав небезпечне автоматичне відкриття `mdnotes.lastVault` перед поверненням контролю UI;
- PR #21 заблокував неявне startup-відкриття останнього vault;
- CI #92 — **PASS**;
- merge: `de7b9d1b8fa7b9f5c7884b4082d977f4a1029b01`.

**Не закрито доказом:** hotfix ще треба перевірити реальною новою Windows-збіркою. Якщо після ручного вибору vault зависання повториться — оптимізувати `note_paths()` / `get_tree()` і винести дорогий index sync із критичного open path.

### Windows portable
- PR #20 додає стандартний release artifact `MD-Notes-<version>-Windows-x64-portable.zip` поряд із `.exe` та `.msi`;
- CI #95 — **PASS**;
- merge: `715e29dfcf3bace2480e6b2bd824cfc80c91a668`.

### Note types / schema / property form
- PR #19: `feat: add schema-driven note properties`;
- використано наявний відкритий `.mdnotes/schema.json` v1 без міграції формату;
- schema-driven форма «Властивості», thin Tauri bridge, typed frontend API, UI 7 мовами, `docs/SCHEMA.md` і unit-тести;
- clean CI #97 — **PASS**;
- merge: `7441259a019eae1f7858898d8a29a9ac4d9ec7cd`.

Runtime GUI-перевірка schema/property form у поточному середовищі не виконувалась.

## Поточний slice — vault presets

Branch: `feature/vault-presets-v0.2`.
Draft PR #23: `feat: add safe vault presets`.

Реалізовано у candidate:

- новий core-модуль presets у `notes-core`;
- `empty` — стандартна `.mdnotes` service metadata без нав'язаної видимої структури;
- `PARA` — `Projects / Areas / Resources / Archives`;
- `Zettelkasten` — `Notes / Sources / daily` + відкритий Markdown-шаблон `.mdnotes/templates/zettel.md`;
- preset creation дозволено лише у **порожній існуючій папці**;
- якщо папка непорожня, операція відмовляє **до створення `.mdnotes`** і не змінює наявні файли;
- unit-тести покривають Empty, PARA, Zettelkasten і refusal/non-modification safety case;
- Tauri `create_vault` + typed frontend `VaultPreset` API;
- startup-gate перед основним App: явні дії «відкрити останнє», «відкрити існуюче», «створити нове»;
- небезпечне автоматичне відкриття `lastVault` не повернуто: startup дозволяє його прочитати лише через одноразовий explicit-open marker після кліку користувача;
- створення нового vault: optional name + Empty/PARA/Zettelkasten + системний вибір порожньої папки;
- startup UI локалізовано UK / EN / FR / DE / ES / KO / JA;
- startup layout має вертикальну прокрутку і компактну картку для вузьких/малих екранів.

### Перевірка PR #23

Перший CI #103:
- frontend + Tauri — **PASS**;
- notes-core Windows — **PASS**;
- notes-core macOS — **PASS**;
- dependency licenses — **PASS**;
- Conventional PR title — **PASS**;
- Ubuntu зупинився лише на `cargo fmt --check`;
- rustfmt diff виправлено на цій самій гілці; функціональна помилка не виявлена.

Після UI/localization змін потрібен новий повний clean CI.
Runtime GUI-перевірка startup/preset flow у поточному середовищі **не виконувалась** і не вважається виконаною.

## Наступна дія

1. додати коротку документацію preset-ів і safety-умови;
2. прогнати clean CI після завершеного UI;
3. виправити всі Rust/TypeScript/API нестикування на цій самій гілці;
4. після green checks перевести PR #23 з draft у ready і технічно інтегрувати в `main`;
5. синхронізувати roadmap / PROJECT_STATE / Issue #8 після merge;
6. наступний незавершений пункт v0.2 — **граф знань**;
7. **не створювати release checkpoint без окремої команди власника «зливай у main»**.

## Останній опублікований checkpoint

- GitHub prerelease [`v0.2.1`](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.2.1), 01.10.2026;
- release commit `26661d8c088df51ce162018d2a3b525502cf30e1`;
- release workflow #19 / run `36924302514` — **SUCCESS**;
- Windows/macOS/Linux packages + `MD-Notes-0.2.1-START.zip` + legal notices + `SHA256SUMS.txt` published.

## Відомі обмеження

- runtime freeze hotfix інтегровано, але ще не підтверджено реальною новою Windows-збіркою;
- великі сховища все ще індексуються синхронно при ручному відкритті;
- startup/preset UI ще не перевірений вручну в реальному Tauri runtime;
- форма властивостей може нормалізувати YAML formatting; YAML-коментарі всередині front matter не гарантуються після застосування форми;
- окремого GUI-редактора `schema.json` поки немає;
- збірки не підписані;
- Android/iOS ще не збираються.
