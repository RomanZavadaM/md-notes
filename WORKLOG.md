# WORKLOG — MD Notes

Оновлено: **30.10.2026**

> Operational log after the first stable desktop release. Older detail remains in Git history and Issue #8.

## Owner decision

30.10.2026: **«розробку першої повноцінної desktop-версії завершено; переходимо в режим використання й стабілізації»**.

## Active stabilization slice — full stable 1.0 audit cleanup

- Owner request: full post-release verification of code, environment, packaging, localization, docs and descriptions.
- Base main: `43fcf3720457b239458dcd34418328893d921dc7`.
- Audit-only PR: #201 — **DO NOT MERGE**; forces current full CI/mobile/package checks and audit assertions.
- Fix branch: `fix/v1.0.0-stable-audit-cleanup`.
- Confirmed issues corrected in this slice:
  - stale v0.3 user-facing mobile Git/WebDAV promise in all 7 languages;
  - Ukrainian-only language descriptions leaking into non-Ukrainian UI;
  - graph/encrypted-vault/backup current docs and AI contracts still scoped to v0.3.23;
  - current guides/readmes retaining obsolete multi-window acceptance wording;
  - ADR/spec/sample docs presenting old v0.3 plans without a stable-1.0 status boundary;
  - stale `app/package-lock.json` root version 0.2.1; release metadata now tracks both lockfile version fields;
  - Rust dependency advisories: ZIP/TLS/webpki/root-cert/ctutils graph refreshed, ZIP moved to 9.x, deprecated YAML backend replaced; weekly RustSec/npm audit added;
  - narrow documented residual RUSTSEC-2026-0387 exception for the unused P-256/HPKE path inside latest `age 0.12.1`; MD Notes uses X25519;
  - missing Tauri `opener:allow-open-path` for plain attachment external-open, now scoped to the main window only;
  - plain image/PDF preview depended on `convertFileSrc` while asset protocol scope was empty; all vault assets now read through the Rust storage provider into Blob URLs and filesystem asset protocol is disabled;
  - release packaging now verifies locked Rust metadata and npm package/lock version consistency.
- Audit facts already PASS: 1.0.0 metadata alignment at the audited checkpoint, required legal/release/docs files, 440 live local Markdown links in the baseline audit, all 13 published v1.0.0 asset URLs, privacy/no-telemetry source scan, and existing encrypted backup/recovery regression drill.
- Separate test-only final packaging branch: `audit/v1.0.0-final-packaging`; **DO NOT MERGE**.
- Done only after the final PR #202 head passes full CI + Android/iOS smoke, the separate macOS/Linux package smoke passes, and the integrated state is recorded.

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
