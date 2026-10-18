# WORKLOG — MD Notes

Оновлено: **18.10.2026**

## STATUS

**ACTIVE. Повний test-release checkpoint v0.3.1 завершено. Наступний активний slice — Git push.**

Release baseline:
- tag/prerelease: `v0.3.1`;
- release commit: `95d95264edf6382c7eb812a0d2559f4c21455b76`;
- Release workflow #57: **SUCCESS**;
- main CI #310: **SUCCESS**;
- Windows/macOS/Linux packages + portable + START/source + legal + checksums: **published**.

## Runtime / build evidence

- Windows v0.2.2 runtime: PASS від власника 03.10.2026.
- Android/iOS build smoke: CI evidence only, не physical-device runtime.
- PR #61 final head `8bac0f284dfbe2fab21ad8077b3b9d40868f6403`: CI #303 SUCCESS + Mobile smoke #99 SUCCESS.
- Release PR #52 final head `909ce415e06d13a7e3556223aaf659d9ba2be88f`: CI #309 SUCCESS + Mobile smoke #104 SUCCESS.

## Нещодавно завершено

### Standalone Markdown

- PR #54 merged: `c940dcc7aa2f8a4028b7ba4a8a15862070b7907b`.
- PR #56 merged: `a6429a40f87827e710dbd2e6ebb7b9e6df194375`.
- `.md/.markdown` open/edit/save без vault.
- macOS Finder open-event підтримується.
- Системна асоціація лише після explicit user action.

### Git pull planner / safety / apply

- PR #59 merge: `ad0444eee38d81236d1f60c20e6bfaaf73f7f6f3`.
- PR #60 merge: `d9d4d5fd3becf77d1587981733e6f442881ae6a9`.
- PR #61 merge: `ed924059fa68bc31014c5b484fb78daaeba675b0`.
- clean FastForward-only automatic apply;
- dirty worktree blocks mutation;
- LocalAhead / Diverged / Unrelated не мутуються;
- no blind hard reset / silent overwrite;
- optimistic ref update + rollback attempt on race.

### Release v0.3.1

- Release PR #52 merge/release commit: `95d95264edf6382c7eb812a0d2559f4c21455b76`.
- Tag/prerelease: `v0.3.1`.
- Release workflow #57: SUCCESS.
- Assets published:
  - Windows setup EXE;
  - Windows MSI;
  - Windows x64 portable ZIP;
  - macOS universal DMG;
  - macOS app tarball;
  - Linux AppImage;
  - Linux deb;
  - Linux rpm;
  - START/source ZIP;
  - LICENSE/COPYRIGHT/THIRD_PARTY_NOTICES;
  - SHA256SUMS.
- README UK / EN / FR / DE / ES / KO / JA синхронізовані.
- Нових user-visible UI strings у PR #61 не було; app i18n keys не потребували змін.

## GitHub Actions optimization

- PR #65 `ci: reduce redundant Actions runs` merged to `main` as `b94edb7a84896b0f8694e3d548398e1187d48c43`.
- Validation before merge: CI #317 **SUCCESS**; Mobile smoke #107 **SUCCESS** (Android + iOS).
- Docs-only pull requests no longer start the heavy desktop CI matrix.
- CI no longer repeats automatically on every push to `main`; PR validation is the primary code gate.
- Release-please PRs skip the heavy Rust/Tauri matrix and use a lightweight version-consistency check instead.
- Mobile smoke skips draft PRs and release-please PRs, while remaining required for ready/non-draft code PRs that touch app/core/mobile-relevant paths.
- Android/iOS smoke jobs now use Rust cache.
- Release builds now use npm cache.
- Docs-only and workflow-only pushes to `main` do not run release-please.
- Real release packaging is deliberately **not** configured with `cancel-in-progress`.
- PR #67 `ci: route checks by changed paths` merged to `main` as `68655c50fa43da74820ea38e723901f2ff6d15e7`.
- PR #67 validation: CI #318 **SUCCESS**; Mobile smoke #108 **SUCCESS** (Android + iOS).
- CI now classifies changed files: frontend-only changes avoid Rust/Tauri matrix; dependency licenses run only when relevant; unknown non-doc paths fall back to full validation.
- Draft PRs avoid heavy CI until `ready_for_review`.
- Mobile smoke paths are narrowed to native/core/dependency files, so pure frontend/i18n changes no longer build Android/iOS.

## Поточна наступна дія

**Git push — почати окремим slice з актуального `main` після перевірки GitHub state.**

Safety requirements для Git push мають зберігати існуючі trust boundaries:
- HTTPS only;
- credentials тільки через системне secret storage / in-memory callback;
- не записувати token у URL, Git config, vault або логи;
- не переписувати remote history мовчки;
- force push не вводити як implicit behavior;
- спершу planner/preflight + tests, потім mutation.

## Після Git push

1. Git state ↔ `SyncManifest` / ADR-0005 conflict policy.
2. WebDAV adapter.
3. Physical Android/iOS runtime validation.
4. Optional Android SAF / iOS security-scoped external folders.

## Відомі обмеження

- Android/iOS physical-device runtime untested;
- Git push not implemented;
- complete conflict workflow not implemented;
- WebDAV not implemented;
- SAF/bookmark external-folder adapters not implemented;
- large vault open/indexing can still be synchronous;
- graph still lacks ForceAtlas2/clustering/layout persistence;
- builds are unsigned unless explicitly stated otherwise.
