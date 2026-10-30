# WORKLOG — MD Notes

Оновлено: **30.10.2026**

> Operational log after the first stable desktop release. Older detail remains in Git history and Issue #8.

## Owner decision

30.10.2026: **«розробку першої повноцінної desktop-версії завершено; переходимо в режим використання й стабілізації»**.

## Stable v1.0.0 — RELEASED

- Release/tag: **v1.0.0**.
- Stable: yes; prerelease: no.
- Release target: `100fb61a7cd20385cd2f73042586527d20cb3dc4`.
- Release workflow: `41089712205` = **SUCCESS**.
- Assets: **13**.
- Post-release README link update: `5d34e85ff648f1adadf258b65b4c8fa0bdf83832`.
- Open PR at release checkpoint: none.
- 0.3.24 was never released.

## Platform build results

- Windows = SUCCESS.
- macOS universal = SUCCESS.
- Ubuntu/Linux = SUCCESS.
- assets = SUCCESS.
- README links = SUCCESS.

Published:
- Windows EXE / MSI / portable ZIP;
- macOS universal DMG / app tar.gz;
- Linux AppImage / DEB / RPM;
- START/source ZIP;
- SHA256SUMS;
- LICENSE / COPYRIGHT / THIRD_PARTY_NOTICES.

## Stabilization delivered before 1.0

### #192 — encrypted vault

- recovery public-recipient UI;
- in-memory encrypted image/PDF preview;
- no plaintext temp external open;
- CI + mobile smoke SUCCESS.

### #194 — regression/security gates

- seven-language parity gate;
- production npm high/critical audit gate;
- hard-coded UI strings localized;
- CI + mobile smoke SUCCESS.

### #196 — Windows packaging

- offline WebView2 installer in NSIS/MSI;
- bundled VC runtime;
- dedicated Windows installer smoke artifact;
- CI + mobile smoke SUCCESS.

### #197 — encrypted recovery drill

- real encrypted vault;
- note + attachment;
- backup;
- restore new vault;
- reopen with recovery private key;
- read restored note + attachment;
- CI + mobile smoke SUCCESS.

### #198 / #199 — stable release

- release-prep CI SUCCESS;
- versions/manifests aligned to 1.0.0;
- stable release PR validated and merged;
- full release workflow SUCCESS.

## Post-release cleanup

- one-time `release-as: 1.0.0` removed;
- README statuses changed from “preparing” to **stable released**;
- START/STATE/WORKLOG converted from release-prep state to stable baseline.

## Boundaries not to overclaim

- manual owner recovery drill on real data: not separately recorded;
- builds: unsigned / not notarized;
- Android/iOS physical runtime: not accepted;
- WebDAV / full SyncManifest conflict policy: not in desktop 1.0;
- encrypted cache: not implemented.

## Next mode

**Use and stabilize.**

Do not start a new feature automatically. Fix factual defects from real use first. Post-1.0 features only by explicit owner decision.
