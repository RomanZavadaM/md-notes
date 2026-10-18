# START_HERE — MD Notes

> Перша точка входу для нового чату або відновлення після обриву.

## Статус

**ACTIVE — v0.3.1 опублікований як prerelease; roadmap stage v0.3 продовжується.**

Останній повний test-release checkpoint: **v0.3.1**, 18.10.2026.

- release commit: `95d95264edf6382c7eb812a0d2559f4c21455b76`;
- Release workflow #57 — **SUCCESS**;
- main CI #310 — **SUCCESS**;
- release assets: Windows installer/MSI + portable ZIP, macOS DMG/app archive, Linux AppImage/deb/rpm, START/source, legal notices, `SHA256SUMS.txt`;
- release status: **prerelease**;
- README синхронізовані мовами UK / EN / FR / DE / ES / KO / JA.

## Що інтегровано у v0.3.1

### Standalone Markdown

- PR #54 — standalone `.md/.markdown` open/edit/save без vault;
- PR #56 — macOS Finder/open-event + explicit Markdown handler action;
- Windows/Linux/macOS можуть працювати з одиночним Markdown-файлом;
- системна асоціація виконується лише після явної команди користувача;
- portable та installer не перехоплюють `.md` автоматично.

### Git pull safety

- PR #59 — repository pull planner;
- PR #60 — safety preflight;
- PR #61 — safe fast-forward apply;
- planner класифікує `UpToDate / FastForward / LocalAhead / Diverged / Unrelated`;
- dirty worktree блокує apply;
- автоматичний apply дозволений тільки для clean `FastForward`;
- немає blind `hard reset`, shell git, merge commit або silent overwrite;
- branch ref оновлюється з optimistic old-HEAD constraint;
- при race виконується спроба відновлення попереднього tree у worktree/index.

PR #61:
- final head: `8bac0f284dfbe2fab21ad8077b3b9d40868f6403`;
- CI #303 — **SUCCESS**;
- Mobile smoke #99 — **SUCCESS**;
- merge commit: `ed924059fa68bc31014c5b484fb78daaeba675b0`.

Release PR #52:
- final head: `909ce415e06d13a7e3556223aaf659d9ba2be88f`;
- CI #309 — **SUCCESS**;
- Mobile smoke #104 — **SUCCESS**;
- merge/release commit: `95d95264edf6382c7eb812a0d2559f4c21455b76`.

## Що ще не завершено

1. **Git push** — наступний головний slice.
2. Git state ↔ `SyncManifest` / ADR-0005 conflict policy.
3. WebDAV.
4. Physical Android/iOS runtime validation.
5. Optional Android SAF / iOS security-scoped external folders.

Не заявляти, що push/WebDAV/physical-device runtime вже готові.

## Startup protocol

1. Прочитати `PROJECT_RULES.md`.
2. Прочитати `PROJECT_STATE.md`.
3. Прочитати `WORKLOG.md`.
4. Перевірити фактичний GitHub: `main` SHA, open PR, останні workflow runs і latest release.
5. Прочитати останні записи GitHub Issue #8.
6. Якщо GitHub і документація суперечать одне одному — GitHub має пріоритет, документацію синхронізувати.
7. Продовжити з першої незавершеної дії `WORKLOG.md`; не повторювати вже інтегровану роботу.

## Джерела істини

- `PROJECT_RULES.md` — постійні правила;
- `PROJECT_STATE.md` — інтегрований стан `main`;
- `WORKLOG.md` — operational queue і точна наступна дія;
- GitHub Issue #8 — append-only development ledger;
- `Cargo.toml`, `app/package.json`, `app/src-tauri/tauri.conf.json` — machine source версії;
- `docs/releases/` — release notes;
- `docs/roadmap.md` — roadmap.

## Команда власника «злити у main»

**«Злити у main / зливай у main»** означає повний test-release checkpoint: нова версія, зелені checks, merge, збірки всіх підтримуваних платформ, START/source, checksums/legal, tag + GitHub prerelease, документація та ledger.

## Для нового чату

> **Продовжуємо MD Notes. Відкрий у GitHub `START_HERE.md` і продовжуй строго за ним.**
