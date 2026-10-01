# Development rules

- Before any project work, read `START_HERE.md`, `PROJECT_RULES.md`, `PROJECT_STATE.md` and `WORKLOG.md`; they are the canonical persistent rules and current checkpoint.

- Product scope: a local-first Markdown knowledge base. User notes are plain open files; never introduce hidden or locked data formats.
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
