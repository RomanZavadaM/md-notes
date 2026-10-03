# WORKLOG — MD Notes

Оновлено: **03.10.2026**

## STATUS

**ACTIVE. v0.2.2 опублікований і підтверджений ручним Windows runtime-тестом. Roadmap stage v0.3 у роботі.**

Release baseline:
- tag/prerelease `v0.2.2`;
- release commit `b422b497b4e3980c665f7ae7017f34e73788218d`;
- Windows/macOS/Linux packages + START/source + legal + `SHA256SUMS.txt` опубліковані;
- release-please тепер правильно веде майбутній stage release як `0.3.0` у PR #40; **не merge до окремого full test-release checkpoint**.

## Runtime / build evidence

- 03.10.2026 власник повторно перевірив v0.2.2 на реальній Windows: **працює нормально**; blocker startup-freeze v0.2.1 закритий.
- Mobile smoke інтегрований у `main`: Android aarch64 debug APK та iOS arm64-simulator debug build збираються в CI.
- Це build evidence, а не device runtime evidence.

## Завершено й інтегровано у v0.3

### PR #30 — StorageProvider foundation
- merge `4545c938fcb70c80caa9e7595b4d89e8eead27aa`;
- `StorageProvider`, `LocalFsProvider`, atomic writes, path safety, provider operations/tests.

### PR #31 — provider-backed Vault I/O
- merge `230ee8e4373fe909c178c44865c5b5cd9d7bcd3e`;
- `VaultStorage`, provider-backed tree/config/read/write/unique paths;
- existing `Vault` delegates provider-neutral I/O.

### PR #32 — mobile sandbox bootstrap
- merge `588219e2310e94f2fbd04a3ba97288b8ba94813f`;
- app-data sandbox vault for Android/iOS;
- mobile startup path without desktop folder picker;
- UK/EN/FR/DE/ES/KO/JA localization.

### PR #33 — local-first sync decision foundation
- merge `e91734df8f37d2be1a776fcad7883bbd30160b01`;
- upload/download/delete planner;
- modification wins over concurrent deletion;
- divergent change => merge/conflict;
- conflict-copy path + unit tests.

### PR #35 — mobile build validation
- merge `801a706fe1439c17822b23c623e142a68c8a3d5a`;
- Android + iOS Tauri init/build smoke added to CI.

### PR #37 — persistent local sync state
- merge `952a5f2f6e9c8891db3c03ff530d26d9c4d1ae45`;
- atomic `.mdnotes/cache/sync-state.json`;
- version guard, empty fallback, corruption tests.

### PR #39 — `gix` Git clone/open foundation
- merge `0aa96720533bc4823c840e5debb66f8f11830894`;
- `gix` 0.88 with minimal HTTPS/rustls/worktree feature set;
- HTTPS-only remote validation;
- credentials/query/fragment/whitespace rejected in remote URL;
- isolated repository config: no implicit system/user Git credential helpers;
- public HTTPS clone + checkout; open existing repo + HEAD/worktree info;
- Cargo-generated lock + legal notice;
- PR validation: CI #188 SUCCESS + Mobile smoke #10 SUCCESS.

## Активний slice — PR #41: local Git dirty-state

Branch: `feature/git-local-commit-v0.3`

Purpose:
- enable minimal `gix` status component;
- add `git_has_changes()` for tracked/untracked worktree changes;
- keep isolated config boundary;
- no-network unit test on an initialized local repository;
- keep `Cargo.lock` current.

Current final code head after removing the temporary lock writer: `161109d8ac8b036e417281d1d3855930280ad933`.

Current gates:
- CI #193 running; Ubuntu/macOS core and licenses already green at last check, Windows/app finishing;
- Mobile smoke #14 running for Android/iOS;
- do not merge until both workflows are fully green.

## Exact next actions

1. Finish CI #193 + Mobile smoke #14 for PR #41; fix only real failures, then integrate PR #41 into `main`.
2. Local commit slice:
   - enumerate worktree changes with `gix status`;
   - apply changed/deleted files to a tree via `Repository::edit_tree()` + `write_blob()`;
   - create commit with explicit application identity (do not depend on user Git config);
   - rebuild/write index from the committed tree so repository returns clean after commit;
   - tests for initial commit, modification, deletion and no-op commit.
3. Network sync slice:
   - explicit in-memory credential callback only;
   - token/password source must be system secret store at app layer;
   - fetch/pull first, then push;
   - never persist secrets in vault, Git remote URL, logs, `localStorage` or Git config.
4. Integrate Git state with `SyncManifest` / ADR-0005 conflict policy.
5. WebDAV slice via current OpenDAL WebDAV service.
6. After stable sandbox + sync, optional external-folder adapters: Android SAF and iOS security-scoped bookmarks.

## Відомі обмеження

- Android/iOS runtime on real devices is still untested;
- Git auth/fetch/pull/commit/push are not yet integrated into the app;
- WebDAV adapter not yet implemented;
- system secret-store UI/backend not yet implemented;
- external SAF/bookmark adapters not yet implemented;
- large vaults can still index synchronously on open;
- graph slice still lacks ForceAtlas2/clustering/layout persistence;
- builds are unsigned.
