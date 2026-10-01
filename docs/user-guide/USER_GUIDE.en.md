# MD Notes — User Guide

[🇺🇦 Українська](USER_GUIDE.uk.md) · **🇬🇧 English** · [🇫🇷 Français](USER_GUIDE.fr.md) · [🇩🇪 Deutsch](USER_GUIDE.de.md) · [🇪🇸 Español](USER_GUIDE.es.md) · [🇰🇷 한국어](USER_GUIDE.ko.md) · [🇯🇵 日本語](USER_GUIDE.ja.md)

Applies to **v0.1.0**. The Ukrainian version is the reference text.

## 1. First launch

1. Install MD Notes for your OS (see the README).
2. Click **“Open folder”** and choose a folder with notes. Any folder with `.md` files works, including `sample-vault/` from the repository.
3. MD Notes remembers the folder and opens it on the next launch.

Opening a folder does not change it. The service folder `.mdnotes/` is created only when it is actually needed (trash, settings, index).

## 2. The window

- **Toolbar** — sidebar button ☰, vault name, title of the open note (a dot means unsaved changes), the “Editor” / “Side by side” / “Preview” modes and the theme selector.
- **Sidebar** — the vault file tree. Folders come first; hidden folders (`.mdnotes`, `.git`) are not shown. Files in other formats are greyed out.
- **Workspace** — the editor, the preview, or both side by side.
- **Status bar** — note path, tags, number of links, property errors and save state.

On narrow screens the sidebar opens over the content and hides after you pick a note.

## 3. Notes and folders

- **+ Note** creates a note in the selected folder (or next to the selected file). The note title becomes the file name; characters not allowed in file names are removed.
- A new note gets the properties `id` (a stable identifier), `type: note` and `created`.
- **+ Folder** creates a folder.
- **✎** renames the selected item. The `.md` extension is added automatically.
- **🗑** moves the item to the vault trash `.mdnotes/trash/`. Nothing is deleted permanently: you can move the file back by hand.

## 4. Editing

- Changes are saved automatically a moment after you stop typing, and with `Ctrl+S` / `Cmd+S`.
- Writes are atomic: a failure in the middle of saving never leaves a damaged file.
- **Side by side** shows the editor and the preview together. The preview supports GitHub Flavored Markdown: tables, task lists, strikethrough.

## 5. Links

- `[[Note title]]` links to another note. It is clickable in the preview.
- `[[Title|text]]` shows different text, `[[Title#Section]]` links to a section.
- If no note with that name exists, clicking the link creates it.
- Links are resolved by path, then by file name, ignoring case. Links inside code are ignored.
- Regular `https://…` links open in the system browser.

## 6. Tags and properties

- `#tag` in the text or a `tags` list in the properties marks a topic. Nested tags are supported: `#project/design`.
- Properties are written as YAML front matter at the top of the file between `---` lines.
- In the preview, properties are collapsed into a **“Properties”** block.
- If the YAML contains an error, the status bar shows it and the note opens as plain text.

## 7. Themes

Choose **System**, **Light** or **Dark** in the right corner of the toolbar. The choice is remembered.

## 8. Where data is stored

- Notes are ordinary files in the chosen folder. They remain usable without MD Notes.
- `.mdnotes/` is the vault service folder: settings, templates, trash, cache.
- MD Notes sends your data nowhere: no servers, analytics or telemetry.
- Back up the vault folder. Git is a convenient way to keep versions.

## 9. Keyboard

| Action | Windows / Linux | macOS |
|---|---|---|
| Save | `Ctrl+S` | `Cmd+S` |
| Undo / redo | `Ctrl+Z` / `Ctrl+Shift+Z` | `Cmd+Z` / `Cmd+Shift+Z` |
| Indent | `Tab` | `Tab` |

## 10. Troubleshooting

- **Windows shows SmartScreen** — the build is not code-signed. Choose “More info → Run anyway”.
- **macOS does not open the app** — the build is not notarized. Use **System Settings → Privacy & Security → Open Anyway**.
- **The vault does not open on launch** — the folder was renamed or moved. Open it again with **“Other vault…”**.
- **A note does not open** — MD Notes opens only `.md` and `.markdown` files.

## 11. Copyright

Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. MD Notes is proprietary software. Your notes belong to you. See [LICENSE.md](../../LICENSE.md) and [LEGAL_AND_COPYRIGHT.md](../LEGAL_AND_COPYRIGHT.md).
