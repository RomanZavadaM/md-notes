# START_HERE — MD Notes

> Перша точка входу для нового чату або відновлення після обриву.

## Поточний checkpoint — 30.10.2026

> **Рішення власника:** розробку першої повноцінної desktop-версії вважаємо завершеною. Проєкт перейшов у **режим стабілізації до 1.0.0**. Нові продуктові функції не додаємо до 1.0; Android/iOS — post-1.0 і не блокують desktop release.

- Репозиторій: `RomanZavadaM/md-notes`.
- Останній опублікований реліз: **v0.3.23 prerelease**.
- Release target: `92e8478511e776ccf90867aacd38d8bcb656c478`.
- Release workflow: **41068168980 = SUCCESS**.
- Після автоматичного оновлення README фактичний `main` перед цим docs-checkpoint: `619e94a6787bfe9152377269ac4940806e9ce764`.
- У v0.3.23 опубліковано **13 assets**: Windows Portable + EXE/MSI, macOS universal DMG/app archive, Linux AppImage/deb/rpm, START/source, `SHA256SUMS.txt` і legal notices.
- Збірки не підписані й не нотаризовані.

## Що щойно завершено

- PR #186: **file-tree multi-select + batch trash r1**.
- Ctrl (Windows/Linux) / Cmd (macOS) додає або знімає елемент із вибору; Shift вибирає видимий діапазон.
- Пакетний кошик не дублює вкладені елементи, якщо вибрано батьківську папку; відкриті вкладки видалених нотаток узгоджуються зі станом диска; частковий failure також коректно reconciled.
- Власник перевірив Windows TEST r1 і дав практичне приймання: **«вроді все добре»**. Це достатньо для релізу, але не є exhaustive edge-case test.
- Batch move і batch rename **не входять** у v0.3.23.
- User guides синхронізовані UK / EN / FR / DE / ES / KO / JA.
- Release notes: `docs/releases/RELEASE_NOTES_v0_3_23.md`.
- PR #191 — окремий docs-only checkpoint: детальні принципи **графа/relations**, **зашифрованого vault** і **backup/recovery** додані UK / EN / FR / DE / ES / KO / JA.
- Для ШІ/автоматизації додано єдині канонічні контракти в `docs/ai/` (Markdown + YAML); людські переклади не створюють окремої нормативної семантики.

## Важливі межі

- ADR-0010 multi-window прийнято раніше у v0.3.22; не полірувати без нового фактичного дефекту.
- ADR-0009 backup/restore реалізовано з v0.3.22: portable ZIP + manifest/SHA-256, encrypted vault копіюється як ciphertext, restore лише в новий vault.
- Окремого ручного owner runtime-тесту backup/restore ще не було; є автоматизовані desktop tests.
- Physical Android/iOS runtime не підтверджений; WebDAV не реалізовано; signing/notarization не реалізовано.
- TEST PR #188 закрито без merge після створення archive trace branch `archive/test-file-tree-multiselect-r1-20172b7`.
- Активний 1.0 stabilization PR #192: encrypted-vault recovery UX + safe in-memory attachment preview.

## Перед будь-якою новою роботою

1. Прочитати `PROJECT_RULES.md`, `PROJECT_STATE.md`, `WORKLOG.md`.
2. Звірити GitHub: `main` SHA, open PR, Actions, latest release, Issue #8.
3. GitHub має пріоритет над документацією й пам'яттю.
4. Не починати новий product slice без прямого слова власника.
5. Для наступної розробки дотримуватись рекомендацій `dev-strategy/outbox/MD_NOTES_RECOMMENDATIONS_20261030.md`: один PR на фічу, мінімум тестових гілок, архівний слід перед видаленням незлитих гілок.

## Шлях до 1.0.0

1. Закрити security slice PR #192: recovery recipient UX + encrypted attachment/image/PDF preview без plaintext temp files.
2. Додати frontend regression tests / i18n parity gate.
3. Зробити Windows installer офлайн-встановлюваним щодо WebView2; portable лишити з чесною залежністю, якщо повністю self-contained portable недоцільний.
4. Провести/автоматизувати recovery drill backup → validate → restore, окремо для encrypted vault.
5. Підготувати release 1.0.0, документацію й stable release notes; signing/notarization виконати лише якщо доступні реальні сертифікати/credentials.

Post-1.0: batch move/rename, Git ↔ SyncManifest, WebDAV, physical Android/iOS validation, SAF/security-scoped folders та інші нові slices.

> **Продовжуємо MD Notes. Відкрий у GitHub `START_HERE.md` і продовжуй строго за ним.**
