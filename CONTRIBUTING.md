# Contributing to MD Notes

MD Notes is **proprietary software**, not an open-source project.

Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.

## External contributions

Unsolicited code contributions are not automatically accepted. Before submitting a material code, design, documentation, translation or branding contribution, obtain prior agreement from the repository owner regarding the contribution and the rights needed to incorporate it into the proprietary project.

A public pull request does not by itself transfer copyright ownership and does not change the MD Notes license.

Pull requests submitted without prior agreement may be reviewed for discussion or closed without merge.

## Technical requirements for an agreed contribution

- work against the current `main` in a separate branch with a pull request;
- follow `PROJECT_RULES.md` and `PROJECT_STATE.md`, including the local-first and privacy boundaries;
- write commit messages and PR titles in [Conventional Commits](https://www.conventionalcommits.org/) form (`feat:`, `fix:`, `docs:` …) — release-please builds the version and CHANGELOG from them;
- run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p notes-core`, and `npm ci && npm run build` in `app/`;
- add every new user-visible string to all seven language packs (UK, EN, FR, DE, ES, KO, JA); Ukrainian is the reference;
- preserve all copyright and third-party notices; update `THIRD_PARTY_NOTICES.md` for new direct dependencies;
- do not commit vaults, personal notes, credentials, index databases or private documents;
- record significant architectural decisions as ADRs in `docs/adr/`.

Code, comments and identifiers are written in English; project documentation is in Ukrainian with translations of README and the user guide.
