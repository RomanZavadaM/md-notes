# START_HERE — MD Notes

> Перша точка входу для нового чату або відновлення після обриву.

## Поточний checkpoint — 30.10.2026

> **Рішення власника:** перша повноцінна desktop-версія функціонально завершена. Stabilization gates до **stable 1.0.0** пройдені; зараз виконується release-prep.

- Репозиторій: `RomanZavadaM/md-notes`.
- Поточний інтегрований `main` перед release-prep: `423b8d2e3a92c953367c818eab64da999f9fdcff`.
- Останній опублікований реліз: **v0.3.23 prerelease**.
- Проміжні Release Please PR на 0.3.24 (#193, #195) закриті без merge; 0.3.24 не випускався.
- Ціль release-prep: **stable v1.0.0**, не prerelease.
- Платформи desktop 1.0: Windows, macOS, Linux.
- Android/iOS: build-smoke проходить, physical-device runtime — post-1.0.
- Збірки поки unsigned / not notarized.

## Stabilization gates до 1.0 — COMPLETE

### PR #192 — encrypted vault stabilization

- Merge: `ea226a90b98ae4d3f7eb2ea9d1c35dc6ba25c53e`.
- Existing encrypted vault: localized UI для додавання **public recovery recipient**.
- Images/PDF: decrypt only in memory через active storage provider + Blob URL.
- External open encrypted attachments вимкнено, щоб не створювати plaintext temp file.
- CI `41079247425` = SUCCESS.
- Mobile smoke `41079247290` = SUCCESS.

### PR #194 — regression/security gates

- Merge: `256b0d61d9b13f93d938ff7cb33045e0682876fa`.
- 7-language parity gate: UK / EN / FR / DE / ES / KO / JA.
- Production `npm audit --omit=dev --audit-level=high` у frontend CI.
- CI `41081985372` = SUCCESS.
- Mobile smoke `41081985348` = SUCCESS.

### PR #196 — Windows offline installer

- Merge: `48e0eb0085fe57964ff3d8d98902e767cb3acfec`.
- NSIS/MSI: WebView2 `offlineInstaller` + bundled VC runtime.
- Portable ZIP лишається легким і використовує system WebView2 Runtime.
- Windows installer smoke реально зібрав NSIS+MSI.
- Artifact id: `12991027276`.
- Artifact digest: `sha256:1b328fab1502a03f2471cb12c8b9e884cdfebd6f2438db6ecdab82300a0659a7`.
- CI `41083660181` = SUCCESS.
- Mobile smoke `41083660161` = SUCCESS.

### PR #197 — encrypted recovery drill

- Merge / current main: `423b8d2e3a92c953367c818eab64da999f9fdcff`.
- Automated scenario: encrypted vault → note + attachment → backup → restore → recovery key → read note + attachment.
- CI `41085366766` = SUCCESS.
- Mobile smoke `41085366949` = SUCCESS.

## Документація 1.0

Release-prep синхронізує:

- root README + 6 translated README;
- user guides UK / EN / FR / DE / ES / KO / JA;
- backup/recovery procedures 7 мовами;
- `docs/roadmap.md`;
- `docs/releases/RELEASE_NOTES_v1_0_0.md`;
- `PROJECT_STATE.md`, `WORKLOG.md`;
- Release Please config: stable `1.0.0`.

Детальні граф/encryption/backup human guides і канонічні AI contracts уже інтегровані раніше.

## Важливі межі 1.0

- `.mdnotes/cache` не входить у encrypted trust boundary.
- Manual owner backup/restore drill на реальних даних окремо не заявляється; automated encrypted recovery drill пройдено.
- Physical Android/iOS runtime не підтверджений.
- WebDAV і Git↔SyncManifest conflict policy — post-1.0.
- Batch move / batch rename — post-1.0.
- Signing/notarization — лише коли є реальні credentials.

## Наступна точна дія

1. Merge release-prep PR після перевірок.
2. Переконатися, що Release Please створив **stable 1.0.0** PR.
3. Перевірити release metadata consistency і merge 1.0.0 release PR.
4. Дочекатися release workflow на Windows/macOS/Linux + assets + README links.
5. Перевірити tag/release/assets/SHA256SUMS і фінальний `main`.
6. Оновити Issue #8 та незалежний Google Drive checkpoint уже фактичними даними 1.0.0.

Після 1.0: лише реальні дефекти/стабілізація або окремо погоджені product slices.

> **Продовжуємо MD Notes. Відкрий у GitHub `START_HERE.md` і продовжуй строго за ним.**
