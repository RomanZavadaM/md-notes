# PROJECT_STATE — MD Notes

Оновлено: **30.10.2026**

## Інтегрований стан

- Статус: **desktop feature-complete / stabilization gates complete / preparing stable 1.0.0**.
- Поточний `main` перед release-prep: `423b8d2e3a92c953367c818eab64da999f9fdcff`.
- Останній опублікований реліз: **v0.3.23 prerelease**.
- 0.3.24 не випускався; generated PR #193/#195 закриті без merge.
- Release target next: **stable v1.0.0**.
- Desktop platforms: Windows / macOS / Linux.
- Ліцензія: proprietary / All Rights Reserved; Roman Zavada / Роман Завада.
- Основна мова документації: українська; user-facing guides: UK / EN / FR / DE / ES / KO / JA.

## Desktop 1.0 baseline

Функціонально завершено:

- local-first Markdown vault без proprietary content format;
- editor / preview / split / autosave / atomic writes;
- file tree, tabs, standalone Markdown, detached/multi-window views;
- search/FTS5, tags, aliases, backlinks, quick open;
- templates, daily notes, schema properties, PARA/Zettelkasten/Empty presets;
- attachments, Mermaid, KaTeX;
- global/local knowledge graph, typed relations A1–F6, reliability grades, relation tree, weakest-link path;
- Git foundation + encrypted-vault plaintext push guard;
- folder import + identity merge / duplicate candidates;
- encrypted vault age/X25519, random physical names, OS secret store;
- public recovery recipient management for existing encrypted vault;
- encrypted image/PDF preview only in memory;
- backup ZIP + manifest/SHA-256 + safe restore into new vault;
- encrypted recovery drill;
- 7-language UI/documentation;
- Windows/macOS/Linux CI + Android/iOS build smoke.

## 1.0 stabilization evidence

### #192 encrypted vault

Merge: `ea226a90b98ae4d3f7eb2ea9d1c35dc6ba25c53e`.

- memory-only image/PDF preview;
- no plaintext temp-file external open;
- recovery recipient UI;
- AI security contract updated to encrypted-vault/2;
- human security/user docs aligned in 7 languages.
- CI `41079247425` = SUCCESS.
- Mobile smoke `41079247290` = SUCCESS.

### #194 regression/security gates

Merge: `256b0d61d9b13f93d938ff7cb33045e0682876fa`.

- automated parity for all 7 languages across specialized maps;
- high/critical production npm advisory gate;
- remaining hard-coded Git menu / detached error strings localized.
- CI `41081985372` = SUCCESS.
- Mobile smoke `41081985348` = SUCCESS.

At checkpoint, production audit had only low-severity KaTeX-chain advisories; no high/critical production blocker passed the gate.

### #196 Windows packaging

Merge: `48e0eb0085fe57964ff3d8d98902e767cb3acfec`.

- NSIS/MSI embed WebView2 offline installer;
- Windows installer bundles VC runtime;
- CI adds a path-aware installer smoke build;
- smoke artifact id `12991027276`, size 456803020 bytes;
- artifact digest `sha256:1b328fab1502a03f2471cb12c8b9e884cdfebd6f2438db6ecdab82300a0659a7`.
- CI `41083660181` = SUCCESS.
- Mobile smoke `41083660161` = SUCCESS.

Portable ZIP remains intentionally lightweight and requires an existing system WebView2 Runtime.

### #197 encrypted backup/recovery

Merge / current main: `423b8d2e3a92c953367c818eab64da999f9fdcff`.

Automated end-to-end test proves:

- create real encrypted vault with recovery recipient;
- write note + binary attachment;
- backup;
- restore into new folder;
- reopen using recovery private key;
- read restored note + attachment bytes.

CI `41085366766` = SUCCESS.
Mobile smoke `41085366949` = SUCCESS.

## Backup / recovery boundary

- automated encrypted recovery drill = PASS;
- separate manual owner backup/restore drill on real data = **not claimed**;
- encrypted backup contains ciphertext and no secret/recovery private keys;
- restore validates before writing and never silently overwrites an existing vault.

## 1.0 release packaging

Expected stable release assets after release workflow:

- Windows: NSIS EXE, MSI, portable ZIP;
- macOS: universal DMG + app archive;
- Linux: AppImage, deb, rpm;
- START/source ZIP;
- SHA256SUMS;
- LICENSE / COPYRIGHT / THIRD_PARTY_NOTICES.

Builds remain unsigned/not notarized until real credentials exist.

## Post-1.0 backlog

Not blockers for desktop 1.0:

- batch move / batch rename;
- Git ↔ SyncManifest / ADR-0005;
- WebDAV;
- physical Android/iOS runtime acceptance;
- Android SAF / iOS security-scoped folders;
- encrypted cache option;
- signing/notarization + auto-update when credentials/channel exist;
- larger App.tsx decomposition when future work requires it;
- future task/calendar/cloud/canvas/history ideas only after explicit owner approval.

## Метод роботи

- GitHub is the factual source of truth.
- One PR per slice; acceptance rounds stay in the same PR.
- Issued test revision numbers are never reused.
- Closed-without-merge test work keeps an archive trace.
- START_HERE / PROJECT_STATE / WORKLOG stay compact.
- No new product scope before 1.0 release completes.

Release notes candidate: `docs/releases/RELEASE_NOTES_v1_0_0.md`.
