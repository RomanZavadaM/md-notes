# WORKLOG — MD Notes

Оновлено: **30.10.2026**

> Поточний operational log. Розгорнута попередня історія лишається в Git history та Issue #8.

## Рішення власника

30.10.2026: **«розробку першої повноцінної desktop-версії завершено; переходимо в режим використання й стабілізації»**.

До stable 1.0.0 не додаємо нових desktop features. Mobile, sync expansion і batch move/rename — post-1.0.

## Live state перед release-prep

- Repository: `RomanZavadaM/md-notes`.
- Main: `423b8d2e3a92c953367c818eab64da999f9fdcff`.
- Latest published release: **v0.3.23 prerelease**.
- Generated 0.3.24 PR #193/#195: closed without merge; no 0.3.24 release exists.
- Release-prep target: **stable 1.0.0**.

## Stabilization completed

### PR #192 — encrypted vault

- Merge: `ea226a90b98ae4d3f7eb2ea9d1c35dc6ba25c53e`.
- Recovery public-recipient UI for existing encrypted vault.
- Encrypted image/PDF preview: plaintext only in memory.
- Encrypted attachment external-open disabled to avoid plaintext temp files.
- Human docs 7 languages + canonical AI contract v2.
- CI `41079247425`: SUCCESS.
- Mobile smoke `41079247290`: SUCCESS.

### PR #194 — regression/security gates

- Merge: `256b0d61d9b13f93d938ff7cb33045e0682876fa`.
- AST-based i18n parity: UK/EN/FR/DE/ES/KO/JA; 11 specialized maps at the checkpoint.
- Production npm audit high/critical gate in existing frontend job.
- Remaining hard-coded Git menu / detached-window error localized.
- CI `41081985372`: SUCCESS.
- Mobile smoke `41081985348`: SUCCESS.

### PR #196 — Windows offline installer

- Merge: `48e0eb0085fe57964ff3d8d98902e767cb3acfec`.
- Tauri Windows packaging: WebView2 `offlineInstaller` + `bundleVCRuntime`.
- CI path classifier gains packaging scope + Windows NSIS/MSI smoke job.
- Installer build passed.
- Artifact id: `12991027276`.
- Artifact ZIP size: 456803020 bytes.
- Digest: `sha256:1b328fab1502a03f2471cb12c8b9e884cdfebd6f2438db6ecdab82300a0659a7`.
- CI `41083660181`: SUCCESS.
- Mobile smoke `41083660161`: SUCCESS.

### PR #197 — encrypted recovery drill

- Merge: `423b8d2e3a92c953367c818eab64da999f9fdcff`.
- Automated real encrypted-vault recovery flow:
  encrypted vault → note + attachment → portable backup → restore new vault → recovery private key → read note + attachment.
- CI `41085366766`: SUCCESS.
- Mobile smoke `41085366949`: SUCCESS.

## Release-prep content

Prepared for stable 1.0:

- release-please: `prerelease=false`, explicit `release-as=1.0.0`;
- root + six translated README: desktop 1.0 status, mobile boundary, Windows offline-installer semantics;
- user guides all seven languages marked for v1.0.0;
- backup/recovery docs all seven languages include the automated recovery drill and preserve the manual-owner-test boundary;
- roadmap rewritten around desktop 1.0 / post-1.0;
- `docs/releases/RELEASE_NOTES_v1_0_0.md`;
- START/STATE/WORKLOG aligned.

## Remaining exact release sequence

1. Merge release-prep after checks.
2. Verify Release Please creates stable **1.0.0** PR (not 0.3.x).
3. Verify release metadata consistency, then merge the 1.0.0 release PR.
4. Wait for Release workflow: Windows/macOS/Linux, assets, README links.
5. Verify release tag, asset inventory, SHA256SUMS and final main SHA.
6. Update Issue #8 and Google Drive checkpoint with final immutable 1.0 facts.
7. Then switch project status to **stable / use-and-stabilize**.

## Non-blocking post-1.0 work

- batch move / rename;
- Git SyncManifest / WebDAV;
- physical mobile acceptance;
- encrypted cache;
- signing/notarization if credentials are supplied;
- future features only by owner decision.
