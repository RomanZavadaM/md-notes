# START_HERE — MD Notes

> Перша точка входу для нового чату або відновлення після обриву.

## Статус

**ACTIVE — v0.3.0 опублікований як prerelease; продовжуємо roadmap stage v0.3 після завершеного Git/security checkpoint.**

Останній опублікований checkpoint: **v0.3.0** prerelease (03.10.2026), release commit `42656f0075d0072e295bee33c74f059c3ae6e656`.

v0.3.0 опублікований з Windows/macOS/Linux release assets, START/source, legal files і checksums. Android та iOS пройшли CI build smoke; physical-device runtime ще не підтверджений.

Roadmap stage **v0.3** продовжується після релізного checkpoint-а.

## Інтегрований v0.3 baseline

У `main` уже інтегровано:

- PR #30 — `StorageProvider` + `LocalFsProvider`;
- PR #31 — provider-backed `VaultStorage`;
- PR #32 — mobile sandbox vault + mobile startup flow;
- PR #33 — local-first sync decision/conflict foundation;
- PR #35 — Android/iOS mobile build smoke;
- PR #37 — persistent `.mdnotes/cache/sync-state.json`;
- PR #39 — `gix`/gitoxide HTTPS clone/open foundation;
- PR #41 — local Git dirty-state detection;
- PR #42 — local Git commit pipeline;
- PR #43 — safe public HTTPS fetch;
- PR #44 — in-memory Git HTTPS authentication.

PR #44 merge checkpoint: `b142973ea4315987b28aa3a4e6563506c219c255`; його PR head `fe5cdd66d416bc80e7e5dcc25e6d969ae917492f` пройшов desktop CI #211 і Mobile smoke #29.

## Завершений release checkpoint — v0.3.0

PR #45 `feat: add native system secret storage` інтегровано у `main` merge commit `70d81622574b365334046520f534a9c3be8784af`.

Фінальний PR head `78e159c02691973cb9caa7468e40d3d1ea982cc5` пройшов:
- CI #235 — SUCCESS;
- Android debug build — SUCCESS;
- iOS simulator debug build — SUCCESS.

Release PR #40 `chore(main): release 0.3.0` інтегровано у `main` merge commit `42656f0075d0072e295bee33c74f059c3ae6e656`.

GitHub prerelease `v0.3.0` опублікований. Release assets включають desktop packages, `MD-Notes-0.3.0-START.zip`, legal files та checksums.

## Після checkpoint-а

Завершений напрям цього етапу: provider/mobile foundation → local sync model → Git local state/commit → safe fetch → in-memory auth → system secret-store boundary.

Наступні незавершені sync slices, коли власник повернеться до MD Notes:

1. Git pull/merge policy без silent overwrite;
2. Git push;
3. інтеграція Git state з `SyncManifest` / ADR-0005 conflict policy;
4. WebDAV;
5. physical-device runtime validation Android/iOS;
6. optional Android SAF та iOS security-scoped external-folder adapters.

Не заявляти, що push/WebDAV/device runtime уже готові.

## Startup protocol

1. Прочитати `PROJECT_RULES.md`.
2. Прочитати `PROJECT_STATE.md`.
3. Прочитати `WORKLOG.md`.
4. Перевірити фактичний GitHub: `main` SHA, open PR, останні workflow runs і latest release.
5. Прочитати останні записи GitHub Issue #8.
6. Якщо GitHub і текст суперечать одне одному — GitHub має пріоритет, документацію синхронізувати.
7. Продовжити з першої незавершеної дії `WORKLOG.md`; не починати новий roadmap stage без рішення власника.

## Джерела істини

- `PROJECT_RULES.md` — постійні правила;
- `PROJECT_STATE.md` — підтверджений інтегрований стан;
- `WORKLOG.md` — operational queue і blockers;
- GitHub Issue #8 — append-only development ledger;
- `Cargo.toml`, `app/package.json`, `app/src-tauri/tauri.conf.json` — machine source версії;
- `docs/releases/` — опубліковані release notes;
- `docs/roadmap.md` — roadmap.

## Команда власника «злити у main»

**«Злити у main / зливай у main»** означає повний test-release checkpoint: нова версія, зелені checks, merge, збірки всіх підтримуваних платформ + START/source, checksums/legal, tag + GitHub prerelease, документація та ledger. Для простого merge використовується «інтегрувати PR у main».

## Для нового чату

> **Продовжуємо MD Notes. Відкрий у GitHub `START_HERE.md` і продовжуй строго за ним.**
