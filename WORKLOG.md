# WORKLOG — MD Notes

Оновлено: **02.10.2026**

## STATUS

**ACTIVE. Checkpoint v0.2.1 — DONE; `main` випереджає опублікований checkpoint.** Runtime hotfix, Windows portable packaging, schema/property form і vault presets уже інтегровані. Поточний наступний slice v0.2: **граф знань — глобальний і локальний**.

## Нещодавно завершено

### Runtime startup hotfix
- користувач на реальній Windows-збірці v0.2.1 підтвердив сильне зависання після запуску;
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
- schema-driven форма «Властивості», thin Tauri bridge, typed frontend API, UI 7 мовами, `docs/SCHEMA.md` і unit-тести;
- clean CI #97 — **PASS**;
- merge: `7441259a019eae1f7858898d8a29a9ac4d9ec7cd`.

### Vault presets / startup gate
- PR #23: `feat: add safe vault presets`;
- Empty, PARA, Zettelkasten у `notes-core`;
- preset creation дозволено лише в порожній папці; refusal відбувається до створення `.mdnotes` і не змінює existing files;
- startup gate: явні дії «відкрити останнє / відкрити існуюче / створити нове»;
- `lastVault` читається тільки через одноразовий explicit-open marker після дії користувача;
- optional vault name, системний folder picker, UI 7 мовами, scroll-friendly layout;
- `docs/VAULT_PRESETS.md` + unit-тести;
- перший CI #103 знайшов лише rustfmt; виправлено;
- clean CI #114 — **PASS**: Windows/macOS/Linux core, frontend/Tauri, dependency licenses, PR title;
- merge: `c078d075d089257c83b903eea44c317f391b765c`.

Runtime GUI-перевірка startup/preset flow у поточному середовищі **не виконувалась**.

## Поточний slice — knowledge graph

**Ще не розпочато в коді.** Це останній незавершений функціональний пункт roadmap v0.2.

Очікуваний scope:
- глобальний граф усіх нотаток і внутрішніх зв'язків vault;
- локальний граф для активної нотатки: сама нотатка, прямі вихідні та зворотні зв'язки;
- клік по вузлу відкриває відповідну нотатку;
- граф не створює окремий закритий формат даних — джерело істини лишається Markdown + індекс;
- використати вже наявні index/link дані через `notes-core` / thin Tauri bridge;
- UI локалізувати UK / EN / FR / DE / ES / KO / JA;
- на вузьких екранах граф має бути доступний без прихованих кнопок і без горизонтального блокування;
- нова dependency допускається лише після license gate і оновлення `THIRD_PARTY_NOTICES.md`.

## Наступна дія

1. перевірити `app/package.json`, current index APIs і наявність/відсутність sigma.js / graphology;
2. спроєктувати мінімальний `GraphNode` / `GraphEdge` API в `notes-core` або Tauri без дублювання даних;
3. створити окрему branch + draft PR;
4. реалізувати global/local graph і навігацію по вузлах;
5. пройти стандартний CI + license gates;
6. після green checks технічно інтегрувати graph slice в `main`;
7. після graph slice етап v0.2 функціонально завершений; release checkpoint робити лише за окремою командою власника **«зливай у main»**.

## Останній опублікований checkpoint

- GitHub prerelease [`v0.2.1`](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.2.1), 01.10.2026;
- release commit `26661d8c088df51ce162018d2a3b525502cf30e1`;
- release workflow #19 / run `36924302514` — **SUCCESS**;
- Windows/macOS/Linux packages + `MD-Notes-0.2.1-START.zip` + legal notices + `SHA256SUMS.txt` published.

## Відомі обмеження

- runtime freeze hotfix інтегровано, але ще не підтверджено реальною новою Windows-збіркою;
- великі сховища все ще індексуються синхронно при ручному відкритті;
- startup/preset UI ще не перевірений вручну в реальному Tauri runtime;
- schema/property form також не пройшов manual runtime validation у поточному середовищі;
- збірки не підписані;
- Android/iOS ще не збираються.
