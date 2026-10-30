# PROJECT_STATE — MD Notes

Оновлено: **30.10.2026**

## Інтегрований стан

- Статус: **ACTIVE / release checkpoint complete**.
- Останній реліз: **v0.3.23 prerelease**.
- Release target: `92e8478511e776ccf90867aacd38d8bcb656c478`.
- Release workflow: `41068168980` — **SUCCESS**.
- Усі release jobs успішні: release-please, Windows, macOS universal, Linux, assets, README links.
- Опубліковано **13 assets**.
- `main` після автоматичного README links update і перед фінальним docs-checkpoint: `619e94a6787bfe9152377269ac4940806e9ce764`.
- Ліцензія: proprietary / All Rights Reserved; Roman Zavada / Роман Завада.
- Основна мова документації: українська. User guides: UK / EN / FR / DE / ES / KO / JA.

## v0.3.23

PR #186 додав перший реалізований зріз групових операцій у дереві:

- Ctrl/Cmd toggle multi-selection;
- Shift visible-range selection;
- звичайний клік повертає одиночний вибір;
- batch trash для кількох файлів/папок;
- parent + child selection dedupe;
- безпечне закриття вкладок нотаток, які переміщено в кошик;
- reconciliation при частковій помилці batch trash.

Owner runtime verdict для Windows TEST r1: **practical PASS** — «вроді все добре». Не трактувати як exhaustive edge-case coverage.

**Не входять у v0.3.23:** batch move та batch rename. Їх робити окремими slices.

## Документація знань і безпеки

Docs-only PR #191 додає окремий завершений шар пояснень без зміни runtime або версії продукту:

- `docs/concepts/KNOWLEDGE_GRAPH.*.md` — правила wiki-links, typed `relations`, A1–F6, global/local graph, relation tree і weakest-link semantics;
- `docs/security/ENCRYPTED_VAULT.*.md` — age/X25519, random-name layout нових vault, OS secret store, recovery, plaintext cache boundary і threat model;
- `docs/security/BACKUP_AND_RECOVERY.*.md` — ZIP/manifest/SHA-256, exclusions, safe restore в новий vault та recovery drill;
- усі три людські набори є UK / EN / FR / DE / ES / KO / JA, українська — канонічна;
- `docs/ai/` містить одну канонічну нормативну пару Markdown + machine-readable YAML для graph і encrypted-vault, щоб AI-правила не розходилися між перекладами.

## Multi-window / ADR-0010

- Прийнято власником як достатньо робочий у v0.3.22 (r8).
- Main лишається центральним вікном.
- Graph може бути незалежним native-вікном.
- Graph navigation використовує звичайні вкладки main.
- Explicit detached note/editor windows лишаються окремою дією.
- Не продовжувати полірування без нового фактичного runtime-дефекту.

## Backup / restore / ADR-0009

Реалізовано й випущено з v0.3.22:

- manual portable ZIP backup;
- manifest + SHA-256 для вмісту;
- перед backup відкриті документи зберігаються;
- включаються нотатки, вкладення, конфігурація, шаблони та кошик;
- не включаються `.git`, `.mdnotes/cache` і тимчасовий системний шум;
- encrypted vault копіюється як ciphertext, без розшифрування й без secret/recovery keys;
- restore спочатку повністю валідовує архів і відновлює лише в новий vault, без silent overwrite.

Автоматизовані desktop tests пройдені. **Окремого owner manual runtime backup/restore test не було.**

## Релізні пакети v0.3.23

- Windows: Portable ZIP, NSIS EXE, MSI.
- macOS: universal DMG, universal app archive.
- Linux: AppImage, deb, rpm.
- Додатково: START/source ZIP, `SHA256SUMS.txt`, `LICENSE.md`, `COPYRIGHT.md`, `THIRD_PARTY_NOTICES.md`.
- Збірки unsigned / not notarized.

## Відкритий технічний хвіст

- PR #188 — тимчасовий Windows TEST PR для file-tree r1; **не merge**.
- За правилами `dev-strategy` незлитий тестовий branch перед закриттям має мати архівний слід. Доступний у цьому чаті GitHub connector не має create-tag action, тому PR не закривати лише заради чистоти списку.

## Backlog

Високий/ближчий:
1. Batch move — окремий slice.
2. Batch rename — окремий slice після move.
3. Frontend autotests / i18n parity у CI (MDN-04).
4. WebView2 self-contained packaging для Windows (MDN-08).
5. З'ясувати/зафіксувати причину bot-generated release-please `action_required` без jobs (MDN-07; у v0.3.23 human docs commit дав реальний зелений CI).

Інший:
- Git state ↔ `SyncManifest` / ADR-0005;
- WebDAV;
- physical-device Android/iOS runtime validation;
- Android SAF / iOS security-scoped external folders;
- encrypted-vault gaps (attachments/images, recovery-key UX, encrypted cache);
- signing/notarization;
- подальше розбиття великого `App.tsx`.

## Метод роботи

- GitHub — фактичне джерело істини.
- Один PR на фічу; acceptance rounds вести в тому самому PR, не множити гілки.
- Видану тестову ревізію rN не перевикористовувати.
- Закриті без merge гілки не видаляти без звірки з main та архівного сліду.
- `START_HERE.md` і цей файл тримати короткими; історія є у Git та Issue #8.
- Новий product slice починати лише після прямої вказівки власника.

Деталі релізу: `docs/releases/RELEASE_NOTES_v0_3_23.md`.
