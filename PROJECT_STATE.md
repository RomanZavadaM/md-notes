# PROJECT_STATE — MD Notes

Оновлено: **18.10.2026**

## Поточний стан

- Статус розвитку: **ACTIVE**.
- Останній опублікований checkpoint: **v0.3.1 prerelease**, 18.10.2026.
- Release commit: `95d95264edf6382c7eb812a0d2559f4c21455b76`.
- Release workflow #57 — **SUCCESS**.
- Main CI #310 — **SUCCESS**.
- README languages: UK / EN / FR / DE / ES / KO / JA — синхронізовані під v0.3.1.
- UI languages: UK / EN / FR / DE / ES / KO / JA.
- Ліцензійна модель: proprietary / All Rights Reserved, Roman Zavada / Роман Завада.

## Release v0.3.1

Опубліковано:

- Windows: installer `.exe`, `.msi`, portable ZIP;
- macOS: universal `.dmg`, `.app.tar.gz`;
- Linux: `.AppImage`, `.deb`, `.rpm`;
- `MD-Notes-0.3.1-START.zip`;
- `LICENSE.md`, `COPYRIGHT.md`, `THIRD_PARTY_NOTICES.md`;
- `SHA256SUMS.txt`.

Builds не вважаються підписаними/нотаризованими, якщо конкретний release прямо не підтверджує інше.

## Інтегрований v0.3 baseline

### Storage / mobile

- PR #30 — provider-neutral `StorageProvider` + `LocalFsProvider`;
- PR #31 — provider-backed `VaultStorage`;
- PR #32 — mobile sandbox vault + mobile startup flow;
- PR #35 — Android aarch64 + iOS simulator build smoke.

### Sync foundation

- PR #33 — `SyncManifest`, snapshots, upload/download/delete planning, visible conflict-copy policy;
- PR #37 — persistent atomic `.mdnotes/cache/sync-state.json`.

### Git foundation

- PR #39 — `gix` HTTPS clone/open foundation;
- PR #41 — dirty worktree detection;
- PR #42 — local Git commit pipeline;
- PR #43 — safe public HTTPS fetch;
- PR #44 — in-memory HTTPS authentication;
- PR #45 — native system credential storage;
- PR #59 — repository pull planner;
- PR #60 — pull safety preflight;
- PR #61 — safe fast-forward apply.

Поточна safety policy:
- clean `FastForward` — може бути автоматично застосований;
- dirty worktree — не чіпати;
- `UpToDate`, `LocalAhead`, `Diverged`, `Unrelated` — не мутувати автоматично;
- blind `hard reset` і silent overwrite заборонені.

### Standalone Markdown

- PR #54 — одиночний `.md/.markdown` без vault: open/edit/save;
- PR #56 — macOS Finder open-event + explicit handler action;
- Windows/Linux/macOS desktop integration без автоматичного takeover файлової асоціації.

## Межа доказу / відомі обмеження

- Windows v0.2.2 runtime — підтверджено власником;
- Android/iOS buildability — підтверджено CI, physical-device runtime ще не підтверджений;
- зашифроване сховище (ADR-0007) ще не реалізоване;
- Git push ще не реалізований;
- Git sync ще не зв’язаний повністю з `SyncManifest` / ADR-0005 conflict resolution;
- WebDAV ще не реалізований;
- Android SAF / iOS security-scoped bookmarks ще не реалізовані;
- large vault open/indexing може лишатися синхронним;
- graph still lacks ForceAtlas2/clustering/layout persistence;
- release builds unsigned unless explicitly stated otherwise.

## Наступний напрям

1. **Зашифроване сховище (age, ADR-0007), зрізи 0-3.**
1a. **Git push** (одразу після зрізу 3).
2. Integration Git state ↔ `SyncManifest` / ADR-0005.
3. WebDAV.
4. Physical-device Android/iOS runtime validation.
5. Optional Android SAF / iOS security-scoped external folders.

Перед новими dependency обов’язково проходити license gates та оновлювати `THIRD_PARTY_NOTICES.md`.
