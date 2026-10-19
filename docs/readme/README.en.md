# MD Notes

[🇺🇦 Українська](../../README.md) · **🇬🇧 English** · [🇫🇷 Français](README.fr.md) · [🇩🇪 Deutsch](README.de.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · [🇯🇵 日本語](README.ja.md)

<!-- x-release-please-start-version -->
> **Release checkpoint: MD Notes v0.3.4**
>
> **Development status: ACTIVE — roadmap stage v0.3 “mobile platforms and synchronization”.**

## Download MD Notes v0.3.4

**Quick install:**

- **Windows 64-bit:** [download .exe installer](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.4/MD.Notes_0.3.4_x64-setup.exe) · [MSI](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.4/MD.Notes_0.3.4_x64_en-US.msi) · [portable ZIP](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.4/MD-Notes-0.3.4-Windows-x64-portable.zip)
- **macOS Universal:** [download DMG](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.4/MD.Notes_0.3.4_universal.dmg)
- **Linux x86_64:** [AppImage](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.4/MD.Notes_0.3.4_amd64.AppImage) · [DEB](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.4/MD.Notes_0.3.4_amd64.deb) · [RPM](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.4/MD.Notes-0.3.4-1.x86_64.rpm)

[**All v0.3.4 release files →**](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.3.4)
<!-- x-release-please-end -->

> Builds are currently unsigned/not notarized, so your operating system may show a standard security warning.

## What it is

**MD Notes** is a cross-platform, local-first Markdown knowledge-base app. It works with ordinary `.md/.markdown` files, attachments and open JSON metadata without a hidden proprietary data format.

**Your data belongs to you.** Notes can be opened with any editor, stored in your own filesystem and versioned with Git. MD Notes has no application server, analytics or telemetry.

Platforms: **Windows, macOS, Linux**. Android and iOS are active v0.3 targets; CI validates buildability, while physical-device runtime is not yet claimed as validated.

## Implemented

### v0.2 — completed functional baseline

- local-folder vault, file tree, create/rename/trash;
- CodeMirror 6, preview/split, autosave and atomic writes;
- SQLite/FTS5 index, search, tags and quick open;
- wiki links, aliases, backlinks and link rewriting on rename/move;
- Mermaid 11, KaTeX, images, attachments, templates and daily notes;
- schema-driven properties through open `.mdnotes/schema.json`;
- Empty / PARA / Zettelkasten presets;
- global/local knowledge graph;
- seven-language UI and light/dark/system themes.

### v0.3 — integrated through the v0.3.4 checkpoint

- provider-neutral `StorageProvider` / `VaultStorage`;
- mobile sandbox vault for Android/iOS;
- Android + iOS build smoke in CI;
- local-first sync decision foundation and persistent sync state;
- `gix`/gitoxide HTTPS Git foundation;
- dirty-worktree detection and local Git commit pipeline;
- safe public HTTPS fetch;
- in-memory HTTPS authentication without persisting tokens in Git config, the vault or remote URL;
- native system Git credential storage on Windows, macOS, iOS, Android and Linux;
- standalone `.md/.markdown` open/edit/save without a vault;
- Windows/Linux/macOS Markdown desktop integration, with default association only after explicit user action;
- Git pull planner for `UpToDate / FastForward / LocalAhead / Diverged / Unrelated`;
- pull safety preflight that blocks dirty worktrees;
- safe fast-forward apply only for a clean `FastForward`, without blind `hard reset`, merge commits or silent overwrite;
- optional encrypted vault (age): notes are stored on disk as `.age` files, the private key lives only in the device's system secret store (Windows Credential Manager, macOS Keychain, Linux Secret Service); dialogs cover creation with a one-time key backup, importing the key on another device, and an optional recovery key.

Still **not complete**: Git push, full Git-state integration with `SyncManifest` / ADR-0005 conflict policy, WebDAV, physical Android/iOS runtime validation, and optional Android SAF / iOS security-scoped external folders.

**Encrypted vault** is an option, not a requirement: ordinary vaults work unchanged, and a build without the `encrypted-vault` feature contains no encryption code. For now an encrypted vault does not display images and attachments, has no interface for adding a recovery key to an existing vault, and the `.mdnotes/cache` index stays unencrypted; Git push from such a vault will arrive with a guard that refuses to send plaintext.

Full plan: [roadmap](../roadmap.md) (Ukrainian, canonical).

## Install

Download packages from the [releases page](https://github.com/RomanZavadaM/md-notes/releases): Windows (`.exe`, `.msi`, portable ZIP), macOS (`.dmg`, `.app.tar.gz`) and Linux (`.AppImage`, `.deb`, `.rpm`).

The **v0.3.4** checkpoint includes standalone Markdown support and the safe Git fast-forward pipeline. Release builds remain unsigned/not notarized unless a specific release explicitly says otherwise.

Full guide: **[English User Guide](../user-guide/USER_GUIDE.en.md)**.

## Development, privacy and legal

Requires Rust stable, Node.js 20+ and Tauri system prerequisites. Integration gates include Rust fmt/clippy/tests, frontend build, dependency-license checks, desktop CI and Android/iOS mobile smoke for relevant changes.

MD Notes is local-first. Sync secrets must never be stored in the vault, Markdown files, remote URLs, Git config, logs or `localStorage`; they belong in the operating-system credential store. Sync conflicts must never be overwritten silently, and automatic Git apply is allowed only for a clean fast-forward state.

Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. MD Notes is proprietary software; the public repository grants no open-source license. Your notes belong to you. See [LICENSE.md](../../LICENSE.md) and [legal notices](../LEGAL_AND_COPYRIGHT.md).
