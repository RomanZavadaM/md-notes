# START_HERE — MD Notes

> Перша точка входу для нового чату або відновлення після обриву.

## Поточний checkpoint — stable v1.0.0

> **Рішення власника від 30.10.2026:** перша повноцінна desktop-версія завершена. **Stable MD Notes v1.0.0 опубліковано; проєкт перейшов у режим використання й стабілізації.**

### Immutable release facts

- Репозиторій: `RomanZavadaM/md-notes`.
- Stable release: **v1.0.0**.
- Release type: **stable**, не prerelease.
- Release/tag target: `100fb61a7cd20385cd2f73042586527d20cb3dc4`.
- Release workflow: `41089712205` — **SUCCESS**.
- Published: 30.10.2026.
- Release assets: **13**.
- Post-release README-links commit: `5d34e85ff648f1adadf258b65b4c8fa0bdf83832`.
- Поточний `main` надалі звіряти live перед змінами — release target вище є immutable baseline.
- Open PR після release: немає на checkpoint.
- Проміжні 0.3.24 Release Please PR #193/#195 були закриті без merge; **0.3.24 не випускався**.

### Desktop 1.0 platforms

- Windows x64;
- macOS Universal;
- Linux x86_64.

Android/iOS проходять build smoke у CI, але physical-device runtime і store publication — post-1.0.

## Release assets v1.0.0

Windows:
- NSIS EXE;
- MSI;
- portable ZIP.

macOS:
- universal DMG;
- universal app tar.gz.

Linux:
- AppImage;
- DEB;
- RPM.

Додатково:
- `MD-Notes-1.0.0-START.zip`;
- `SHA256SUMS.txt`;
- `LICENSE.md`;
- `COPYRIGHT.md`;
- `THIRD_PARTY_NOTICES.md`.

Windows NSIS/MSI містять offline WebView2 installer + bundled VC runtime. Portable ZIP навмисно лишається легким і використовує system WebView2 Runtime.

## Stabilization gates перед 1.0 — COMPLETE

- PR #192 — encrypted vault stabilization.
  - CI `41079247425` = SUCCESS.
  - Mobile smoke `41079247290` = SUCCESS.
- PR #194 — 7-language parity + production npm security gate.
  - CI `41081985372` = SUCCESS.
  - Mobile smoke `41081985348` = SUCCESS.
- PR #196 — Windows offline installer packaging.
  - CI `41083660181` = SUCCESS.
  - Mobile smoke `41083660161` = SUCCESS.
  - Smoke artifact digest: `sha256:1b328fab1502a03f2471cb12c8b9e884cdfebd6f2438db6ecdab82300a0659a7`.
- PR #197 — real encrypted backup/recovery drill.
  - CI `41085366766` = SUCCESS.
  - Mobile smoke `41085366949` = SUCCESS.
- PR #198 — release-prep.
  - CI `41088207365` = SUCCESS.
- PR #199 — release 1.0.0.
  - release metadata consistency = SUCCESS.
  - release commit `100fb61a7cd20385cd2f73042586527d20cb3dc4`.

## Security / recovery boundary

- encrypted vault: age/X25519;
- existing encrypted vault can add public recovery recipient;
- encrypted images/PDF decrypt only in memory;
- external open encrypted attachments remains disabled to avoid plaintext temp files;
- `.mdnotes/cache` is outside the encrypted trust boundary;
- automated encrypted recovery drill passed: encrypted vault → note + attachment → backup → restore → recovery key → read restored note + attachment;
- separate manual owner backup/restore drill on real data is **not claimed**.

## Release-management cleanup

The one-time `release-as: 1.0.0` override is removed after the successful release. Future releases must follow normal semantic release flow; do not force 1.0.0 again.

## Post-1.0 backlog

Not blockers for v1.0.0:

- real-use bug fixes and stabilization;
- batch move / batch rename;
- Git ↔ SyncManifest conflict policy;
- WebDAV;
- physical Android/iOS validation;
- Android SAF / iOS security-scoped folders;
- stronger encrypted-cache mode if justified;
- signing/notarization and auto-update when real credentials/channel exist;
- future features only by explicit owner decision.

## Наступна точна дія

**Не починати новий product slice автоматично.**

Після старту нового чату:
1. звірити live GitHub: current main SHA, open PRs, latest release, recent workflows, Issue #8;
2. вважати stable v1.0.0 базовою desktop-версією;
3. працювати лише над фактичними дефектами/стабілізацією або окремо погодженим post-1.0 slice.

> **Продовжуємо MD Notes. Відкрий у GitHub `START_HERE.md` і продовжуй строго за ним.**
