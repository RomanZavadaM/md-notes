# WORKLOG — MD Notes

Оновлено: **17.10.2026**

## STATUS

**ACTIVE. v0.3.0 опублікований як prerelease. Git/security checkpoint завершений; наступна робота v0.3 — pull/merge policy без silent overwrite.**

Release baseline:
- tag/prerelease `v0.3.0`;
- release commit `42656f0075d0072e295bee33c74f059c3ae6e656`;
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
- PR #47 — safe Git pull relation planner.
- PR #54 — standalone Markdown reader/editor + opt-in Windows/Linux file association.

PR #44 merge checkpoint: `b142973ea4315987b28aa3a4e6563506c219c255`.
Validation on PR #44 head `fe5cdd66d416bc80e7e5dcc25e6d969ae917492f`: CI #211 SUCCESS + Mobile smoke #29 SUCCESS.

## Завершений Git/security checkpoint

- PR #45 інтегрований у `main`: `70d81622574b365334046520f534a9c3be8784af`.
- Final PR head: `78e159c02691973cb9caa7468e40d3d1ea982cc5`.
- CI #235: SUCCESS.
- Android debug build: SUCCESS.
- iOS simulator debug build: SUCCESS.
- Release PR #40 інтегрований у `main`: `42656f0075d0072e295bee33c74f059c3ae6e656`.
- GitHub prerelease `v0.3.0`: published.

## Standalone Markdown checkpoint

- PR #54 merged: `c940dcc7aa2f8a4028b7ba4a8a15862070b7907b`.
- Final head `937047ada68cf38626a0cb35bd75457c2efab700`.
- CI #270: SUCCESS.
- Mobile smoke #78: SUCCESS.
- `.md/.markdown` можна відкрити зсередини MD Notes без vault і зберегти назад.
- Windows/Linux: системна прив'язка Markdown виконується тільки після явної команди користувача; portable та installer поводяться однаково з точки зору opt-in.
- PR #56 closes macOS standalone Markdown integration: Finder open-event + explicit handler action, without automatic default takeover. Desktop CI + Android are green; final iOS rerun is pending before merge.

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
