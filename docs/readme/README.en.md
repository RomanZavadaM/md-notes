# MD Notes

[🇺🇦 Українська](../../README.md) · **🇬🇧 English** · [🇫🇷 Français](README.fr.md) · [🇩🇪 Deutsch](README.de.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · [🇯🇵 日本語](README.ja.md)

<!-- x-release-please-start-version -->
> **Release checkpoint: MD Notes v0.3.23**
>
> **Status: desktop feature-complete; stabilization is complete and stable v1.0.0 is being prepared.**

## Download MD Notes v0.3.23

**Quick install:**

- **Windows 64-bit:** [download .exe installer](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.23/MD.Notes_0.3.23_x64-setup.exe) · [MSI](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.23/MD.Notes_0.3.23_x64_en-US.msi) · [portable ZIP](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.23/MD-Notes-0.3.23-Windows-x64-portable.zip)
- **macOS Universal:** [download DMG](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.23/MD.Notes_0.3.23_universal.dmg)
- **Linux x86_64:** [AppImage](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.23/MD.Notes_0.3.23_amd64.AppImage) · [DEB](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.23/MD.Notes_0.3.23_amd64.deb) · [RPM](https://github.com/RomanZavadaM/md-notes/releases/download/v0.3.23/MD.Notes-0.3.23-1.x86_64.rpm)

[**All v0.3.23 release files →**](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.3.23)
<!-- x-release-please-end -->

> Builds are currently unsigned/not notarized, so your operating system may show a standard security warning.

> **Windows 1.0 packaging:** NSIS/MSI include the offline WebView2 installer and VC runtime; the lightweight portable ZIP uses the WebView2 Runtime already installed in Windows.

## What it is

**MD Notes** is a cross-platform, local-first Markdown knowledge-base app. It works with ordinary `.md/.markdown` files, attachments and open JSON metadata without a hidden proprietary data format.

**Your data belongs to you.** Notes can be opened with any editor, stored in your own filesystem and versioned with Git. MD Notes has no application server, analytics or telemetry.

Desktop 1.0 platforms: **Windows, macOS, Linux**. Android/iOS continue to build in CI, but physical-device runtime validation and mobile-store publication are post-1.0.

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
- live graph physics (a “floating” graph), graph in a separate window, relation tree graded by its weakest link, path between two notes;
- subject lookup by exact code (EDRPOU/tax number) or name, folder import with merging by code, possible duplicates;
- seven-language UI and light/dark/system themes.

### v0.3 — integrated through the v0.3.18 checkpoint

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
- optional encrypted vault (age): notes are stored on disk as `.age` files, the private key lives only in the device's system secret store (Windows Credential Manager, macOS Keychain, Linux Secret Service); dialogs cover creation with a one-time key backup, importing the key on another device, and an optional recovery key;
- Git push through the system `git` (Git must be installed) with a guard for encrypted vaults: plaintext is never sent; a Git panel in the app (status, fetch, pull, commit, push).

Still **not complete**: full Git-state integration with `SyncManifest` / ADR-0005 conflict policy, WebDAV, physical Android/iOS runtime validation, and optional Android SAF / iOS security-scoped external folders.

**Encrypted vault** is optional: ordinary vaults work unchanged, and a build without the `encrypted-vault` feature contains no encryption code. In the 1.0 stabilization, images and PDFs are previewed by decrypting only in memory, and an existing vault can add a public recovery recipient from the File menu. External opening of encrypted attachments is disabled to avoid a plaintext disk copy; the `.mdnotes/cache` index remains unencrypted, and Git push still passes through the plaintext guard.

Full plan: [roadmap](../roadmap.md) (Ukrainian, canonical).

> The FR / DE / ES / KO / JA translations of this README, the user guide and the UI strings are machine-generated and have not been reviewed by native speakers; Ukrainian is the authoritative text. Everything in this section is covered by automated tests and the GitHub build; manual checks in the running app and on physical Android/iOS devices are not yet confirmed by the owner.

## Install

Download packages from the [releases page](https://github.com/RomanZavadaM/md-notes/releases): Windows (`.exe`, `.msi`, portable ZIP), macOS (`.dmg`, `.app.tar.gz`) and Linux (`.AppImage`, `.deb`, `.rpm`).

The **v0.3.18** checkpoint includes standalone Markdown support and the safe Git fast-forward pipeline. Release builds remain unsigned/not notarized unless a specific release explicitly says otherwise.

Full guide: **[English User Guide](../user-guide/USER_GUIDE.en.md)**.

## Development, privacy and legal

Requires Rust stable, Node.js 20+ and Tauri system prerequisites. Integration gates include Rust fmt/clippy/tests, frontend build, dependency-license checks, desktop CI and Android/iOS mobile smoke for relevant changes.

MD Notes is local-first. Sync secrets must never be stored in the vault, Markdown files, remote URLs, Git config, logs or `localStorage`; they belong in the operating-system credential store. Sync conflicts must never be overwritten silently, and automatic Git apply is allowed only for a clean fast-forward state.

Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. MD Notes is proprietary software; the public repository grants no open-source license. Your notes belong to you. See [LICENSE.md](../../LICENSE.md) and [legal notices](../LEGAL_AND_COPYRIGHT.md).


## Multi-window editor (current `main`)

**v0.3.21** publishes ADR-0010 M1–M3. Multiple vault notes can stay open in tabs, each with its own draft. On desktop, the active note can be moved with **Open in separate window** into an independent editor window. Changes are saved before the move, and the main window returns to the previous tab or an empty workspace.

M1–M3 are part of published **v0.3.21**: conflict guards, safe detached-editor close, and save-all/safe Quit are implemented. Windows/macOS/Linux CI and Android/iOS build smoke pass. **M4 now means manual runtime acceptance**; the test prerelease is already published.


## Backup and restore

The **File** menu provides manual **Create backup…** and **Restore backup…** actions.

- backups are portable ZIP archives with `manifest.json` and a SHA-256 checksum for every file;
- MD Notes saves open notes before creating a backup;
- notes, attachments, configuration, templates and trash are included; `.git`, `.mdnotes/cache` and temporary OS noise are excluded;
- encrypted vaults are copied **without decryption**; private/recovery keys are never stored in the archive;
- restore validates paths, sizes and checksums first, then creates a **new** vault folder and never overwrites the current vault.

## Detailed principles and procedures

- [Knowledge links and graph](../concepts/KNOWLEDGE_GRAPH.en.md)
- [Encrypted vault](../security/ENCRYPTED_VAULT.en.md)
- [Backup and recovery](../security/BACKUP_AND_RECOVERY.en.md)
- [Canonical AI/automation contracts](../ai/README.md)

