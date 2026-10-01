# START_HERE — MD Notes

> Перша точка входу для нового чату або відновлення після обриву.

## Статус

**ACTIVE — йде розробка v0.2 «структура і зв'язки».**

Поточний опублікований checkpoint: **v0.1.0** (GitHub Release `v0.1.0`, 01.10.2026).
Деталі стану — `PROJECT_STATE.md`, активна робота — `WORKLOG.md`.

Не відновлювати старі work/feature branches як джерело коду і не повторювати merged slices.

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
- `PROJECT_STATE.md` — підтверджений стан продукту.
- `WORKLOG.md` — активний slice і наступна дія.
- GitHub Issue #8 — append-only development ledger.
- `Cargo.toml`, `app/package.json`, `app/src-tauri/tauri.conf.json` — machine source версії.
- `docs/releases/` — нотатки опублікованих релізів.
- `docs/roadmap.md` — план етапів.

## Команда власника «злити у main»

**«Злити у main / зливай у main»** означає повний test-release checkpoint: нова версія, зелені checks, merge, збірки всіх підтримуваних платформ + START/source, checksums/legal, tag + GitHub prerelease, синхронізація документації та ledger. Для простого merge — «інтегрувати PR у main». Див. `PROJECT_RULES.md`, розділ 11.

## Для нового чату

Достатньо фрази:

> **Продовжуємо MD Notes. Відкрий у GitHub `START_HERE.md` і продовжуй строго за ним.**
