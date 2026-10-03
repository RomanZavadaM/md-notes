# WORKLOG — MD Notes

Оновлено: **03.10.2026**

## STATUS

**ACTIVE. v0.2.2 опублікований і підтверджений Windows runtime-тестом. Roadmap stage v0.3 у роботі. Поточний Git/security checkpoint оформлюється до завершеного стану перед переходом власника до іншої роботи.**

Release baseline:
- tag/prerelease `v0.2.2`;
- release commit `b422b497b4e3980c665f7ae7017f34e73788218d`;
- Windows/macOS/Linux packages + START/source + legal + checksums опубліковані.

## Runtime / build evidence

- Windows v0.2.2 runtime: PASS від власника 03.10.2026.
- Android/iOS build smoke інтегрований у `main` і використовується як обов’язковий gate для mobile/Tauri changes.
- Mobile smoke = build evidence, не physical-device runtime evidence.

## Завершено й інтегровано у v0.3

- PR #30 — `StorageProvider` foundation.
- PR #31 — provider-backed `VaultStorage`.
- PR #32 — mobile sandbox bootstrap.
- PR #33 — local-first sync decision/conflict foundation.
- PR #35 — Android/iOS build validation.
- PR #37 — persistent local sync state.
- PR #39 — `gix` HTTPS clone/open foundation.
- PR #41 — dirty worktree detection.
- PR #42 — local Git commit pipeline.
- PR #43 — safe public HTTPS fetch.
- PR #44 — in-memory Git HTTPS authentication.

PR #44 merge checkpoint: `b142973ea4315987b28aa3a4e6563506c219c255`.
Validation on PR #44 head `fe5cdd66d416bc80e7e5dcc25e6d969ae917492f`: CI #211 SUCCESS + Mobile smoke #29 SUCCESS.

## Активний фінальний slice цього checkpoint — PR #45

Branch: `feature/system-secret-store-v0.3`.

Purpose:
- system credential storage for Git HTTPS secrets;
- Windows Credential Manager;
- macOS Keychain;
- iOS Protected Data;
- Android Keystore-backed storage;
- Linux Secret Service;
- only target-specific native store dependency is linked;
- save / has / clear Tauri commands;
- no IPC command returns the token;
- authenticated fetch loads secret only inside Rust;
- session lock is released before network/credential work;
- Cargo-generated lockfile and updated legal notices.

Documentation checkpoint added on the same branch:
- `README.md` updated to v0.2.2 published baseline + active v0.3 state;
- localized README descriptions updated: EN / FR / DE / ES / KO / JA;
- `START_HERE.md`, `PROJECT_STATE.md`, `WORKLOG.md`, `CHANGELOG.md` synchronized;
- Issue #8 must receive the final append-only checkpoint after merge.

## Gate before considering this stage clean

1. PR #45 final ordinary CI = SUCCESS.
2. PR #45 final Mobile smoke Android + iOS = SUCCESS.
3. Mark ready and integrate PR #45 into `main`.
4. Add Issue #8 ledger entry with merge SHA + workflow evidence.
5. Verify `main` and leave no temporary write-capable workflow behind.

## Next work when MD Notes resumes

1. Git pull/merge policy with no silent overwrite.
2. Git push.
3. Integrate Git state with `SyncManifest` / ADR-0005 conflict policy.
4. WebDAV adapter.
5. Physical Android/iOS runtime validation.
6. Optional Android SAF / iOS security-scoped external folders.

## Відомі обмеження

- Android/iOS physical-device runtime untested;
- Git push not implemented;
- complete Git pull/merge/conflict workflow not implemented;
- WebDAV not implemented;
- SAF/bookmark external-folder adapters not implemented;
- large vault open/indexing can still be synchronous;
- graph still lacks ForceAtlas2/clustering/layout persistence;
- builds are unsigned.
