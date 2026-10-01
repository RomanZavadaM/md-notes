# MD Notes

[🇺🇦 Українська](../../README.md) · **🇬🇧 English** · [🇫🇷 Français](README.fr.md) · [🇩🇪 Deutsch](README.de.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · [🇯🇵 日本語](README.ja.md)

> **Current checkpoint: [MD Notes v0.1.0](https://github.com/RomanZavadaM/md-notes/releases/tag/v0.1.0)**
>
> **Development status: ACTIVE — stage v0.2 “structure and links”.**

## What it is

**MD Notes** is a cross-platform, local-first app for a personal knowledge base in Markdown: view, edit, structure and visualize plain `.md` files together with their attachments.

**Your data belongs to you.** Notes are open text files. You can open them in any editor, browse them on GitHub and version them with Git. The app never creates hidden formats.

Platforms: **Windows, macOS, Linux**; Android and iOS are planned for v0.3.

## v0.1.0 highlights

- a local folder as a vault; file tree with create, rename and trash;
- CodeMirror 6 editor, preview, side-by-side mode, autosave, atomic writes;
- `[[wiki links]]` that open the target note or create it when missing;
- `#tags` and YAML front matter properties;
- light, dark and system themes; layout for narrow screens;
- the sample knowledge base `sample-vault/`.

In progress (v0.2): index and full-text search, backlinks, link updates on rename, Mermaid and KaTeX, templates and daily notes, the interface in seven languages. Full plan: [roadmap](../roadmap.md) (Ukrainian).

## Install

Download the package for your OS from the [release page](https://github.com/RomanZavadaM/md-notes/releases):

- **Windows** — `MD.Notes_<version>_x64-setup.exe` or `.msi`. The build is not code-signed, so Windows may show SmartScreen.
- **macOS** — `.dmg` / `.app.tar.gz` (universal). The build is not notarized; you may need **System Settings → Privacy & Security → Open Anyway**.
- **Linux** — `.AppImage`, `.deb` or `.rpm`.

After launch, click **“Open folder”** and choose a folder with notes or `sample-vault/` from this repository.

Full guide: **[English User Guide](../user-guide/USER_GUIDE.en.md)**.

## For developers

Requires [Rust](https://rustup.rs/) (stable), [Node.js](https://nodejs.org/) 20+ and the [Tauri system prerequisites](https://v2.tauri.app/start/prerequisites/). Run `npm ci` and `npm run tauri dev` in `app/`. Development rules: [PROJECT_RULES.md](../../PROJECT_RULES.md) (Ukrainian, canonical).

## Privacy and legal

MD Notes is local-first: notes, attachments and the index stay on your device or in the storage you choose. There are no servers, analytics or telemetry. Sync conflicts are never overwritten silently. Keep backups of your vaults.

Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. MD Notes is proprietary software; the public repository does not grant an open-source license. Your notes belong to you. See [LICENSE.md](../../LICENSE.md) and [legal notices](../LEGAL_AND_COPYRIGHT.md).
