# WORKLOG — MD Notes

Оновлено: **02.10.2026**

## STATUS

**ACTIVE. Checkpoint v0.2.1 — DONE.** Поточний пріоритет: **hotfix runtime-зависання звичайної Windows-збірки**. Функціональний slice schema/property form тимчасово не втрачається, але не має пріоритету над runtime blocker.

## Runtime blocker 02.10.2026

Користувач підтвердив на реальній Windows-збірці: застосунок встановлюється і запускається, але після запуску комп'ютер може сильно зависати; симптом видно навіть при спробі відкрити «Про програму».

Аудит показав небезпечний startup path:

- frontend автоматично відкривав `mdnotes.lastVault` одразу після старту;
- backend `open_vault` синхронно індексує весь vault;
- `Index::sync()` отримує `vault.note_paths()`, а поточний `note_paths()` будує повне рекурсивне дерево;
- після `open_vault` frontend окремо викликає `get_tree()`, тобто велика папка може бути рекурсивно просканована ще раз;
- будь-яка звичайна папка може бути обрана як vault, тому старий `lastVault` міг автоматично запускати дорогий обхід до того, як користувач отримає контроль над UI.

Hotfix branch: `fix/runtime-startup-freeze`.

Перший захисний крок уже реалізовано:

- `mdnotes.lastVault` більше не читається автоматично під час startup;
- шлях останнього vault продовжує записуватися для майбутньої явної дії «відкрити недавнє», але старт застосунку має залишатися idle, доки користувач сам не вибере vault.

Це навмисно змінює convenience-поведінку заради безпеки запуску. Runtime GUI-перевірка у поточному середовищі ще не виконана; підтвердження має бути на реальній Windows-машині.

## Паралельні PR

- PR #19 — `feat: add schema-driven note properties`; CI доопрацьовується окремо, не merge до завершення blocker-а;
- PR #20 — `build: add Windows portable release package`; portable додається як стандартний release artifact і не вважається виправленням runtime-зависання.

## Наступна дія

1. прогнати CI hotfix PR;
2. інтегрувати hotfix у `main` після green checks;
3. дати нову Windows-збірку для реального runtime-тесту;
4. якщо зависання повториться після відключення auto-reopen — окремо оптимізувати `note_paths()` / `get_tree()` і винести дорогий index sync із startup path;
5. після підтвердження runtime повернутися до PR #19 і PR #20.

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

## Відомі обмеження

- runtime GUI-перевірка v0.2.1 вручну виявила blocker із зависанням; автоматичні CI/build checks цього не виявили;
- великі сховища індексуються синхронно під час відкриття;
- збірки не підписані;
- Android/iOS ще не збираються.
