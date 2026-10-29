# Development rules

- Before any project work, read `START_HERE.md`, `PROJECT_RULES.md`, `PROJECT_STATE.md` and `WORKLOG.md`; they are the canonical persistent rules and current checkpoint.

- Product scope: a local-first Markdown knowledge base. User notes are plain open files; never introduce hidden or locked data formats. Exception: an optional encrypted vault in the open, documented age format (ADR-0007), created only by explicit user action; for it never write decrypted note or attachment text to disk, keep private keys only in the OS secret store, never write own cryptography, keep plain vaults and builds without the `encrypted-vault` feature fully working, and add the `age` crate only after `cargo deny check licenses` passes.
- Notes, attachments, credentials and personal files must never traverse MD Notes servers; there are none. No analytics, telemetry, session replay, remote fonts or third-party tracking scripts.
- Credentials for network/cloud vaults live only in the OS secret store; never in vault files, localStorage or Git.
- Sync conflicts are never overwritten silently (ADR-0005). Deletion moves files to `.mdnotes/trash/`.
- The active product is the Tauri 2 app in `app/` with the Rust core `crates/notes-core`. Run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p notes-core`, and `npm ci && npm run build` in `app/` for code changes.
- Data logic belongs in `notes-core` with tests; the Tauri layer only adapts it for the UI.
- Discuss any change to these trust boundaries with the user before implementing it.

- Original MD Notes project materials are proprietary and owned by Roman Zavada (Роман Завада). Public repository visibility is not an open-source license.
- Preserve LICENSE.md, COPYRIGHT.md, THIRD_PARTY_NOTICES.md, visible copyright notices and platform metadata. Do not relicense the project or change the named copyright owner without the owner's explicit instruction.
- Third-party software and materials retain their own licenses, terms, attribution and rights; never claim them as original MD Notes property. User notes belong to the user.

- Product language rule: Ukrainian is canonical/default. Design user-visible strings for localization to English, French, German, Spanish, Korean and Japanese; every new string goes into all language packs in `app/src/i18n/`. Translations must preserve meaning, legal notices and data semantics. README and user guides are maintained in all seven languages.

## Методологія (спільна для проєктів власника)

Загальна стратегія розробки лежить у приватному репозиторії `RomanZavadaM/dev-strategy` (читати `STRATEGY.md`, шаблони в `templates/`, скрипти в `scripts/`). Для цього проєкту діє:

- Кожна гілка має PR (чернетка від першого пушу); без PR гілки не лишаємо. Закриття без злиття — лише з архівною міткою (`scripts/archive_branch.py`).
- Спільну теку проєкту не редагуємо: своя сесія = своє робоче дерево; `git stash` не використовуємо (WIP-коміт на своїй гілці).
- Початок і кінець сесії — за `templates/SESSION_START.md` і `SESSION_END.md`; перевірка `scripts/repo_health.py --local` має бути без червоного.
- `STATE` замінюється, а не доповнюється; ліміти: START ≤60, STATE ≤150 рядків.
- Огляд раз на 2 тижні: `docs/audit/AUDIT_<дата>.md` цього проєкту порівнюється з новим знімком; пункти закриваються з посиланням на коміт/PR.
- Питання про методологію → спершу `dev-strategy`; нова порада → PR туди.
