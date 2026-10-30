# PROJECT_STATE — MD Notes

## АКТУАЛЬНИЙ СТАН — stable desktop v1.0.2 (30.10.2026)

- **Latest stable release v1.0.2**; release workflow 41114443139 SUCCESS, 13 artifacts для Windows/macOS/Linux та юридичних/контрольних файлів.
- PR #204/205 merged, open PRs 0 на час перевірки. Release tag https://github.com/RomanZavadaM/md-notes/releases/tag/v1.0.2.
- v1.0.1 packaging failure через Cargo.lock виправлено; v1.0.1 tag не переміщували. Поточний main до docs update f1724c6d2e60b1452926d4897f200ea53e005685.
- Режим use-and-stabilize: без нових функцій, фіксувати лише реальні дефекти. Старі записи нижче збережено як історичний checkpoint.


Оновлено: **30.10.2026**

## Статус продукту

- **Stable desktop v1.0.0 RELEASED.**
- Режим роботи: **use-and-stabilize**.
- Release/tag target: `100fb61a7cd20385cd2f73042586527d20cb3dc4`.
- Release workflow `41089712205`: **SUCCESS**.
- Post-release README links commit: `5d34e85ff648f1adadf258b65b4c8fa0bdf83832`.
- Поточний main SHA надалі завжди звіряти live перед змінами.
- Release is stable: `prerelease=false`, `draft=false`.
- Assets: **13**.
- Ліцензія: proprietary / All Rights Reserved; Roman Zavada / Роман Завада.
- Основна мова: українська; user-facing interface/docs: UK / EN / FR / DE / ES / KO / JA.

## Desktop 1.0 baseline

Функціонально завершено:

- local-first Markdown vault;
- editor / preview / split / autosave / atomic writes;
- file tree, tabs, standalone Markdown, detached/multi-window views;
- search/FTS5, tags, aliases, backlinks, quick open;
- templates, daily notes, schema properties, PARA/Zettelkasten/Empty;
- attachments, Mermaid, KaTeX;
- global/local graph, typed relations A1–F6, reliability grades, relation tree, weakest-link path;
- Git foundation + encrypted-vault plaintext push guard;
- folder import + identity merge / duplicate candidates;
- encrypted vault age/X25519, random physical names, OS secret store;
- public recovery recipient management;
- encrypted image/PDF preview only in memory;
- backup ZIP + manifest/SHA-256 + safe restore into new vault;
- automated encrypted recovery drill;
- 7-language UI/docs;
- Windows/macOS/Linux CI + Android/iOS build smoke.

## v1.0.0 release evidence

Release workflow: `41089712205` = SUCCESS.

Platform jobs:
- Windows build = SUCCESS;
- macOS universal build = SUCCESS;
- Linux build = SUCCESS;
- assets = SUCCESS;
- README links = SUCCESS.

Assets:
- Windows NSIS EXE / MSI / portable ZIP;
- macOS universal DMG / app tar.gz;
- Linux AppImage / DEB / RPM;
- START/source ZIP;
- SHA256SUMS;
- LICENSE / COPYRIGHT / THIRD_PARTY_NOTICES.

Key published digests:
- Windows portable: `sha256:0d85c6c59de2a355e9a976a605ac0882b04e40179f5073de703bb977388786fc`;
- Windows EXE: `sha256:55fc7213a093790d95e51618e0b5eef94492e08f38ba3e83063a369893c7bcf2`;
- Windows MSI: `sha256:76fe580c6d39ca5b609a37587a4e9cb2493a9636340b1b7fc66f58622facc78b`;
- macOS DMG: `sha256:84d6ee38d9df72a09454b9186292743360c21b7eeb88b1a1b28aa6280e1b92c5`;
- AppImage: `sha256:e800a3ed9fcdfee97ade8f6fe1466eb99a49f74e7c37d2c4f8a806f075212768`;
- `SHA256SUMS.txt` asset digest: `sha256:1c0082447f2a8f1ff46c32ffe65a0791a34c12df0956fa64d51bc1bbdc008ccf`.

## 1.0 stabilization evidence

- #192 encrypted vault: CI `41079247425`, mobile `41079247290` — SUCCESS.
- #194 i18n/security gates: CI `41081985372`, mobile `41081985348` — SUCCESS.
- #196 offline Windows packaging: CI `41083660181`, mobile `41083660161` — SUCCESS.
- #197 encrypted recovery drill: CI `41085366766`, mobile `41085366949` — SUCCESS.
- #198 release prep: CI `41088207365` — SUCCESS.
- #199 release metadata consistency — SUCCESS.

## Backup / security boundary

- automated encrypted recovery drill = PASS;
- separate manual owner recovery drill on real data = not claimed;
- recovery/private keys are not included in backup;
- restore never silently overwrites an existing vault;
- `.mdnotes/cache` remains outside the encrypted trust boundary;
- encrypted external attachment open is intentionally disabled to avoid plaintext disk copies.

## Packaging boundary

- Windows NSIS/MSI: offline WebView2 installer + bundled VC runtime;
- Windows portable: lightweight; system WebView2 Runtime required;
- builds remain unsigned/not notarized until real signing credentials exist.

## Release management

- v1.0.0 was created with a one-time `release-as` override.
- After successful release that override is removed.
- Future releases use normal semantic release flow.
- 0.3.24 was never released.

## Post-1.0

Not blockers / future separate slices:

- real-use defects and stabilization;
- batch move / rename;
- Git ↔ SyncManifest / ADR-0005;
- WebDAV;
- physical Android/iOS acceptance;
- SAF/security-scoped folders;
- encrypted-cache option;
- signing/notarization + auto-update when credentials exist;
- future task/calendar/cloud/canvas/history ideas only after owner approval.

## Method

- GitHub facts beat docs if they differ.
- One PR per slice; no revision reuse after test issuance.
- No automatic product expansion after 1.0.
- START_HERE remains source of truth for next session.
