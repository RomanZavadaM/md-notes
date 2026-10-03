# START_HERE — MD Notes

> Перша точка входу для нового чату або відновлення після обриву.

## Статус

**ACTIVE — v0.2.2 лишається останнім опублікованим prerelease; roadmap stage v0.3 активно розробляється.**

Останній опублікований checkpoint: **v0.2.2** (GitHub prerelease `v0.2.2`, 02.10.2026).
Release commit: `b422b497b4e3980c665f7ae7017f34e73788218d`.
Release workflow #30 / `36990461869` — **SUCCESS**.

03.10.2026 власник повторно протестував v0.2.2 на реальній Windows і підтвердив: **працює нормально**. Попередній startup-freeze blocker закритий для v0.2.2.

Власник погодив перехід до roadmap stage **v0.3**. Уже інтегровано в `main`:
- PR #30 — `StorageProvider` + `LocalFsProvider`, merge `4545c938fcb70c80caa9e7595b4d89e8eead27aa`, CI #147 PASS;
- PR #31 — provider-backed `VaultStorage` і делегування provider-neutral Vault I/O, merge `230ee8e4373fe909c178c44865c5b5cd9d7bcd3e`, CI #153 PASS;
- PR #32 — default mobile sandbox vault + mobile startup flow 7 мовами, merge `588219e2310e94f2fbd04a3ba97288b8ba94813f`, CI #161 PASS;
- PR #33 — local-first sync decision/conflict foundation за ADR-0005, merge `e91734df8f37d2be1a776fcad7883bbd30160b01`, CI #162 PASS.

Поточна перша дія: **mobile build validation PR #35**, який перевіряє Android debug build та iOS simulator debug build. Це build evidence, не runtime evidence на фізичному пристрої.

Після clean mobile-build CI наступний функціональний напрям — **Git sync (`gix`/gitoxide)** поверх уже інтегрованого sync foundation, потім WebDAV. Optional external-folder adapters (Android SAF та iOS security-scoped bookmarks) — після стабільного sandbox + sync згідно ADR-0006.

Деталі стану — `PROJECT_STATE.md`, operational queue — `WORKLOG.md`.

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

У межах поточного roadmap stage звичайні `feat:` checkpoint-и **не повинні автоматично переводити minor-версію**. Наприклад, `0.3.0 → 0.3.1`. Перехід `0.2 → 0.3` означає зміну roadmap stage і має бути свідомим рішенням власника. Такий перехід власником уже погоджений 03.10.2026.

Застарілий release-please PR #29 на `0.2.3` закрито без merge, оскільки він суперечив погодженому переходу до v0.3.

## Команда власника «злити у main»

**«Злити у main / зливай у main»** означає повний test-release checkpoint: нова версія, зелені checks, merge, збірки всіх підтримуваних платформ + START/source, checksums/legal, tag + GitHub prerelease, синхронізація документації та ledger. Для простого merge — «інтегрувати PR у main».

## Для нового чату

Достатньо фрази:

> **Продовжуємо MD Notes. Відкрий у GitHub `START_HERE.md` і продовжуй строго за ним.**
