# WORKLOG — MD Notes

Оновлено: **30.10.2026**

> Поточний operational log. Розгорнута історія попередніх ітерацій доступна в Git history та GitHub Issue #8; цей файл навмисно більше не дублює сотні історичних checkpoint.

## Поточний стан

- Репозиторій: `RomanZavadaM/md-notes`.
- Latest release: **v0.3.23 prerelease**.
- Release target: `92e8478511e776ccf90867aacd38d8bcb656c478`.
- Release workflow: `41068168980` = **SUCCESS**.
- Post-release README update main перед docs-checkpoint: `619e94a6787bfe9152377269ac4940806e9ce764`.
- Release assets: **13**.
- Відкритий PR: #188 — temporary Windows TEST PR; **не merge**.

## Завершено 30.10.2026

### File-tree multi-select + batch trash r1

- Product PR #186 злитий squash.
- Product merge SHA: `61dff93da651e6151ccf702fc99d25560fe3394f`.
- Виправлено partial batch-trash failure: UI reconciles успішно видалену частину, failed/unattempted entries лишаються вибраними.
- Feature CI на final head `95a4cdf5f74a8e65978da3daf86f3f439151b252` = SUCCESS.
- Windows TEST r1 зібрано окремо; archive/checksum integrity перевірено.
- Owner verdict: **«вроді все добре»** → practical runtime PASS, достатній для релізу; не exhaustive edge-case claim.

### v0.3.23 release

- Release Please PR #189 створив версію 0.3.23.
- Bot-created PR checks спочатку мали `action_required` без jobs; це не був code failure.
- Додано user-guide зміни всіма 7 мовами та `docs/releases/RELEASE_NOTES_v0_3_23.md`.
- Human-triggered final CI `41068081552` = SUCCESS.
- PR #189 злитий squash → release target `92e8478511e776ccf90867aacd38d8bcb656c478`.
- Release run `41068168980` = SUCCESS:
  - Windows = SUCCESS;
  - macOS universal = SUCCESS;
  - Linux = SUCCESS;
  - assets = SUCCESS;
  - README links = SUCCESS.
- Опубліковано 13 assets: Windows Portable/EXE/MSI, macOS DMG/app archive, Linux AppImage/deb/rpm, START/source, SHA256SUMS та legal notices.
- Після автоматичного README links update `main` = `619e94a6787bfe9152377269ac4940806e9ce764`.

### Graph / encryption / recovery documentation contracts

- Owner прямо погодив окремий docs slice.
- PR #191: `docs: add graph encryption and recovery guides`.
- Human guides: graph/relations, encrypted vault, backup/recovery — по 7 мов (UK / EN / FR / DE / ES / KO / JA), українська канонічна.
- AI/automation: одна канонічна серія контрактів у `docs/ai/` + YAML, без 7 незалежних нормативних копій.
- Зміст звірено з фактичним v0.3.23 code path: one-hop local graph, weakest-link pathGrade, random-name layout нових encrypted vault, plaintext cache exception, ciphertext backup і safe restore.
- Slice docs-only: runtime/version/release не змінюються; heavy CI не запускається за path-aware policy.

### Backup / checkpoint

- Product ADR-0009 backup/restore уже реалізований з v0.3.22; у v0.3.23 збережений без регресій.
- Manual owner runtime backup/restore test окремо не проводився; automated desktop tests — наявні.
- Створено незалежний Google Drive checkpoint:
  `MD_NOTES_CHECKPOINT_20261030_v0.3.23`
  https://docs.google.com/document/d/1SQpKJ-nCUP82cYzCC2RfMV-4w5ios2rTrjYe1rsleck
- У checkpoint записано release SHA/run/assets, runtime evidence, backup/restore boundary та backlog.

## Відкритий технічний хвіст

- PR #188 / branch `test/file-tree-multiselect-r1-windows` — test-only.
- Не зливати.
- За `dev-strategy` перед закриттям незлитої гілки потрібен archive tag/trace.
- Поточний GitHub connector не має create-tag action; тому не закривати PR лише заради чистого списку.

## Наступна точна дія

**Пауза після v0.3.23. Новий product slice починати лише після прямої вказівки власника.**

Кандидати окремими slices:
1. batch move;
2. batch rename;
3. frontend autotests / i18n parity;
4. WebView2 self-contained packaging;
5. Git ↔ SyncManifest / WebDAV / physical mobile / encrypted-vault gaps / signing.

Правила наступної сесії:
- спочатку `START_HERE.md`, `PROJECT_RULES.md`, `PROJECT_STATE.md`, цей файл та Issue #8;
- обов'язково звірити live GitHub;
- GitHub має пріоритет;
- один PR на фічу; rounds у тому самому PR;
- видані test revisions не перевикористовувати.
