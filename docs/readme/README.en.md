# MD Notes

[🇺🇦 Українська](../../README.md) · **🇬🇧 English** · [🇫🇷 Français](README.fr.md) · [🇩🇪 Deutsch](README.de.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · [🇯🇵 日本語](README.ja.md)

> **Latest published checkpoint: [MD Notes v0.2.2](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.2.2)**
>
> **Development status: ACTIVE — roadmap stage v0.3 “mobile platforms and synchronization”.**

## What it is

**MD Notes** is a cross-platform, local-first Markdown knowledge-base app. It works with ordinary `.md` files and attachments without a hidden proprietary data format.

**Your data belongs to you.** Notes can be opened with any editor, stored in your own filesystem and versioned with Git. MD Notes has no application server, analytics or telemetry.

Platforms: **Windows, macOS, Linux**. Android and iOS are under active v0.3 development; CI validates that mobile builds compile, but physical-device runtime is not yet claimed as validated.

## What is already implemented

### v0.2 — completed functional baseline

- local folder vault, file tree, create/rename/trash;
- CodeMirror 6, preview/split, autosave and atomic writes;
- SQLite/FTS5 index, search, tags and quick open;
- wiki links, aliases, backlinks and link rewriting on rename/move;
- Mermaid 11, KaTeX, images, attachments, templates and daily notes;
- schema-driven properties through open `.mdnotes/schema.json`;
- Empty / PARA / Zettelkasten presets;
- global/local knowledge graph;
- seven-language UI and light/dark/system themes.

### v0.3 — active development

Already integrated into `main`:

- provider-neutral `StorageProvider` / `VaultStorage`;
- mobile sandbox vault for Android/iOS;
- Android + iOS build smoke in CI;
- local-first sync decision foundation and persistent sync state;
- `gix`/gitoxide HTTPS Git foundation;
- dirty-worktree detection;
- local Git commit pipeline without user/system Git config;
- safe public HTTPS fetch;
- in-memory HTTPS authentication without persisting a token in Git config, the vault or the remote URL.

The current security checkpoint adds **system Git credential storage** in the Tauri layer: Windows Credential Manager, macOS Keychain, iOS Protected Data, Android Keystore-backed storage and Linux Secret Service. The frontend can save/check/clear credentials but cannot read the token back; authenticated fetch reads the secret only inside Rust immediately before the network call.

Still **not complete**: Git pull/merge policy, push, full integration with the conflict policy, WebDAV, physical Android/iOS runtime validation, and optional Android SAF / iOS security-scoped external folders.

Full plan: [roadmap](../roadmap.md) (Ukrainian, canonical).

## Install

Download packages from the [releases page](https://github.com/RomanZavadaM/md-notes/releases):

- **Windows** — `.exe`, `.msi` or portable ZIP;
- **macOS** — `.dmg` / `.app.tar.gz`;
- **Linux** — `.AppImage`, `.deb` or `.rpm`.

The latest published prerelease is **v0.2.2**. Builds are currently unsigned/not notarized, so the OS may show standard security warnings.

Full guide: **[English User Guide](../user-guide/USER_GUIDE.en.md)**.

## For developers

Requires Rust stable, Node.js 20+ and Tauri system prerequisites. Run `npm ci` and `npm run tauri dev` in `app/`.

Required integration gates include Rust format/clippy/tests, frontend build, dependency-license checks, desktop CI and Android/iOS mobile smoke when the mobile/Tauri boundary changes.

Canonical project state: [START_HERE.md](../../START_HERE.md), [PROJECT_RULES.md](../../PROJECT_RULES.md), [PROJECT_STATE.md](../../PROJECT_STATE.md), [WORKLOG.md](../../WORKLOG.md) and GitHub Issue #8.

## Privacy, credentials and legal

MD Notes is local-first. Notes, attachments and the rebuildable index stay on your device or in storage you choose. Sync secrets must never be persisted in the vault, Markdown files, remote URLs, Git config, logs or `localStorage`; they belong in the operating system credential store.

Sync conflicts must never be overwritten silently. Keep backups of important vaults.

Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. MD Notes is proprietary software; the public repository does not grant an open-source license. Your notes belong to you. See [LICENSE.md](../../LICENSE.md) and [legal notices](../LEGAL_AND_COPYRIGHT.md).
