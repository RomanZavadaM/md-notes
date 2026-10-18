# MD Notes v0.3.1 — release notes

Дата checkpoint-а: **18.10.2026**

## Що увійшло

### Standalone Markdown
- одиночні `.md/.markdown` файли відкриваються без vault;
- файл можна читати, редагувати та зберігати назад;
- Windows/Linux/macOS підтримують desktop integration;
- macOS підтримує відкриття Markdown із Finder;
- системна асоціація Markdown виконується тільки після явної команди користувача;
- portable та installer не перехоплюють `.md` автоматично.

### Git pull safety
- read-only planner класифікує `UpToDate / FastForward / LocalAhead / Diverged / Unrelated`;
- preflight додає перевірку чистоти worktree;
- автоматичне застосування дозволене лише для clean `FastForward`;
- safe fast-forward apply повторно перевіряє стан безпосередньо перед mutation;
- detached/unborn/non-fast-forward стани не застосовуються автоматично;
- немає shell `git`, merge commit або blind `hard reset`;
- worktree/index оновлюються через gix;
- branch ref оновлюється з optimistic old-HEAD constraint;
- при гонці оновлення ref виконується спроба відновити попередній tree.

## Перевірки перед release checkpoint

Фінальний функціональний head PR #61:
`8bac0f284dfbe2fab21ad8077b3b9d40868f6403`

- CI #303 — **SUCCESS**;
- Mobile smoke #99 — **SUCCESS**;
- Android debug build — **SUCCESS**;
- iOS simulator debug build — **SUCCESS**;
- PR #61 merge commit — `ed924059fa68bc31014c5b484fb78daaeba675b0`.

## Межа доказу

- Android/iOS mobile evidence у цьому checkpoint — CI build smoke, не physical-device runtime;
- релізні builds залишаються unsigned, якщо workflow явно не підтвердить signing;
- Git push ще не реалізований;
- Git ↔ `SyncManifest` / ADR-0005 conflict integration ще не завершена;
- WebDAV ще не реалізований.

## Наступний напрям

1. Git push.
2. Git state ↔ `SyncManifest` / ADR-0005 conflict policy.
3. WebDAV.
4. Physical Android/iOS runtime validation.
5. Optional Android SAF / iOS security-scoped external folders.
