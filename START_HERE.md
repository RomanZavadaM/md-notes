# START_HERE — MD Notes

> Перша точка входу для нового чату або відновлення після обриву.

## Статус

**ACTIVE — функціональний scope v0.2 завершений у `main`; перед наступним checkpoint потрібен Windows runtime evidence.**

Останній опублікований checkpoint: **v0.2.1** (GitHub prerelease `v0.2.1`, 01.10.2026).
`main` уже випереджає цей release: інтегровані runtime-startup hotfix, Windows portable packaging, schema-driven properties, safe vault presets/startup gate і global/local knowledge graph.

Поточна перша дія: **реальний Windows runtime-повторний тест актуального `main`**, особливо startup freeze hotfix. Green CI/build не прирівнюється до runtime validation.

Деталі стану — `PROJECT_STATE.md`, точний чек-лист — `WORKLOG.md`.

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
- `PROJECT_STATE.md` — підтверджений інтегрований стан продукту.
- `WORKLOG.md` — активний operational state, blockers і наступна дія.
- GitHub Issue #8 — append-only development ledger.
- `Cargo.toml`, `app/package.json`, `app/src-tauri/tauri.conf.json` — machine source версії.
- `docs/releases/` — нотатки опублікованих релізів.
- `docs/roadmap.md` — план етапів.

## Команда власника «злити у main»

**«Злити у main / зливай у main»** означає повний test-release checkpoint: нова версія, зелені checks, merge, збірки всіх підтримуваних платформ + START/source, checksums/legal, tag + GitHub prerelease, синхронізація документації та ledger. Для простого merge — «інтегрувати PR у main».

Функціональне завершення v0.2 саме по собі **не створює release**. До наступного release checkpoint треба закрити Windows runtime evidence gate або явно зафіксувати відоме обмеження за рішенням власника.

## Для нового чату

Достатньо фрази:

> **Продовжуємо MD Notes. Відкрий у GitHub `START_HERE.md` і продовжуй строго за ним.**
