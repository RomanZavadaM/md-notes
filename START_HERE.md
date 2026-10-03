# START_HERE — MD Notes

> Перша точка входу для нового чату або відновлення після обриву.

## Статус

**ACTIVE — v0.2.2 лишається останнім опублікованим prerelease; roadmap stage v0.3 активно розробляється.**

Останній опублікований checkpoint: **v0.2.2** (02.10.2026), release commit `b422b497b4e3980c665f7ae7017f34e73788218d`, release workflow `36990461869` — **SUCCESS**.

03.10.2026 власник повторно протестував v0.2.2 на реальній Windows і підтвердив нормальну роботу; startup-freeze blocker v0.2.1 закритий.

Власник погодив roadmap stage **v0.3**.

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

## Поточний checkpoint — PR #45

PR #45 `feat: add native system secret storage` завершує security boundary для Git HTTPS credentials у Tauri layer:

- Windows Credential Manager;
- macOS Keychain;
- iOS Protected Data;
- Android Keystore-backed storage;
- Linux Secret Service;
- target-specific native backend dependencies;
- token не повертається frontend-у;
- Tauri API дозволяє save / has / clear credentials;
- authenticated fetch завантажує secret лише всередині Rust безпосередньо перед network call;
- vault/session lock відпускається до credential/network роботи;
- `Cargo.lock` згенеровано Cargo;
- legal notices оновлено;
- README синхронізовано UK/EN/FR/DE/ES/KO/JA.

**PR #45 не вважати інтегрованим, доки його фінальний desktop CI та Android/iOS Mobile smoke не зелені й PR не merged.**

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
