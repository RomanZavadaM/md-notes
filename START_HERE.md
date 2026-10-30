# START_HERE — MD Notes

> Перша точка входу для нового чату або відновлення після обриву.

## Поточний checkpoint — 30.10.2026

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

## Важливі межі

- ADR-0010 multi-window прийнято раніше у v0.3.22; не полірувати без нового фактичного дефекту.
- ADR-0009 backup/restore реалізовано з v0.3.22: portable ZIP + manifest/SHA-256, encrypted vault копіюється як ciphertext, restore лише в новий vault.
- Окремого ручного owner runtime-тесту backup/restore ще не було; є автоматизовані desktop tests.
- Physical Android/iOS runtime не підтверджений; WebDAV не реалізовано; signing/notarization не реалізовано.
- Тимчасовий PR #188 **не зливати**. Це TEST-build checkpoint; закривати його лише з архівним слідом відповідно до `dev-strategy`.

## Перед будь-якою новою роботою

1. Прочитати `PROJECT_RULES.md`, `PROJECT_STATE.md`, `WORKLOG.md`.
2. Звірити GitHub: `main` SHA, open PR, Actions, latest release, Issue #8.
3. GitHub має пріоритет над документацією й пам'яттю.
4. Не починати новий product slice без прямого слова власника.
5. Для наступної розробки дотримуватись рекомендацій `dev-strategy/outbox/MD_NOTES_RECOMMENDATIONS_20261030.md`: один PR на фічу, мінімум тестових гілок, архівний слід перед видаленням незлитих гілок.

## Наступні окремі кандидати

- batch move, потім batch rename — тільки окремими slices;
- frontend autotests / i18n parity у CI;
- WebView2 self-contained packaging;
- Git ↔ SyncManifest, WebDAV, physical mobile validation, encrypted-vault gaps, signing.

> **Продовжуємо MD Notes. Відкрий у GitHub `START_HERE.md` і продовжуй строго за ним.**
