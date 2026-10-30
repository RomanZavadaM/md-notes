# START_HERE — MD Notes

## ПІДСУМКОВИЙ ПРОТОКОЛ ЗАКРИТТЯ ВИПУСКУ — 30.10.2026

- Власник: завершити випуск, **не починати нові функції**, вести протоколи GitHub + Google Drive.
- Перевірений stable: **v1.0.2**, https://github.com/RomanZavadaM/md-notes/releases/tag/v1.0.2; GitHub Release workflow **41114443139 SUCCESS**, у релізі **13 assets**.
- Windows: portable ZIP, NSIS EXE, MSI; macOS: Universal DMG, app tar.gz; Linux: AppImage, DEB, RPM; додатково START ZIP, SHA256SUMS.txt, LICENSE.md, COPYRIGHT.md, THIRD_PARTY_NOTICES.md.
- Закрито проблему невідповідності Cargo.lock у release-please: PR #204 merged SHA ee5e0b8ecddc94da1d386bfdfc4b17ba4c4e0343; release PR #205 merged SHA 3e99b014b9bdbc41f1cf798200dd21e0054fbc1f; CI 41114353779 SUCCESS.
- Неповний v1.0.1 зберігається як історичний immutable tag (не переписувати). Новий рекомендований реліз — v1.0.2.
- Live GitHub перед заключним протоколом: main SHA 3849b5cf1e15c137eac78253669959b81f3477ae; відкритих PR 0.
- Автоматична збірка/публікація підтверджена. **Не заявляти** про ручне GUI-приймання на кожній ОС, реальне відновлення власного сховища чи окреме локальне скачування й звірку SHA256 кожного файла.
- Наступний режим: використання та стабілізація. Лише підтверджені дефекти, нові функції — за окремим рішенням. Усі наступні чати починають із START_HERE.md і звіряють live GitHub.
- Дзеркало протоколу: GitHub Issue #8, Google Drive документ MD_NOTES_CHECKPOINT_20261028_v0.3.21 (із новими секціями 30.10.2026).


## АКТУАЛЬНИЙ CHECKPOINT — stable v1.0.2 (30.10.2026)

- **Останній повний stable release: v1.0.2**, tag/release: https://github.com/RomanZavadaM/md-notes/releases/tag/v1.0.2.
- Release workflow 41114443139 — **SUCCESS**; 13/13 artifacts: Windows NSIS EXE, MSI, portable ZIP; macOS Universal DMG, app tar.gz; Linux AppImage, DEB, RPM; START ZIP, SHA256SUMS, LICENSE, COPYRIGHT, THIRD_PARTY_NOTICES.
- PR #204 та release PR #205 злиті, відкритих PR на контрольній точці немає.
- Попередній реліз v1.0.1 неповний через старий Cargo.lock; тег не змінювати. На v1.0.2 Cargo.lock синхронізовано, release metadata CI вимагає cargo metadata --locked.
- Контрольний main до оновлення документації: f1724c6d2e60b1452926d4897f200ea53e005685. Поточний main щоразу перевіряти заново.
- **Режим: тільки використання і стабілізація; жодних нових функцій без рішення власника.** Перевірка реального запуску на пристроях власника окремо не заявляється.
- Вести докладний протокол у WORKLOG.md, Issue #8 і Google Drive MD_NOTES_CHECKPOINT_20261028_v0.3.21. Старі секції нижче — історія.


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
