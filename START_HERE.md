# START_HERE — MD Notes

> Перша точка входу для нового чату або відновлення після обриву.

## Статус

**ACTIVE — checkpoint v0.2.2 опублікований; функціональний scope v0.2 завершений.**

Останній опублікований checkpoint: **v0.2.2** (GitHub prerelease `v0.2.2`, 02.10.2026).
Release commit: `b422b497b4e3980c665f7ae7017f34e73788218d`.
Release workflow #30 / `36990461869` — **SUCCESS**.

У v0.2.2 входять runtime-startup hotfix, Windows portable packaging, schema-driven properties, safe vault presets/startup gate і global/local knowledge graph.

Поточна перша дія: **реальний Windows runtime-тест v0.2.2**, особливо перевірка попереднього freeze blocker. Green CI/build не прирівнюється до runtime validation.

Якщо після ручного відкриття vault зависання повторюється — пріоритетно оптимізувати `note_paths()` / `get_tree()` і винести дорогий index sync із критичного open path. Якщо runtime стабільний — наступний roadmap stage v0.3: Android/iOS + Git/WebDAV sync.

Деталі стану — `PROJECT_STATE.md`, точний чек-лист — `WORKLOG.md`.

Не відновлювати старі work/feature/test branches як джерело коду і не повторювати merged slices.

## Startup protocol

1. Прочитати `PROJECT_RULES.md`.
2. Прочитати `PROJECT_STATE.md`.
3. Прочитати `WORKLOG.md`.
4. Перевірити фактичний GitHub: `main` SHA, відкриті PR, останні workflow runs і latest release.
5. Прочитати останні записи GitHub Issue **#8**.
6. Якщо GitHub і текст суперечать одне одному — GitHub має пріоритет, після чого документацію синхронізувати.
7. Продовжити з першої незавершеної дії `WORKLOG.md`. Нові етапи поза roadmap починати лише після рішення власника.

## Джерела істини

- `PROJECT_RULES.md` — постійні правила.
- `PROJECT_STATE.md` — підтверджений інтегрований стан продукту.
- `WORKLOG.md` — активний operational state, blockers і наступна дія.
- GitHub Issue #8 — append-only development ledger.
- `Cargo.toml`, `app/package.json`, `app/src-tauri/tauri.conf.json` — machine source версії.
- `docs/releases/` — нотатки опублікованих релізів.
- `docs/roadmap.md` — план етапів.

## Правило версій до 1.0

У межах поточного roadmap stage звичайні `feat:` checkpoint-и **не повинні автоматично переводити minor-версію**. Наприклад, `0.2.1 → 0.2.2`. Перехід `0.2 → 0.3` означає зміну roadmap stage і має бути свідомим рішенням власника.

## Команда власника «злити у main»

**«Злити у main / зливай у main»** означає повний test-release checkpoint: нова версія, зелені checks, merge, збірки всіх підтримуваних платформ + START/source, checksums/legal, tag + GitHub prerelease, синхронізація документації та ledger. Для простого merge — «інтегрувати PR у main».

## Для нового чату

Достатньо фрази:

> **Продовжуємо MD Notes. Відкрий у GitHub `START_HERE.md` і продовжуй строго за ним.**
